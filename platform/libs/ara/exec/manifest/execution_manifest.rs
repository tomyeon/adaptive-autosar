use std::{collections::HashMap, io::Read};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::machine_manifest::MachineManifest;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct EnterExitTimeout {
    enter: i32,
    exit: i32,
}

#[derive(Debug, Error)]
enum ExecutionManifestError {
    //#[error("Empty process name")]
    //EmptyProcessName(),
    #[error("Inavlid application dependency format: {0} for {1}")]
    InvalidApplicationDependencyFormat(String, String),
    #[error("Inavlid application dependency: {0} for {1}")]
    InvalidApplicationDependencyMode(String, String),
    #[error("Inavlid mode dependency format: {0} for {1}")]
    InvalidModeDependencyFormat(String, String),
    #[error("Function group({0}) doesn't exist for {1}")]
    FGNotExist(String, String),
    #[error("No mode({0}) for {1}")]
    NoModeInFG(String, String),
}

// DO NOT ADD Default derive
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionManifest {
    name: String,
    #[serde(default)]
    environmental_variable: HashMap<String, String>,
    #[serde(default)]
    argument: HashMap<String, String>,
    #[serde(default)]
    enter_exit_timeout: Option<EnterExitTimeout>,
    #[serde(default)]
    reporting_behavior: bool,
    #[serde(default)]
    number_of_restart: i32,
    #[serde(default)]
    app_dependency: Vec<String>,
    #[serde(default)]
    mode_dependency: Vec<String>,
}

impl ExecutionManifest {
    pub fn from(contents: &str) -> Result<Self> {
        let manifest: ExecutionManifest = serde_yaml::from_str(&contents)?;
        Ok(manifest)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        ExecutionManifest::from(&contents)
    }

    pub fn validate(&self, machine_manifest: &MachineManifest) -> Result<()> {
        // check app-dependency
        for dependency in &self.app_dependency {
            match dependency.split_once('.') {
                Some((_name, mode)) => {
                    if !machine_manifest.process_mode.contains(&mode.to_string()) {
                        return Err(ExecutionManifestError::InvalidApplicationDependencyMode(
                            dependency.clone(),
                            self.name.clone(),
                        )
                        .into());
                    }
                }
                None => {
                    return Err(ExecutionManifestError::InvalidApplicationDependencyFormat(
                        dependency.clone(),
                        self.name.clone(),
                    )
                    .into());
                }
            }
        }

        // check mode-dependency
        for dependency in &self.mode_dependency {
            match dependency.split_once('.') {
                Some((fg, mode)) => {
                    let fg = fg.to_string();
                    if machine_manifest.function_group_set.contains_key(&fg) {
                        println!("{} ==> {:?}", fg, machine_manifest.function_group_set.get(&fg));
                        if let Some(fg_mode) = machine_manifest.function_group_set.get(&fg) {
                            let mode = mode.to_string();
                            if !fg_mode.mode.contains(&mode) {
                                return Err(ExecutionManifestError::NoModeInFG(
                                    dependency.clone(), self.name.clone(),
                                )
                                .into());
                            }
                        }
                    } else {
                        return Err(
                            ExecutionManifestError::FGNotExist(fg, self.name.clone()).into()
                        );
                    }
                }
                None => {
                    return Err(ExecutionManifestError::InvalidModeDependencyFormat(
                        dependency.clone(),
                        self.name.clone(),
                    )
                    .into());
                }
            }
        }

        Ok(())
    }
}

mod tests {
    use super::*;
    use super::super::machine_manifest::FunctionGroupMode;

    #[test]
    fn serialize() {
        let execution_manifest_str = r#"
            name: SM
            environmental_variable:
              ENV1: "environment variable smaple1"
              ENV2: "environment variable smaple2"
            argument:
              ARG1: "argument variable1"
              ARG2: "argument variable2"
            enter_exit_timeout: # use machine_manifest's default_applicattion_timeout if omits
              enter: 1          # 1 second
              exit: 1           # 1 second
            reporting_behavior: true # true or false
            number_of_restart: 0     # really dont' know it is necessary
            app_dependency:
              - UCM.Running
              - APP.Running
            mode_dependency:
              - MachineFG.Startup
        "#;

        let execution_manifest = ExecutionManifest::from(execution_manifest_str).unwrap();
        assert_eq!(
            execution_manifest,
            ExecutionManifest {
                name: String::from("SM"),
                environmental_variable: {
                    let mut map = HashMap::new();
                    map.insert(
                        String::from("ENV1"),
                        String::from("environment variable smaple1"),
                    );
                    map.insert(
                        String::from("ENV2"),
                        String::from("environment variable smaple2"),
                    );
                    map
                },
                argument: {
                    let mut map = HashMap::new();
                    map.insert(String::from("ARG1"), String::from("argument variable1"));
                    map.insert(String::from("ARG2"), String::from("argument variable2"));
                    map
                },
                enter_exit_timeout: Some(EnterExitTimeout { enter: 1, exit: 1 }),
                reporting_behavior: true,
                number_of_restart: 0,
                app_dependency: vec![String::from("UCM.Running"), String::from("APP.Running"),],
                mode_dependency: vec![String::from("MachineFG.Startup"),],
            }
        )
    }

    #[test]
    fn default_serialize() {
        let execution_manifest_str = r#"
        "#;

        let execution_manifest = ExecutionManifest::from(execution_manifest_str);
        assert!(execution_manifest.is_err());
    }

    #[test]
    fn app_dependency_validate() {
        let execution_manifest_str = r#"
            name: TestApp
        "#;

        let mut execution_manifest = ExecutionManifest::from(&execution_manifest_str).unwrap();
        let machine_manifest = MachineManifest::from("").unwrap();
        execution_manifest.name = "TestApp".to_owned();
        execution_manifest.app_dependency = vec!["APP1.Running".to_owned(), "APP2.Terminated".to_owned()];
        assert!(execution_manifest.validate(&machine_manifest).is_ok());

        // InvalidApplicationDependencyFormat
        execution_manifest.app_dependency = vec!["APP1Running".to_owned(), "APP2.Terminated".to_owned()];
        let validate = execution_manifest.validate(&machine_manifest);
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Inavlid application dependency format: APP1Running for TestApp"),
        );

        // InvalidApplicationDependencyMode
        execution_manifest.app_dependency = vec!["APP1.Running".to_owned(), "APP2.Terminating".to_owned()];
        let validate = execution_manifest.validate(&machine_manifest);
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Inavlid application dependency: APP2.Terminating for TestApp"),
        );
    }

    #[test]
    fn mode_dependency_validate() {
        let execution_manifest_str = r#"
            name: TestApp
        "#;

        let mut execution_manifest = ExecutionManifest::from(&execution_manifest_str).unwrap();
        let mut machine_manifest = MachineManifest::from("").unwrap();

        execution_manifest.name = "TestApp".to_owned();
        execution_manifest.mode_dependency = vec!["MachineFG.Startup".to_owned()];
        assert!(execution_manifest.validate(&machine_manifest).is_ok());

        // InvalidModeDependencyFormat
        execution_manifest.mode_dependency = vec!["MachineFGStartup".to_owned()];
        let validate = execution_manifest.validate(&machine_manifest);
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Inavlid mode dependency format: MachineFGStartup for TestApp"),
        );

        // FGNotExist
        execution_manifest.mode_dependency = vec!["FG1.On".to_owned()];
        let validate = execution_manifest.validate(&machine_manifest);
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("Function group(FG1) doesn't exist for TestApp"),
        );

        // EmptyModeInFg
        execution_manifest.mode_dependency = vec!["FG1.On".to_owned()];
        machine_manifest.function_group_set.insert("FG1".to_owned(), FunctionGroupMode {
            initial_mode: "FG1".to_owned(),
            mode: vec![],
        });
        let validate = execution_manifest.validate(&machine_manifest);
        assert_eq!(
            validate.err().map(|e| e.to_string()).unwrap(),
            String::from("No mode(FG1.On) for TestApp"),
        );
    }
}

/*
/// Class Executable
/// Package M2::AUTOSARTemplates::AdaptivePlatform::ApplicationDesign::ApplicationStructure
/// Note This meta-class represents an executable program.
struct Executable {
    /// This attribute describes the buildType of a module and/or platform implementation.
    build_type: Option<BuildType>,

    /// This aggregation contains the collection of
    /// implementation-specific properties necessary to properly
    /// build the enclosing Executable.
    /// Not understandable
    //implementation_props: Vec<()>

    /// this attribute controls the execution state reporting
    /// behavior of the enclosing Executable.
    reporting_behavior: Option<ReportingBehavior>,

    /// This represents the root SwCompositionPrototype of the
    /// Executable. This aggregation is required (in contrast to a
    /// direct reference of a SwComponentType) in order to
    /// support the definition of instanceRefs in Executable
    /// context.
    /// not here
    //root_sw_omponent_prototype:


}

struct Process {
    /// Reference to executable that is executed in the process.
    executable: Executable,

    /// This attribute specifies which functional cluster the Process is affiliated with.
    number_of_restart_attempts: u32,

    process_state_machine:
}

/// The purpose of the execution manifest is to provide information that is needed for the
/// actual deployment of an application (formally modeled as an SwComponentType) onto
/// the AUTOSAR adaptive platform
struct ExecutionManifest {

}*/
