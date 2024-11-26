use lazy_static::lazy_static;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use thiserror::Error;
use crate::function_group::FunctionGroupState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};

pub const OARA_SM_DOMAIN_SOCKET: &'static str = "/tmp/oara_sm_domain_socket";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmClientCommand {
    GetInitialState,
    SetState(FunctionGroupState),
    // TBD
}

#[derive(Debug, Clone, Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum InitialStateError {
    #[error("Failed to change state to MachineFg.Startup")]
    FailedInitializeInitialState,
    #[error("can’t communicate with Execution Management")]
    CommunicationError,
}

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum SetStateError {
    // not understand this requirement which means to call SetState from multi-thread or multi-process
    #[error("cancelled by a newer request")]
    Canceled,
    #[error("transition to the requested Function Group state failed")]
    Failed,
    #[error("Unexpected Termination in Process of target Function Group State happened")]
    FailedUnexpectedTerminationOnEnter,
    #[error("can’t communicate with Execution Management")]
    CommunicationError,
    #[error("transition to the requested state is prohibited, or invalid state")]
    InvalidTransition,
    #[error("an integrity or authenticity check failed during state transition")]
    IntegrityorAuthenticity,
    #[error("One of the processes terminated in an unexpected way during the state transition")]
    FailedUnexpectedTermination,
    #[error("The given Function Group State couldn’t be found in the ProcessedManifest")]
    MetamodelError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmResponse {
    GetInitialState(Result<(), InitialStateError>),
    SetState(Result<(), SetStateError>),
}

lazy_static! {
    static ref STATE_CLIENT: Arc<Mutex<StateClient>> = Arc::new(Mutex::new(StateClient::new()));
}

#[allow(unused)]    // FIXME
async fn connect<P>(path: Option<P>)
where
    P: AsRef<Path>,
{
    if path.is_none() {
        STATE_CLIENT.lock().await.connect(&OARA_SM_DOMAIN_SOCKET);
    } else {
        STATE_CLIENT.lock().await.connect(path.unwrap());
    }
}

#[allow(unused)]    // FIXME
async fn state_client() -> Arc<Mutex<StateClient>> {
    STATE_CLIENT.clone()
}

#[derive(Debug)]
struct StateClient {
    //undefined_state_callback: Box<dyn Fn(ExecutionClientError)>   // TBD
    socket: Option<UnixStream>,
}

impl StateClient {
    fn new() -> Self {
        Self { socket: None }
    }

    async fn connect<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        assert!(self.socket.is_none());
        match UnixStream::connect(path).await {
            Ok(stream) => {
                self.socket = Some(stream);
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
        //self.socket = Some(UnixStream::connect(OARA_SM_DOMAIN_SOCKET).await?);
        //let x = UnixStream::connect(OARA_SM_DOMAIN_SOCKET).await;
        //println!("--==============-> {:?}", x);
        //self.socket = x.unwrap();
    }

    /// ara::exec::ExecErrc::kCancelled
    ///   StateManagement may decide to cancel SWS_EM_01023
    ///   transition and start specific startup sequence. This could happen
    ///   for number of reasons and one of them could be interrupted Machine update sequence.
    /// ara::exec::ExecErrc::kFailed
    ///   if transition to the requested Function Group state failed
    /// ara::exec::ExecErrc::kCommunicationError
    ///   if StateClient can’t communicate with Execution Management (e.g.IPC link is down)

    #[allow(unused)]
    pub async fn get_initial_machine_state_transition_result(&mut self) -> Result<()> {
        assert!(self.socket.is_some());
        if let Some(socket) = self.socket.as_mut() {
            // serialze command
            let command = SmClientCommand::GetInitialState;
            let encoded_command = bincode::serialize(&command)?;
            socket.write_all(&encoded_command).await?;

            // wait the result from server
            let mut buffer = vec![0; 1024];

            match timeout(Duration::from_secs(1), socket.read(&mut buffer)).await {
                Ok(Ok(n)) => {
                    let response: SmResponse = bincode::deserialize(&buffer[..n])?;
                    match response {
                        SmResponse::GetInitialState(response) => match response {
                            Ok(_) => {}
                            Err(error) => {
                                return Err(error.into());
                            }
                        },
                        _ => {
                            panic!("Invalid response for GetInitialState");
                        }
                    }
                }
                Ok(Err(_)) => {
                    // error on read
                    return Err(InitialStateError::CommunicationError.into());
                }
                Err(_) => {
                    // timeout
                    return Err(InitialStateError::CommunicationError.into());
                }
            }
        }

        Ok(())
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
    #[allow(unused)]
    pub async fn set_state(&mut self, state: &FunctionGroupState) -> Result<()> {
        assert!(self.socket.is_some());
        if let Some(socket) = self.socket.as_mut() {
            // serialze command
            let command = SmClientCommand::SetState(state.clone());
            let encoded_command = bincode::serialize(&command)?;
            socket.write_all(&encoded_command).await?;

            // wait the result from server
            let mut buffer = vec![0; 1024];

            match timeout(Duration::from_secs(1), socket.read(&mut buffer)).await {
                Ok(Ok(n)) => {
                    let response: SmResponse = bincode::deserialize(&buffer[..n])?;
                    match response {
                        SmResponse::SetState(response) => match response {
                            Ok(_) => {}
                            Err(error) => {
                                return Err(error.into());
                            }
                        },
                        _ => {
                            panic!("Invalid response for SetState");
                        }
                    }
                }
                Ok(Err(_)) => {
                    // error on read
                    return Err(SetStateError::CommunicationError.into());
                }
                Err(_) => {
                    // timeout
                    return Err(SetStateError::CommunicationError.into());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::UnixListener;

    #[tokio::test]
    async fn get_initial_machine_state_transition() {
        let domain_socket_path = std::env::temp_dir().join("test_domain_socket1");
        let cloned_socket_path = domain_socket_path.clone();

        let handle = tokio::spawn(async move {
            if tokio::fs::metadata(&cloned_socket_path).await.is_ok() {
                tokio::fs::remove_file(&cloned_socket_path).await.unwrap();
            }

            let listener = UnixListener::bind(&cloned_socket_path).unwrap();
            let (mut stream, _) = listener.accept().await.unwrap();

            let mut buffer = vec![0; 1024];
            loop {
                match stream.read(&mut buffer).await {
                    Ok(len) => {
                        assert!(len > 0);
                        let request_command =
                            bincode::deserialize::<SmClientCommand>(&buffer).unwrap();
                        match request_command {
                            SmClientCommand::GetInitialState => {
                                let response = SmResponse::GetInitialState(Result::Ok(()));
                                let serialized_resonse = bincode::serialize(&response).unwrap();
                                stream.write(&serialized_resonse).await.unwrap();
                            }
                            SmClientCommand::SetState(_fg_state) => {
                                assert!(false);
                            }
                        }
                    }
                    Err(error) => {
                        panic!("error on read with '{:?}'", error);
                    }
                }
                break;
            }
        });

        // wait a second to create domain socket
        tokio::time::sleep(Duration::from_millis(10)).await;

        let mut state_client = StateClient::new();
        state_client.connect(&domain_socket_path).await.unwrap();

        let result = state_client
            .get_initial_machine_state_transition_result()
            .await;
        assert!(result.is_ok());

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn get_initial_machine_state_transition_failure() {
        let domain_socket_path = std::env::temp_dir().join("test_domain_socket2");
        let cloned_socket_path = domain_socket_path.clone();

        let handle = tokio::spawn(async move {
            if tokio::fs::metadata(&cloned_socket_path).await.is_ok() {
                tokio::fs::remove_file(&cloned_socket_path).await.unwrap();
            }

            let listener = UnixListener::bind(&cloned_socket_path).unwrap();
            let (mut stream, _) = listener.accept().await.unwrap();

            let mut buffer = vec![0; 1024];
            loop {
                match stream.read(&mut buffer).await {
                    Ok(len) => {
                        assert!(len > 0);
                        let request_command =
                            bincode::deserialize::<SmClientCommand>(&buffer).unwrap();
                        match request_command {
                            SmClientCommand::GetInitialState => {
                                let response = SmResponse::GetInitialState(Err(InitialStateError::FailedInitializeInitialState.into()));
                                let serialized_resonse = bincode::serialize(&response).unwrap();
                                stream.write(&serialized_resonse).await.unwrap();
                            }
                            SmClientCommand::SetState(_fg_state) => {
                                assert!(false);
                            }
                        }
                    }
                    Err(error) => {
                        panic!("error on read with '{:?}'", error);
                    }
                }
                break;
            }
        });

        // wait a second to create domain socket
        tokio::time::sleep(Duration::from_millis(10)).await;

        let mut state_client = StateClient::new();
        state_client.connect(&domain_socket_path).await.unwrap();

        let result = state_client
            .get_initial_machine_state_transition_result()
            .await;

        let error = result.err().unwrap();
        let initial_state_error = error.downcast_ref::<InitialStateError>().unwrap();
        assert_eq!(
            initial_state_error,
            &InitialStateError::FailedInitializeInitialState
        );

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn set_state() {
        let domain_socket_path = std::env::temp_dir().join("test_domain_socket3");
        let cloned_socket_path = domain_socket_path.clone();

        let handle = tokio::spawn(async move {
            if tokio::fs::metadata(&cloned_socket_path).await.is_ok() {
                tokio::fs::remove_file(&cloned_socket_path).await.unwrap();
            }

            let listener = UnixListener::bind(&cloned_socket_path).unwrap();
            let (mut stream, _) = listener.accept().await.unwrap();

            let mut buffer = vec![0; 1024];
            loop {
                match stream.read(&mut buffer).await {
                    Ok(len) => {
                        assert!(len > 0);
                        let request_command =
                            bincode::deserialize::<SmClientCommand>(&buffer).unwrap();
                        match request_command {
                            SmClientCommand::GetInitialState => {
                                assert!(false);
                            }
                            SmClientCommand::SetState(fg_state) => {
                                assert_eq!(fg_state, FunctionGroupState{
                                    function_group: "MachineFg".to_owned(),
                                    function_group_state: "Startup".to_owned(),
                                });

                                let response = SmResponse::SetState(Ok(()));
                                let serialized_resonse = bincode::serialize(&response).unwrap();
                                stream.write(&serialized_resonse).await.unwrap();
                            }
                        }
                    }
                    Err(error) => {
                        panic!("error on read with '{:?}'", error);
                    }
                }
                break;
            }
        });

        // wait a second to create domain socket
        tokio::time::sleep(Duration::from_millis(10)).await;

        let mut state_client = StateClient::new();
        state_client.connect(&domain_socket_path).await.unwrap();

        let fg_state = FunctionGroupState::new("MachineFg".to_owned(), "Startup".to_owned());
        let result = state_client.set_state(&fg_state).await;
        assert!(result.is_ok());

        handle.await.unwrap();
    }
}

/*use super::execution_client::ExecutionClientError;
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

        let listener = UnixListener::bind(socket_path)?;
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
}*/
