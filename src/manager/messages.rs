use std::sync::Arc;

use indicatif::ProgressBar;

pub enum Messages {
    Restart(Arc<ProgressBar>),
    Stop(Arc<ProgressBar>),
    Start(Arc<ProgressBar>),
}
