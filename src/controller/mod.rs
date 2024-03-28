use crate::{config::Config, lock::lock::Lock};

//ff
pub struct Controller<'config, 'lock> {
    config: &'config mut Config,
    lock: &'lock mut Lock,
}

impl<'config, 'lock> Controller<'config, 'lock> {
    pub fn new(config: &'config mut Config, lock: &'lock mut Lock) -> Self {
        Self { config, lock }
    }
}
