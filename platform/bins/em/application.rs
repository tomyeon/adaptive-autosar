use ara_exec::manifest::execution_manifest::ExecutionManifest;
use libc::pid_t;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum ProcessState {
    Idle,
    Starting,
    Running,
    Terminating,
    Terminated,
}

pub struct Process {
    pub execution_manifest: Arc<Mutex<ExecutionManifest>>,
    pub process_state: ProcessState,
    pub pid: Option<pid_t>,
}
