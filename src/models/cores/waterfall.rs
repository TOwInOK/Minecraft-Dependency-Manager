use super::paper::ModelCorePaperFamily;

pub struct Waterfall();
impl ModelCorePaperFamily for Waterfall {
    const CORE_NAME: &'static str = "waterfall";
}
