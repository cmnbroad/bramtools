use strum_macros::{EnumString, Display};

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString, Display)]
pub enum Command {
    Write
}
