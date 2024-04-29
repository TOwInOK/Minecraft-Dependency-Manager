use super::paper::ModelCorePaperFamily;

pub struct Velocity();
impl ModelCorePaperFamily for Velocity {
    const CORE_NAME: &'static str = "velocity";
}
