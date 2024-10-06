use thiserror::Error;
use anyhow::Result;
use super::execution_client::ExecutionClientError;
use super::FunctionGroupState;

#[derive(Debug, Error)]
enum StateClientError {
    #[error("cancelled by a newer request")]
    kCancelled,
    #[error("transition to the requested Function Group state failed")]
    kFailed,
    #[error("Unexpected Termination in Process of target Function Group State happened")]
    kFailedUnexpectedTerminationOnEnter,
    #[error("can’t communicate with Execution Management")]
    kCommunicationError,
    #[error("transition to the requested state is prohibited, or invalid state")]
    kInvalidTransition,
    #[error("an integrity or authenticity check failed during state transition")]
    kIntegrityOrAuthenticity,
    #[error("One of the processes terminated in an unexpected way during the state transition")]
    kFailedUnexpectedTermination,
    #[error("The given Function Group State couldn’t be found in the ProcessedManifest")]
    kMetaModelError,
}

/// Symbol: StateClient
/// Syntax: class StateClient final {...};
/// Description: StateClient is an interface of Execution Management that is used by State Management to
/// request transitions between Function Group States or to perform other related operations.
/// Notes: StateClient opens communication channel to Execution Management (e.g. POSIX FIFO). Each
/// Process that intends to perform state management, should create an instance of this class and it
/// should have rights to use it. To eventually implement the Named Constructor Idiom, the
/// developer may either make the default constructor private or delete it and define a non-default
/// constructor.
#[derive(Clone)]
struct StateClient {

    undefined_state_callback: Box<dyn Fn(ExecutionClientError)>
}

impl StateClient {
    /// [SWS_EM_02561]{DRAFT} Definition of API function ara::exec::StateClient::StateClient
    /// Symbol: StateClient(std::function< void(const ara::exec::ExecutionErrorEvent &)> undefinedState
    /// Callback)
    /// Syntax: StateClient (std::function< void(const ara::exec::ExecutionErrorEvent
    /// &)> undefinedStateCallback);
    /// Parameters (in): undefinedStateCallback callback to be invoked by StateClient library if a FunctionGroup
    /// changes its state unexpectedly to an Undefined Function Group
    /// State, i.e. without previous request by SetState(). The affected
    /// FunctionGroup and ExecutionError is provided as an argument to
    /// the callback in form of ExecutionErrorEvent
    /// Errors: ara::exec::ExecErrc::kCommunicationError communication error occurred
    /// Description: Regular constructor for StateClient.
    pub fn new(undefined_state_callback: Box<dyn Fn(ExecutionClientError)>) -> Result<Self> {
        Self {
            undefined_state_callback,
        }
    }

    /// [SWS_EM_02276] Definition of API function ara::exec::StateClient::Create
    /// Symbol: Create(std::function< void(const ara::exec::ExecutionErrorEvent &)> undefinedStateCallback)
    /// Syntax: static ara::core::Result< StateClient > Create (std::function<
    /// void(const ara::exec::ExecutionErrorEvent &)> undefinedStateCallback) noexcept;
    /// Parameters (in): undefinedStateCallback callback to be invoked by StateClient library if a FunctionGroup
    /// changes its state unexpectedly to an Undefined Function Group
    /// State, i.e. without previous request by SetState(). The affected
    /// FunctionGroup and ExecutionError is provided as an argument to
    /// the callback in form of ExecutionErrorEvent.
    /// Return value: ara::core::Result< StateClient > a result that contains either a StateClient object or an error.
    /// Errors: ara::exec::ExecErrc::kCommunicationError communication error occurred
    /// Description: Named constructor for StateClient.
    /// Notes: This named constructor may call a private constructor defined by the developer.
    pub fn create(undefined_state_callback: Box<dyn Fn(ExecutionClientError)>) -> Result<Self> {
        StateClient::new(undefined_state_callback)
    }

    /// [SWS_EM_02278] Definition of API function ara::exec::StateClient::SetState
    /// Syntax: ara::core::Future< void > SetState (const FunctionGroupState &state) const noexcept;
    /// Parameters (in): state representing meta-model definition of a state inside a specific
    /// Function Group. Execution Management will perform state
    /// transition from the current state to the state identified by this parameter.
    /// Return value: ara::core::Future< void > void if requested transition is successful, otherwise it returns Exec
    /// ErrorDomain error.
    /// Errors:
    /// ara::exec::ExecErrc::kCancelled
    ///   if transition to the requested Function Group state was cancelled by a newer request
    /// ara::exec::ExecErrc::kFailed
    ///   if transition to the requested Function Group state failed
    /// ara::exec::ExecErrc::kFailedUnexpectedTerminationOnEnter
    ///   if Unexpected Termination in Process of target Function Group State happened.
    /// ara::exec::ExecErrc::kCommunicationError
    ///   if StateClient can’t communicate with Execution Management (e.g./// IPC link is down)
    /// ara::exec::ExecErrc::kInvalidTransition
    ///   if transition to the requested state is prohibited (e.g. Off state for
    ///   MachineFG) or the requested Function Group State is invalid (e.g.
    ///   does not exist anymore after a software update)
    /// ara::exec::ExecErrc::kIntegrityOrAuthenticity
    ///   if an integrity or authenticity check failed during state transition
    /// ara::exec::ExecErrc::kFailedUnexpectedTermination
    ///   One of the processes terminated in an unexpected way during the state transition.
    /// ara::exec::ExecErrc::kMetaModelError
    ///   The given Function Group State couldn’t be found in the ProcessedManifest.
    /// Description: Method to request state transition for a single Function Group.
    /// This method will request Execution Management to perform state transition and return
    /// immediately. Returned ara::core::Future can be used to determine result of requested transition.
    pub async fn set_state(state: &FunctionGroupState) -> Result<()> {

        Ok(())
    }

    /// [SWS_EM_02279] Definition of API function ara::exec::StateClient::GetInitialMachineStateTransitionResult
    /// Symbol: GetInitialMachineStateTransitionResult()
    /// Syntax: ara::core::Future< void > GetInitialMachineStateTransitionResult () const noexcept;
    /// Return value: ara::core::Future< void > void if requested transition is successful, otherwise it returns Exec
    /// ErrorDomain error.
    /// Errors:
    /// ara::exec::ExecErrc::kCancelled
    ///   StateManagement may decide to cancel SWS_EM_01023
    ///   transition and start specific startup sequence. This could happen
    ///   for number of reasons and one of them could be interrupted Machine update sequence.
    /// ara::exec::ExecErrc::kFailed
    ///   if transition to the requested Function Group state failed
    /// ara::exec::ExecErrc::kCommunicationError
    ///   if StateClient can’t communicate with Execution Management (e.g.IPC link is down)
    /// Description: Method to retrieve result of Machine State initial transition to Startup state.
    /// TBD : errors..
    /// Notes: This method allows State Management to retrieve the result of a transition specified by SWS_
    /// EM_01023 and SWS_EM_02241. Please note that this transition happens once per machine life
    /// cycle, thus the result delivered by this method shall not change (unless machine is started again).
    pub async fn get_initial_machine_state_transition_result() -> Result<()> {
        Ok(())
    }

    /// Symbol: GetExecutionError(const ara::exec::FunctionGroupState &functionGroupState)
    /// Syntax: ara::core::Result< ara::exec::ExecutionErrorEvent > GetExecutionError
    ///  (const ara::exec::FunctionGroupState &functionGroupState) noexcept;
    /// Parameters (in): functionGroupState Function Group State of interest.
    /// Return value: ara::core::Result<ara::exec::ExecutionErrorEvent>
    ///   The execution error which changed the Function Group of the givenFunction Group State
    ///   to an Undefined Function Group State.
    /// Errors:
    /// ara::exec::ExecErrc::kMetaModelError
    ///   The given Function Group State couldn’t be found in the ProcessedManifest.
    /// ara::exec::ExecErrc::kFailed
    ///   The Function Group of the given Function Group State is not in an Undefined Function Group State.
    /// Description: Returns the execution error which changed the Function Group of the given Function Group
    ///   State to an Undefined Function Group State.
    ///   This function will return with error and will not return an ExecutionErrorEvent object, if the
    ///   Function Group is in a defined Function Group state again.
    /// TBD : errors
    pub fn get_execution_error(function_group_state: &FunctionGroupState) -> Result<ExecutionErrorEvent> {
        Ok(ExecutionClientError::kCommunicationError)
    }

    // TBD
    // [SWS_EM_02543]{DRAFT} Default value for ExecutionError dIn case of Unexpected
    // Termination or Unexpected Self-termination of a Modelled
    // Process which does not have an executionError configured, Execution Management
    // shall report the ExecutionError value 1.c(RS_EM_00101)
}