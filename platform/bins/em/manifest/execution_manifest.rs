use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct EnterExitTimeout {
    enter: i32,
    exit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecutionManifest {
    name: String,
    environmental_variable: HashMap<String, String>,
    argument: HashMap<String, String>,
    #[serde(default = None)]
    enter_timeout_value: Option<EnterExitTimeout>,
    #[serde(default = false)]
    reporting_behavior: bool,
    #[serde(default = 0)]
    number_of_restart: i32,
    app_dependency: Vec<String>,
    mode_dependency: Vec<String>,
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

