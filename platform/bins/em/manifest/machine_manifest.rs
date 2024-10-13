use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FunctionGroup {
    inital_mode: String,
    mode: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MachineManifest {
    default_application_timeout: u32,
    environment_variable: HashMap<String, String>,
    process_mode: Vector<String>,
    function_group_set: Vector<FunctionGroup>,
}

mod tests {

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

