use super::paper::ModelCorePaperFamily;

pub struct Folia();
impl ModelCorePaperFamily for Folia {
    const CORE_NAME: &'static str = "folia";
}
