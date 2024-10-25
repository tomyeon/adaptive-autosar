use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::{EXECUTION_MANIFEST_FILE, MACHINE_MANIFEST_FILE, OARA_CONFIG_EXEC};
use anyhow::Result;
use ara_exec::manifest::execution_manifest::ExecutionManifest;
use ara_exec::manifest::machine_manifest::MachineManifest;
use thiserror::Error;

/// Load manifest files
/// Structures
/// /usr/bin/oara  (RO_OARA_ROOT)
///           |- EM
///           |- SM
///           ...
///           \- Others
/// /etc/oara  (OARA_CONFIG)
///           |- machine_manifest.yaml
///           |- exec
///               |- em_execution_manifest.yaml
///               |- sm_execution_manifest.yaml
///               ...
///               \- others_execution_manifest.yaml
/// /opt/oara (RW_OARA_ROOT, optional)
///           |- App1
///           |   |- bin - App1
///           |   \- manifest - execution_manifest.yaml
///           |- others
///           ...

#[derive(Debug, Error)]
enum ExecutionManifestError {
    #[error("Duplicated application name : {0}")]
    DuplicatedAppName(String),
    #[error("Missing dependency application : {0} for {1}")]
    MissingDependencyApp(String, String),
    #[error("Self dependency is not allowed : {0}")]
    SelfDependency(String),
    #[error("Dependency app({0}) is not in the mode")]
    InvalidModeDependency(String),
    // circular dependency
}

pub fn load_machine_manifest<P: AsRef<Path>>(path: P) -> Result<MachineManifest> {
    let mut machine_manifest_path = PathBuf::from(path.as_ref());
    machine_manifest_path.push(super::MACHINE_MANIFEST_FILE);
    MachineManifest::from_file(machine_manifest_path)
}

/// Load execution manifest files
pub fn load_execution_manifest<P1, P2>(
    oara_config_path: P1,
    rw_oara_path: P2,
) -> Result<Vec<ExecutionManifest>>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let mut exec_path = PathBuf::from(oara_config_path.as_ref());
    exec_path.push(super::OARA_CONFIG_EXEC);

    let mut execution_manifest_files = Vec::new();
    for entry in std::fs::read_dir(exec_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|ext| ext == "yaml").unwrap_or(false) {
            execution_manifest_files.push(path);
        }
    }

    if !rw_oara_path.as_ref().to_string_lossy().is_empty() {
        for entry in std::fs::read_dir(rw_oara_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                execution_manifest_files
                    .push(path.join("mainfest").join(super::EXECUTION_MANIFEST_FILE));
            }
        }
    }

    let mut execution_manifests = Vec::new();
    for path in execution_manifest_files {
        execution_manifests.push(ExecutionManifest::from_file(path)?);
    }

    Ok(execution_manifests)
}

pub fn validate_manifest(
    machine: MachineManifest,
    executions: Vec<ExecutionManifest>,
) -> Result<()> {
    let mut app_hashmap = HashMap::new();

    // validate app and mode dependency
    for execution in &executions {
        let _ = execution.validate(&machine)?;
        if app_hashmap.contains_key(execution.name.as_str()) {
            return Err(ExecutionManifestError::DuplicatedAppName(execution.name.clone()).into());
        } else {
            app_hashmap.insert(execution.name.as_str(), execution);
        }
    }

    // dependency check
    // dependency application should be in the same function group's state
    // dependency application should be configured
    for execution in &executions {
        for app_dependency in &execution.app_dependency {
            let (app, _) = app_dependency.split_once(".").unwrap();
            if !app_hashmap.contains_key(app) {
                return Err(ExecutionManifestError::MissingDependencyApp(
                    app.to_owned(),
                    execution.name.clone(),
                )
                .into());
            }

            // Not possible to have self dependency
            // It's not possible yet
            // if app == execution.name {
            //     return Err(ExecutionManifestError::SelfDependency(app.to_owned()).into());
            // }

            // dependency app should in the same function group's state

            let mut mode_dependency_valid = false;
            for mode_dependency in &app_hashmap.get(app).unwrap().mode_dependency {
                if execution.mode_dependency.contains(mode_dependency) {
                    mode_dependency_valid = true;
                    break;
                }
            }

            if !mode_dependency_valid {
                return Err(ExecutionManifestError::InvalidModeDependency(app.to_owned()).into());
            }
        }
    }

    // TBD, circular dependency

    Ok(())
}

mod tests {
    use ara_exec::manifest::execution_manifest;

    use super::*;
    use std::{
        env,
        fs::{self, File},
        io::Write,
    };

    fn valid_machine_manifest() -> &'static str {
        let manifest: &'static str = r#"
            function_group_set:
              MachineFG:
                initial_mode: "Startup"
                mode:
                - "Startup"
                - "Shutdown"
                - "Restart"
        "#;

        manifest
    }

    fn valid_execution_manifest() -> &'static str {
        let manifest: &'static str = r#"
            name: SM
            environmental_variable:
              ENV1: "environment variable smaple1"
              ENV2: "environment variable smaple2"
            argument:
              ARG1: "argument variable1"
              ARG2: "argument variable2"
            enter_exit_timeout:
              enter: 1
              exit: 1
            reporting_behavior: true # true or false
            number_of_restart: 0
            mode_dependency:
              - MachineFG.Startup
        "#;
        manifest
    }

    fn make_oara_folder<P: AsRef<Path>>(dir_path: P) -> PathBuf {
        let exe_path = env::current_exe().unwrap();
        let oara_config = exe_path.parent().unwrap().join(dir_path).join("oara");

        // remove directory if exists
        if oara_config.exists() {
            fs::remove_dir_all(&oara_config).unwrap();
        }

        fs::create_dir_all(&oara_config).unwrap();

        oara_config
    }

    fn clean_oara_folder<P: AsRef<Path>>(dir_name: P) {
        let exe_path = env::current_exe().unwrap();
        let oara_config = exe_path.parent().unwrap().join(dir_name);

        if oara_config.exists() {
            fs::remove_dir_all(&oara_config).unwrap();
        }
    }

    fn oara_folder_exist<P: AsRef<Path>>(dir_name: P) -> bool {
        let cur = env::current_exe().unwrap();
        let oara_config = cur.join(dir_name);

        oara_config.exists()
    }

    fn configure_machine_manifest(dir_name: &str, contents: &str) -> PathBuf {
        let mut oara_config = make_oara_folder(dir_name);
        oara_config.push(MACHINE_MANIFEST_FILE);

        let mut file = File::create(&oara_config).unwrap();
        file.write_all(contents.as_bytes()).unwrap();

        let _ = oara_config.pop();
        oara_config
    }

    fn make_oara_exec_folder<P: AsRef<Path>>(dir_path: P) -> PathBuf {
        let oara_path = make_oara_folder(dir_path);
        let oara_exec_path = oara_path.join(OARA_CONFIG_EXEC);

        fs::create_dir_all(&oara_exec_path).unwrap();
        oara_exec_path
    }

    fn add_oara_exec_folder<P: AsRef<Path>>(oara_path: P) -> PathBuf {
        let mut oara_path = if !oara_path.as_ref().exists() {
            make_oara_folder(oara_path)
        } else {
            oara_path.as_ref().to_path_buf()
        };

        oara_path.push(OARA_CONFIG_EXEC);

        fs::create_dir_all(&oara_path).unwrap();
        oara_path
    }

    fn configure_execution_manifest<P>(oara_exec_path: P, app_name: &str, contents: &str) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let manifest_name = format!("{}_{}", app_name, EXECUTION_MANIFEST_FILE);
        let oara_exec_path: PathBuf = oara_exec_path.as_ref().to_path_buf();
        let oara_manifest_path = oara_exec_path.join(manifest_name);

        let mut file = File::create(&oara_manifest_path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();

        oara_manifest_path
    }

    #[test]
    fn valid_load_machine_manifest() {
        let contents = valid_machine_manifest();

        let oara_config = configure_machine_manifest("configuration-t1", contents);
        let manifest = load_machine_manifest(&oara_config);
        assert!(manifest.is_ok());

        clean_oara_folder("t1");
    }

    #[test]
    fn invalid_load_machine_manifest() {
        let oara_config_path = make_oara_folder("configuration-t2");
        let manifest = load_machine_manifest(&oara_config_path);
        assert!(manifest.is_err());

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    fn valid_load_execution_manifest() {
        // load machine manifest
        let contents = valid_machine_manifest();
        let oara_config_path = configure_machine_manifest("configuration-t3", contents);
        let machine_manifest = load_machine_manifest(&oara_config_path).unwrap();

        // load execution manifest
        let contents = valid_execution_manifest();
        let oara_exec_path = add_oara_exec_folder(&oara_config_path);
        let _oara_exec_file_path = configure_execution_manifest(oara_exec_path, "t3", contents);
        let execution_manifest = load_execution_manifest(&oara_config_path, "");
        assert!(execution_manifest.is_ok());
        assert!(validate_manifest(machine_manifest, execution_manifest.unwrap()).is_ok());

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    fn invalid_load_execution_manifest() {
        // load machine manifest
        let contents = valid_machine_manifest();
        let oara_config_path = configure_machine_manifest("configuration-t4", contents);
        let _machine_manifest = load_machine_manifest(&oara_config_path).unwrap();

        // load execution manifest
        let execution_manifest = load_execution_manifest(&oara_config_path, "");
        assert!(execution_manifest.is_err()); // No oara/exec folder

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    fn execution_manifest_duplicated_app_name() {
        // load machine manifest
        let contents = valid_machine_manifest();
        let oara_config_path = configure_machine_manifest("configuration-t5", contents);
        let machine_manifest = load_machine_manifest(&oara_config_path).unwrap();

        // load execution manifest
        let contents = valid_execution_manifest();
        let oara_exec_path = add_oara_exec_folder(&oara_config_path);
        let oara_exec_file_path = configure_execution_manifest(oara_exec_path, "t5", contents);

        // change file name
        let mut new_exec_file_path = oara_exec_file_path.clone();
        let _ = new_exec_file_path.pop();
        let new_file_name = format!("t6_{}", EXECUTION_MANIFEST_FILE);
        new_exec_file_path.push(new_file_name.as_str());
        fs::copy(oara_exec_file_path, new_exec_file_path).unwrap();

        let execution_manifest = load_execution_manifest(&oara_config_path, "");
        assert!(execution_manifest.is_ok());

        let validate = validate_manifest(machine_manifest, execution_manifest.unwrap());
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Duplicated application name : SM"),
        );

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    fn missing_dependency_app() {
        // load machine manifest
        let contents = valid_machine_manifest();
        let oara_config_path = configure_machine_manifest("configuration-t6", contents);
        let machine_manifest = load_machine_manifest(&oara_config_path).unwrap();

        // load execution manifest
        let app_manifest: &'static str = r#"
            name: SM
            app_dependency:
              - UCM.Running
            mode_dependency:
              - MachineFG.Startup
        "#;
        let oara_exec_path = add_oara_exec_folder(&oara_config_path);
        let _ = configure_execution_manifest(oara_exec_path, "t7", app_manifest);

        let execution_manifest = load_execution_manifest(&oara_config_path, "");
        assert!(execution_manifest.is_ok());

        let validate = validate_manifest(machine_manifest, execution_manifest.unwrap());
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Missing dependency application : UCM for SM"),
        );

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    #[ignore = "not implement missing dependency"]
    fn self_dependency_app() {
        // load machine manifest
        let contents = valid_machine_manifest();
        let oara_config_path = configure_machine_manifest("configuration-t7", contents);
        let machine_manifest = load_machine_manifest(&oara_config_path).unwrap();

        // load execution manifest
        let app_manifest: &'static str = r#"
            name: SM
            app_dependency:
              - SM.Running
            mode_dependency:
              - MachineFG.Startup
        "#;
        let oara_exec_path = add_oara_exec_folder(&oara_config_path);
        let _ = configure_execution_manifest(oara_exec_path, "t8", app_manifest);

        let execution_manifest = load_execution_manifest(&oara_config_path, "");
        assert!(execution_manifest.is_ok());

        let validate = validate_manifest(machine_manifest, execution_manifest.unwrap());
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Self dependency is not allowed : SM"),
        );

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    fn invalid_mode_dependency_app() {
        // load machine manifest
        let contents = valid_machine_manifest();
        let oara_config_path = configure_machine_manifest("configuration-t8", contents);
        let machine_manifest = load_machine_manifest(&oara_config_path).unwrap();

        // load execution manifest
        let app_manifest1: &'static str = r#"
            name: SM
            app_dependency:
              - UCM.Running
            mode_dependency:
              - MachineFG.Startup
        "#;
        let app_manifest2: &'static str = r#"
            name: UCM
            mode_dependency:
              - MachineFG.Shutdown
        "#;

        let oara_exec_path = add_oara_exec_folder(&oara_config_path);
        let _ = configure_execution_manifest(&oara_exec_path, "t9", app_manifest1);
        let _ = configure_execution_manifest(&oara_exec_path, "t10", app_manifest2);

        let execution_manifest = load_execution_manifest(&oara_config_path, "");
        assert!(execution_manifest.is_ok());

        let validate = validate_manifest(machine_manifest, execution_manifest.unwrap());
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Dependency app(UCM) is not in the mode"),
        );

        fs::remove_dir_all(&oara_config_path).unwrap();
    }

    #[test]
    fn circular_dependency_app() {}
}
