pub mod machine_manifest;
pub mod execution_manifest;
pub mod service_manifest;
pub mod parse;

/*type TimeValue = i32;

/// [constr_10432]{DRAFT} Existence of attribute Machine.defaultApplication-
/// Timeout.exitTimeoutValue dFor each Machine, the attribute defaultApplicationTimeout.
/// exitTimeoutValue shall exist at the time when the creation
/// of the manifest is finished
struct EnterExitTimeout {
    enter_timeout_value: Option<TimeValue>,
    exit_timeout_value: Option<TimeValue>,
}

/// Class TagWithOptionalValue
/// Package M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::TagWithOptionalValue
/// Note A tagged value is a combination of a tag (key) and a value that gives supplementary information that is
/// attached to a model element. Please note that keys without a value are allowed.
struct TagWithOptionalValue {
    /// Defines a key.
    key: Option<String>,

    /// The sequenceOffset attribute supports the use case
    /// where TagWithOptionalValue is aggregated as splitable. If
    /// multiple aggregations define the same value of attribute
    /// key then the order in which the value collection is merged
    /// might be significant. As an example consider the
    /// modeling of the $PATH environment variable by means of
    /// a meta class TagWithOptionalValue. The sequenceOffset
    /// describes the relative position of each contribution in the
    /// concatenated value. The contributions are sorted in
    /// increasing integer order.
    sequence_offset: Option<u32>,

    /// Defines the corresponding value.
    value: Option<String>,
}

/// [TPS_MANI_03209]{DRAFT} The meaning of MachineDesign.accessControl
/// dThe MachineDesign.accessControl defines whether the access control is defined
/// by AUTOSAR means in the Application Design with receiverIntent (see
/// [TPS_MANI_01106]) and senderIntent (see [TPS_MANI_01107]) or by a custom
/// lists that are created by a non-AUTOSAR process.
enum AccessControl {
    /// The access restriction to the resource is defined by a non-AUTOSAR process.
    Custom,
    /// The access restriction to the resource is modeled in the AUTOSAR Application Design model or the
    /// AUTOSAR Deployment model.
    Modeled,
}

/// Note This enumeration defines the possible buildTypes a software module may be implemented.
enum BuildType {
    /// buildTypeRelease Used for releasing.
    Release,
    /// buildTypeDebug Used for debugging.
    Debug,
}

/// [TPS_MANI_01279]{DRAFT} Semantics of Executable.reportingBehavior
/// Attribute Executable.reportingBehavior shall be used to control the reporting
/// Note This enumeration provides options for controlling of how an Executable reports its execution state to
/// the Execution Management
enum ReportingBehavior {
    /// The Executable shall report its execution state to the Execution Management
    ReportsExecutionState,

    /// The Executable shall not report its execution state to the Execution Management.
    DoesNotReportExecutionState,
}

/// Class ModeDeclaration
// Package M2::AUTOSARTemplates::CommonStructure::ModeDeclaration
// Note Declaration of one Mode. The name and semantics of a specific mode is not defined in the meta-model.
struct ModeDeclaration {
    /// value PositiveInteger 0..1 attr
    /// Note : The RTE shall take the value of this attribute for
    /// generating the source code representation of this Mode Declaration.
    value : u32,    // FIXME : will be to String
}

/// Class ModeDeclarationGroup
/// Package M2::AUTOSARTemplates::CommonStructure::ModeDeclaration
/// Note A collection of Mode Declarations. Also, the initial mode is explicitly identified.
struct ModeDeclarationGroup {

    initial_mode: String,
}*/