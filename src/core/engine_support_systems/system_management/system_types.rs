use std::fmt::Display;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum SystemType {
    Audio,
    Rendering,
    Physic,
    Log,
    Resource,
    Input,
    Filesystem,
}

impl Display for SystemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SystemType::Audio => {
                write!(f, "Audio system")
            },
            &SystemType::Log => {
                write!(f, "Log system")
            },
            &SystemType::Rendering => {
                write!(f, "Rendering system")
            },
            &SystemType::Resource => {
                write!(f, "Resource system")
            },
            &SystemType::Input => {
                write!(f, "Input system")
            },
            &SystemType::Physic => {
                write!(f, "Physic system")
            },
            &SystemType::Filesystem => {
                write!(f, "File system")
            }
        }
    }
}

