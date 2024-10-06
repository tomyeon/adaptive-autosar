
use thiserror::Error;
use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Error, Debug)]
pub enum AraCoreError {
    #[error("No idea of error")]
    TbdError,
    #[error("Platform not initialized!")]
    PlatformIsNotInitialized,
}

pub mod instance_specifier;

lazy_static! {
    static ref INITALIZED: Mutex<bool> = Mutex::new(false);
}

/// [SWS_CORE_90021]{DRAFT} If a constructor or function takes an ara::core::
/// InstanceSpecifier as an argument it shall check for an initialized platform. That
/// is: ara::core::Initialize has been called successfully and ara::core::
/// Deinitialize has not (yet) been executed. If such a constructor or function is
/// called while the platform is not initialized it shall be treated as a Violation with
/// the message: "Platform not initialized! The platform needs to be
/// initialized before the execution of >constructor or function
/// name<.".c(RS_AP_00111, RS_AP_00142, RS_AP_00149)
///
/// [SWS_CORE_90022]{DRAFT} dIf a functionality (other than the ones mentioned in
/// [SWS_CORE_15002]) is called after ara::core::Deinitialize has been called,
/// the behavior is implementation-defined.c(RS_AP_00111)
/// Let's panic
pub fn is_platform_initialized() -> bool {
    *INITALIZED.lock().unwrap()
}

// pub mod optiona; // use Rust Option
// pub mod result;  // use Rust Result
// pub mod error_domain // todo!, let's see if rust Error can replace it
// pub mod error        // todo!, let's see if rust Error can replace it

/// [SWS_CORE_10001]{DRAFT} Definition of API function ara::core::Initialize
/// ara::core::Initialize allows a central initialization of all included shared libraries
/// of the ARA framework. This could include static initializers or the setup of
/// daemon links (details are up to the platform vendor).
///
/// The general advice for application developers is to call ara::core::Initialize
/// right at the entry point of the application.

/// [SWS_CORE_10001]{DRAFT} Definition of API function ara::core::Initialize
/// Syntax: Result< void > Initialize () noexcept;
/// Return value: Result< void > a Result with an error code, in case an error occurred
/// Exception Safety: noexcept
/// Description: (Pre-)Initialization of the ARA Framework.
///   Prior to this call, interaction with the ARA is not allowed with the exception of types intended to
///   be used independently of initialization as defined in [SWS_CORE_15002]. It is strongly
///   recommended to make this call in a place where it is guaranteed that static initialization has
///   completed.
pub fn initalize() -> Result<()> {
    // todo : implementation of platform initialization
    //  initialization of ARA framework specific data structures
    //  initialization of system resources
    //  spawning of background threads

    *INITALIZED.lock().unwrap() = true;
    Ok(())
}

/// [SWS_CORE_10002]{DRAFT} Definition of API function ara::core::Deinitialize
/// Syntax: Result< void > Deinitialize () noexcept;
/// Return value: Result< void > a Result with an error code, in case an error occurred
/// Exception Safety: noexcept
/// Description: Shutdown of the ARA Framework.
/// After this call, no interaction with the ARA is allowed with the exception of types intended to be
/// used independently of initialization as defined in [SWS_CORE_15002]. As a prerequisite to
/// calling this API it is expected that the use of ARA interfaces is completed (with the given
/// exceptions). It is strongly recommended to make this call in a place where it is guaranteed that
/// the static initialization has completed and destruction of statically initialized data has not yet
/// started.
pub fn deinitialize() -> Result<()> {
    // todo : implementation of platform deinitialization
    //   orderly shutdown of spawned background threads
    //   deallocation of dynamically allocated memory
    //   deallocation of other system resources

    *INITALIZED.lock().unwrap() = false;
    Ok(())
}


/// 7.2.3.2 SIGABRT handler
/// Note : Not support in Rust language
#[allow(dead_code)]
fn abort_handler() {
    unreachable!()
}

// Won't support the following types other than language's type
// 7.2.4.2.1 Array
// 7.2.4.2.2 Vector
// 7.2.4.2.3 Map
// 7.2.4.2.4 String and BasicString
// 7.2.4.2.5 SteadyClock
// 7.2.4.3.1 Optional
// 7.2.4.3.2 Variant
// 7.2.4.3.3 StringView
// 7.2.4.3.4 Span
// 7.2.4.3.5 Byte

// TBD
// 7.2.4.3.6 MemoryResource