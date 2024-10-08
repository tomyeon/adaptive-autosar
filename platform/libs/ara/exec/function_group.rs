
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
}

impl FunctionGroupState {
    pub fn new(function_group: &FunctionGroup, qualified_short_name: &String) -> Result<Self> {
        Ok(Self {

        })
    }
}