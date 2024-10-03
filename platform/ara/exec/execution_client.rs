
/// [SWS_EM_02000] Definition of API enum `ara::exec::ExecutionState`
///
/// Defines the internal states of a Process (see 7.3.1).
/// Scoped enumeration of `uint8_t`.
///
/// # Values:
/// - `RUNNING = 0`: After a Process has been started by Execution Management,
///    it reports the `ExecutionState::RUNNING`.
pub enum ExeuctionClient {
    RUNNING = 0
}

pub struct ExecutionClient {
    service_id : u16,
    method_id : u16,
    client_id : u16,
}
