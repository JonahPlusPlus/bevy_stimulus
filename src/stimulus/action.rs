use super::*;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Action {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub stimuli: Stimuli,
    pub scale: Option<f32>
}

impl Action {
    pub fn new<T>(stimuli: T, scale: Option<f32>) -> Self where T: Into<Stimuli> {
        Self {
            stimuli: stimuli.into(),
            scale
        }
    }
}
