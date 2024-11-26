use std::{
    collections::{HashMap, HashSet},
    io::Read,
    path::Path,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const RUNNING: &str = "Running";
pub const TERMINATED: &str = "Terminated";
pub const MACHINE_FG: &str = "MachineFG";
pub const STARTUP: &str = "Startup";
pub const SHUTDOWN: &str = "Shutdown";
pub const RESTART: &str = "Restart";
pub const OFF: &str = "Off";

#[derive(Debug, Error)]
enum MachineManifestError {
    #[error("Empty process-mode")]
    EmptyProcessMode(),
    #[error("Invalid Process mode({0})")]
    InvalidProcessMode(String),
    #[error("Empty MachineFG")]
    EmptyMachineFG(),
    #[error("Invalid initial mode({0}) for {1}")]
    InvalidFGInitialMode(String, String),
    #[error("Invalid mode({0}) for {1}")]
    InvalidFGMode(String, String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionGroupMode {
    pub initial_mode: String,
    pub mode: Vec<String>,
}

fn default_process_mode() -> Vec<String> {
    vec![RUNNING.to_owned(), TERMINATED.to_owned()]
}

fn default_function_group_set() -> HashMap<String, FunctionGroupMode> {
    let mut set = HashMap::new();
    set.insert(
        MACHINE_FG.to_owned(),
        FunctionGroupMode {
            initial_mode: STARTUP.to_owned(),
            mode: vec![STARTUP.to_owned(), SHUTDOWN.to_owned(), RESTART.to_owned()],
        },
    );
    set
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MachineManifest {
    #[serde(default)]
    pub default_application_timeout: Option<u32>,
    #[serde(default)]
    pub environment_variable: HashMap<String, String>,
    #[serde(default = "default_process_mode")]
    pub process_mode: Vec<String>,
    #[serde(default = "default_function_group_set")]
    pub function_group_set: HashMap<String, FunctionGroupMode>,
}

impl MachineManifest {
    pub fn from(contents: &str) -> Result<Self> {
        let manifest: MachineManifest = serde_yaml::from_str(&contents)?;

        // check validation
        let default_process_mode = vec![RUNNING.to_owned(), TERMINATED.to_owned()];
        let default_machine_fg = vec![STARTUP.to_owned(), SHUTDOWN.to_owned(), RESTART.to_owned()];

        // process
        if manifest.process_mode.is_empty() {
            return Err(MachineManifestError::EmptyProcessMode().into());
        }

        let mode_set: HashSet<_> = manifest.process_mode.iter().collect();
        let default_set: HashSet<_> = default_process_mode.iter().collect();
        if mode_set != default_set {
            let mode = manifest.process_mode.join(",");
            return Err(MachineManifestError::InvalidProcessMode(mode).into());
        }

        // MachineFG
        if !manifest.function_group_set.contains_key(MACHINE_FG) {
            return Err(MachineManifestError::EmptyMachineFG().into());
        }

        // other function groups
        for (name, fg) in manifest.function_group_set.iter() {
            if name == MACHINE_FG {
                if fg.initial_mode != STARTUP {
                    return Err(MachineManifestError::InvalidFGInitialMode(
                        fg.initial_mode.clone(),
                        name.clone(),
                    )
                    .into());
                }
                let mode_set: HashSet<_> = fg.mode.iter().collect();
                let default_set: HashSet<_> = default_machine_fg.iter().collect();
                if mode_set != default_set {
                    let mode = fg.mode.join(",");
                    return Err(MachineManifestError::InvalidFGMode(mode, name.clone()).into());
                }
            } else {
                if fg.initial_mode != OFF {
                    return Err(MachineManifestError::InvalidFGInitialMode(
                        fg.initial_mode.clone(),
                        name.clone(),
                    )
                    .into());
                }
                if !fg.mode.contains(&OFF.to_owned()) {
                    let mode = fg.mode.join(",");
                    return Err(MachineManifestError::InvalidFGMode(mode, name.clone()).into());
                }
            }
        }

        Ok(manifest)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        MachineManifest::from(&contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let machine_manifest_str = r#"
            default_application_timeout: 3  # 3 seconds
            environment_variable:
              ENV1: "environment variable smaple1"
              ENV2: "environment variable smaple2"
              ENV3: "environment variable smaple3"
            process_mode:
              - Running
              - Terminated
            function_group_set:
              MachineFG:
                initial_mode: "Startup"
                mode:
                  - "Startup"
                  - "Shutdown"
                  - "Restart"
              FG1:
                initial_mode: "Off"
                mode:
                  - "Off"
                  - "On"
              FG2:
                initial_mode: "Off"
                mode:
                  - "Off"
                  - "On"
            # ...
        "#;

        let machine_manifest = MachineManifest::from(machine_manifest_str).unwrap();
        assert_eq!(
            machine_manifest,
            MachineManifest {
                default_application_timeout: Some(3),
                environment_variable: {
                    let mut env = HashMap::new();
                    env.insert(
                        String::from("ENV1"),
                        String::from("environment variable smaple1"),
                    );
                    env.insert(
                        String::from("ENV2"),
                        String::from("environment variable smaple2"),
                    );
                    env.insert(
                        String::from("ENV3"),
                        String::from("environment variable smaple3"),
                    );
                    env
                },
                process_mode: vec![String::from(RUNNING), String::from(TERMINATED)],
                function_group_set: {
                    let mut set = HashMap::new();
                    set.insert(
                        String::from(MACHINE_FG),
                        FunctionGroupMode {
                            initial_mode: String::from(STARTUP),
                            mode: vec![
                                String::from(STARTUP),
                                String::from(SHUTDOWN),
                                String::from(RESTART),
                            ],
                        },
                    );
                    set.insert(
                        String::from("FG1"),
                        FunctionGroupMode {
                            initial_mode: String::from(OFF),
                            mode: vec![String::from(OFF), String::from("On")],
                        },
                    );
                    set.insert(
                        String::from("FG2"),
                        FunctionGroupMode {
                            initial_mode: String::from(OFF),
                            mode: vec![String::from(OFF), String::from("On")],
                        },
                    );
                    set
                }
            }
        );
    }

    #[test]
    fn default_serialize() {
        let machine_manifest_str = r#"
        "#;

        let machine_manifest = MachineManifest::from(machine_manifest_str).unwrap();
        assert_eq!(
            machine_manifest,
            MachineManifest {
                default_application_timeout: None,
                environment_variable: { HashMap::new() },
                process_mode: vec![String::from(RUNNING), String::from(TERMINATED)],
                function_group_set: {
                    let mut set = HashMap::new();
                    set.insert(
                        String::from(MACHINE_FG),
                        FunctionGroupMode {
                            initial_mode: String::from(STARTUP),
                            mode: vec![
                                String::from(STARTUP),
                                String::from(SHUTDOWN),
                                String::from(RESTART),
                            ],
                        },
                    );
                    set
                }
            }
        );
    }

    #[test]
    fn invalid_process_mode() {
        let machine_manifest_str = r#"
            process_mode:
              - Ready
              - Terminated
        "#;

        let manifest = MachineManifest::from(machine_manifest_str);
        assert_eq!(
            manifest.err().map(|e| e.to_string()).unwrap(),
            String::from("Invalid Process mode(Ready,Terminated)"),
        );
    }

    #[test]
    fn invalid_machine_fg() {
        let invalid_machine_fg_initial_mode = r#"
            function_group_set:
              MachineFG:
                initial_mode: "Shutdown"
                mode:
                  - "Startup"
                  - "Shutdown"
                  - "Restart"
        "#;

        let manifest = MachineManifest::from(invalid_machine_fg_initial_mode);
        assert_eq!(
            manifest.err().map(|e| e.to_string()).unwrap(),
            String::from("Invalid initial mode(Shutdown) for MachineFG")
        );

        let invalid_machine_fg_mode = r#"
            function_group_set:
              MachineFG:
                initial_mode: "Startup"
                mode:
                - "Startup"
                - "Reboot"
                - "Ready"
        "#;

        let manifest = MachineManifest::from(invalid_machine_fg_mode);
        assert_eq!(
            manifest.err().map(|e| e.to_string()).unwrap(),
            String::from("Invalid mode(Startup,Reboot,Ready) for MachineFG"),
        );
    }

    #[test]
    fn invalid_fg() {
        let invalid_fg_initial_mode = r#"
            function_group_set:
              MachineFG:
                initial_mode: "Startup"
                mode:
                  - "Startup"
                  - "Shutdown"
                  - "Restart"
              FG1:
                initial_mode: "On"
                mode:
                  - "Off"
                  - "On"
        "#;

        let manifest = MachineManifest::from(invalid_fg_initial_mode);
        assert_eq!(
            manifest.err().map(|e| e.to_string()).unwrap(),
            String::from("Invalid initial mode(On) for FG1"),
        );

        let invalid_fg_mode = r#"
            function_group_set:
              MachineFG:
                initial_mode: "Startup"
                mode:
                  - "Startup"
                  - "Shutdown"
                  - "Restart"
              FG1:
                initial_mode: "Off"
                mode:
                  - "Ready"
                  - "Go"
        "#;

        let manifest = MachineManifest::from(invalid_fg_mode);
        assert_eq!(
            manifest.err().map(|e| e.to_string()).unwrap(),
            String::from("Invalid mode(Ready,Go) for FG1"),
        );
    }
}

/*
/// Class Machine
/// Package M2::AUTOSARTemplates::AdaptivePlatform::MachineManifest
///
/// Manifest file to configure a Machine. The Machine Manifest
/// holds all configuration information which cannot be assigned
/// to a specific Executable or process.
struct MachineManifest {
    /// This aggregation defines a default timeout in the context
    /// of a given Machine with respect to the launching and termination of applications
    default_application_timeout: EnterExitTimeout,

    /// This aggregation represents the collection of environment
    /// variables that shall be added to the environment defined
    /// on the level of the enclosing Machine.
    environment_variables: Vector<TagWithOptionalValue>,

    /// Reference to the MachineDesign this Machine is implementing.
    // machine_design: TBD

    /// This represents the collection of processors owned by the enclosing machine
    // processor : Later, TBD

    /// Configuration of Adaptive Autosar module instances that
    /// are running on the machine.
    // moduleInstantiation : Later, TBD

    /// Deployment of secure communication protocol
    /// configuration settings to crypto module entities.
    // secureCommunicationDeployment : Later, TBD

    /// This attribute controls the behavior of how authentication
    /// affects the ability to launch for each Executable.
    // trustedPlatformExecutableLaunchBehavior : Later, TBD
}

/// Class MachineDesign
/// Package M2::AUTOSARTemplates::AdaptivePlatform::SystemDesign
/// Note This meta-class represents the ability to define requirements on a Machine in the context of designing a
/// system.
/// Let's do this later
struct MachineDesign {
    access_control: Option<AccessControl>,

    /// This aggregation defines the network connection of the machine.
    // let's how to do communicationConnector
}

struct Mode {
    inital_mode: String,
    modes: Vec<String>,
}*/
