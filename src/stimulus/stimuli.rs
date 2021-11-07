use serde::{Serialize, Deserialize};
use bevy::prelude::*;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Clone, Debug)]
pub enum Stimuli {
    Digital { digital: Digital },
    Analog { analog: Analog }
}

// Digital Conversions
impl From<KeyCode> for Stimuli {
    fn from(key_code: KeyCode) -> Self {
        Self::Digital {
            digital: Digital::Keyboard(key_code)
        }
    }
}

// Analog Conversions



#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Clone, Debug)]
pub enum Digital {
    Keyboard(KeyCode)
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Clone, Debug)]
pub enum Analog {
    AnyGamepad(GamepadAxisType), // TODO: Check if this is even possible
    SpecificGamepad(GamepadAxis),
    Keyboard (KeyCode, KeyCode),
    MouseAxis(MouseAxisType),
    MouseButton(MouseButton, MouseButton)
}


#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum MouseAxisType {
    MouseX,
    MouseY,
    ScrollX,
    ScrollY
}