use thiserror::Error;
use anyhow::Result;

/// InstanceSpecifierError
#[derive(Error, Debug)]
pub enum InstanceSpecifierError {
    // CoreErrc::kInvalidMetaModelShortname
    //   if any of the path elements of metaModelIdentifier is missing or contains invalid characters
    #[error("Invalid InstanceSpecifer: Invalid meta model shortname (0)")]
    InvalidMetaModelShortname(String),
    // CoreErrc::kInvalidMetaModelPath
    //   if the metaModelIdentifier is not a valid path to a model element
    #[error("Invalid InstanceSpecifer: Invalid meta model path (0)")]
    InvalidMetaModelPath(String),
}

/// [SWS_CORE_08001] Definition of API class ara::core::InstanceSpecifier
/// Symbol: InstanceSpecifier
/// Description: class representing an AUTOSAR Instance Specifier, which is basically an AUTOSAR
/// shortname-path wrapper.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstanceSpecifier<'a> {
    meta_model_identifier: &'a str,
}

impl<'a> InstanceSpecifier<'a> {
    /// [SWS_CORE_08021] Definition of API function ara::core::InstanceSpecifier::InstanceSpecifier
    /// metaModelIdentifier string representation of a valid InstanceSpecifier, according to the
    /// syntax rules given by SWS_CORE_10200 and SWS_CORE_10203.
    /// CoreException in case the given metaModelIdentifier is not a valid meta-model
    /// identifier/short name path.
    ///
    /// [SWS_CORE_10203] Valid InstanceSpecifier representations - functional cluster interaction
    pub fn new(meta_model_identifier: &'a str) -> Result<Self> {
        InstanceSpecifier::validate(meta_model_identifier)
    }

    fn validate(meta_model_identifier: &'a str) -> Result<Self> {
        // [SWS_CORE_10200] Valid InstanceSpecifier representations - application interaction
        // dIn case of application interaction and thus in the presence of PortPrototypes
        // the string representation of a valid ara::core::InstanceSpecifier
        // consists of a "/"-separated list of model element shortNames starting from an Executable
        // via the RootSwComponentPrototype and several SwComponentPrototypes
        // to the respective PortPrototype to which the ara::core::Instance-
        // Specifier shall apply.c(RS_AP_00130)
        // Thus, in case of application interaction the content of a valid ara::core::InstanceSpecifier
        // adheres to the following pattern:
        // Executable.shortName/RootSwComponentPrototype.shortName/SwComponentPrototype.shortName/.../PortPrototype.shortName

        // [SWS_CORE_10203] Valid InstanceSpecifier representations - functional cluster
        // interaction. In case of functional cluster interaction and thus in the absence
        // of PortPrototypes the string representation of a valid ara::core::Instance-
        // Specifier consists of a "/"-separated list of model element shortNames starting
        // from a top-level ARPackage via contained sub-packages to the respective mapping
        // element that is derived from FunctionalClusterInteractsWithFunctional-
        // ClusterMapping (see TPS_MANI_03268 for further details).c(RS_AP_00130)
        // Thus, in case of functional cluster interaction the content of a valid ara::core::
        // InstanceSpecifier adheres to the following pattern:
        // ARPackage.shortName/.../ARPackage.shortName/FunctionalClusterInteractsWithFunctionalClusterMapping.shortName

        // xxx.xxx/.../xxx.xxx
        let parts: Vec<&str> = meta_model_identifier.split('/').collect();
        if parts.is_empty() {
            return Err(InstanceSpecifierError::InvalidMetaModelPath(meta_model_identifier.to_owned()).into());
        }

        for part in parts {
            // xxx.xxx
            if part.is_empty() {
                return Err(InstanceSpecifierError::InvalidMetaModelPath(meta_model_identifier.to_owned()).into());
            }

            if part.split('.').collect::<Vec<&str>>().len() != 2 {
                return Err(InstanceSpecifierError::InvalidMetaModelShortname(meta_model_identifier.to_owned()).into());
            }
        }

        Ok(Self {
            meta_model_identifier
        })
    }

    /// [SWS_CORE_08032] Definition of API function ara::core::InstanceSpecifier::Create
    /// Symbol: Create(StringView metaModelIdentifier)
    /// Syntax: static Result< InstanceSpecifier > Create (StringView metaModelIdentifier) noexcept;
    /// Parameters (in): metaModelIdentifier string representation of a valid InstanceSpecifier, according to the
    ///                  syntax rules given by SWS_CORE_10200 and SWS_CORE_10203.
    /// Return value: Result< InstanceSpecifier >
    ///              a Result, containing either a syntactically valid InstanceSpecifier, or an ErrorCode
    /// Exception Safety: noexcept
    /// Errors :
    ///      CoreErrc::kInvalidMetaModelShortname
    ///          if any of the path elements of metaModelIdentifier is missing or contains invalid characters
    ///      CoreErrc::kInvalidMetaModelPath
    ///           if the metaModelIdentifier is not a valid path to a model element
    /// Description: Create a new instance of this class.
    pub fn create(meta_model_identifier: &'a str) -> Result<Self> {
         InstanceSpecifier::validate(meta_model_identifier)
    }

    /// Symbol: ToString()
    /// Syntax: StringView ToString () const noexcept;
    /// Return value: StringView stringified form of InstanceSpecifier. Lifetime of the underlying
    /// string is only guaranteed for the lifetime of the underlying string of
    /// the StringView passed to the constructor.
    /// Exception Safety: noexcept
    /// Description: method to return the stringified form of InstanceSpecifier
    #[inline(always)]
    //fn to_string(&self) -> &'a str {
    // to_string is not adequate to Rust
    fn as_str(&self) -> &'a str {
        &self.meta_model_identifier
    }
}