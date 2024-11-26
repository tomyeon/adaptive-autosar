use serde::{Deserialize, Serialize};

pub type FunctionGroup = String;

pub const MACHINE_FG: &str = "MachineFg";
pub const STARTUP: &str = "Startup";
pub const RESTART: &str = "Restart";
pub const SHUTDOWN: &str = "Shutdown";
pub const ON: &str = "On";
pub const OFF: &str = "Off";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FunctionGroupState {
    pub function_group: FunctionGroup,
    pub function_group_state: String,
}

impl FunctionGroupState {
    pub fn new(function_group: FunctionGroup, function_group_state: String) -> Self {
        Self {
            function_group,
            function_group_state,
        }
    }
}

pub fn get_machine_fg_state(state: &str) -> FunctionGroupState {
    FunctionGroupState::new(
        MACHINE_FG.to_owned(),
        state.to_owned(),
    )
}

/*
// TBD: all function group's are obsolute

/// [SWS_EM_02263]{OBSOLETE} Definition of API class ara::exec::FunctionGroup
/// Scope: namespace ara::exec
/// Symbol: FunctionGroup
/// Syntax: class FunctionGroup final {...};
/// Description: Class representing Function Group defined in meta-model (ARXML).
/// Notes: Once created based on ARXML path, it’s internal value stay bounded to it for entire lifetime of an
/// object.
#[derive(PartialEq, Eq)]
struct FunctionGroup {
    name: String,
}

impl FunctionGroup {
    /// Symbol: Create(ara::core::StringView qualifiedShortName)
    /// Syntax: static ara::core::Result< FunctionGroup > Create (ara::core::StringView qualifiedShortName) noexcept;
    /// Parameters (in): qualifiedShortName stringified meta model identifier (short name path) where path
    /// separator is ’/’.
    /// Return value: ara::core::Result<FunctionGroup > an instance of FunctionGroup, or ExecErrc error.
    /// Exception Safety: noexcept
    /// Thread Safety: Thread-safe
    /// Errors: ara::exec::ExecErrc::kMetaModelError if qualifiedShortName passed is incorrect (e.g. FunctionGroupState
    ///                                              identifier has been passed).
    /// Description: Named constructor for FunctionGroup.
    /// This method shall validate/verify meta-model path passed and perform FunctionGroup object
    /// creation.
    pub fn new(qualified_short_name: String) -> Result<Self> {
        // let's see what to do
        // stringified meta model identifier (short name path) where path separator is ’/’.
        Ok(Self {
            name: qualified_short_name,
        })
    }
}

/// [SWS_EM_02269] Definition of API class ara::exec::FunctionGroupState
/// Scope: namespace ara::exec
/// Symbol: FunctionGroupState
/// Syntax: class FunctionGroupState final {...};
/// Description: Class representing Function Group State defined in meta-model (ARXML).
/// Notes: Once created based on ARXML path, it’s internal value stay bounded to it for entire lifetime of an object.
struct FunctionGroupState {
    function_group: FunctionGroup,
    function_group_state: String,
}

impl FunctionGroupState {
    pub fn new(function_group: FunctionGroup, qualified_short_name: String) -> Result<Self> {
        Ok(Self {
            function_group,
            function_group_state: qualified_short_name,
        })
    }
}*/
