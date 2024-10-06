
/// [SWS_CORE_00121]{DRAFT} Definition of API type ara::core::ErrorDomain::IdType
type IdType = u64;
/// [SWS_CORE_00122] Definition of API type ara::core::ErrorDomain::CodeType
type CodeType = u32;
/// [SWS_CORE_00123] Definition of API type ara::core::ErrorDomain::SupportDataType
type SupportDataType = u32;

/// [SWS_CORE_00110]{DRAFT} Definition of API class ara::core::ErrorDomain
/// Description: Encapsulation of an error domain.
/// An error domain is the controlling entity for ErrorCode’s error code values, and defines the
/// mapping of such error code values to textual representations.
/// This class is a literal type, and subclasses are strongly advised to be literal types as well.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDomain {
    id_type: IdType,
}

impl ErrorDomain {
    /// [SWS_CORE_00135]{DRAFT} Definition of API function ara::core::ErrorDomain::ErrorDomain
    /// Syntax: explicit constexpr ErrorDomain (IdType id) noexcept;
    /// Parameters (in): id the unique identifier
    /// Description: Construct a new instance with the given identifier.
    ///              Identifiers are expected to be system-wide unique.
    pub fn new(id_type: IdType) -> Self {
        Self {
            id_type
        }
    }

    /// [SWS_CORE_00151] Definition of API function ara::core::ErrorDomain::Id
    /// Symbol: Id()
    /// Syntax: constexpr IdType Id () const noexcept;
    /// Return value: IdType the identifier
    /// Exception Safety: noexcept
    /// Description: Return the unique domain identifier.
    pub fn id(&self) -> IdType {
        self.id_type
    }
}

trait TErrorDomain {
    /// [SWS_CORE_00152] Definition of API function ara::core::ErrorDomain::Name
    ///
    /// TBD : Need to consider that ErrorDomain is adequate for Rust
    /// Syntax: virtual const char * Name () const noexcept=0;
    /// Return value: const char * the name as a null-terminated string, never nullptr
    /// Exception Safety: noexcept
    /// Description: Return the name of this error domain.
    /// The returned pointer remains owned by class ErrorDomain and shall not be freed by clients.
    fn name() -> &str;

    /// [SWS_CORE_00153]{DRAFT} Definition of API function ara::core::ErrorDomain::
    /// Message
    /// Syntax: virtual const char * Message (CodeType errorCode) const noexcept=0;
    /// Parameters (in): errorCode the domain-specific error code
    /// Return value: const char * the text as a null-terminated string, never nullptr
    /// Exception Safety: noexcept
    /// Description: Return a textual representation of the given error code.
    /// It is a Violation if the errorCode did not originate from this error domain, and thus be subject to
    /// SWS_CORE_00003.
    /// The returned pointer remains owned by the ErrorDomain subclass and shall not be freed by
    /// clients.
    fn message() -> &str;
}