use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ManagerMessages {
    waiting_new_iteration: String,
    start_new_iteration: String,
    stop_iteration: String,
    restart: String,
}

impl ManagerMessages {
    pub fn waiting_new_iteration(&self) -> &str {
        &self.waiting_new_iteration
    }

    pub fn start_new_iteration(&self) -> &str {
        &self.start_new_iteration
    }

    pub fn stop_iteration(&self) -> &str {
        &self.stop_iteration
    }

    pub fn restart(&self) -> &str {
        &self.restart
    }
}

impl Default for ManagerMessages {
    fn default() -> Self {
        Self {
            waiting_new_iteration: "Waiting new iteration...".into(),
            start_new_iteration: "Start new iteration!".into(),
            stop_iteration: "Stop iteration!".into(),
            restart: "Restarting...".into(),
        }
    }
}
