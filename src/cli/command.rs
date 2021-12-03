use strum_macros::{EnumString, Display};

#[derive(Copy, Clone, Debug, Display)]
#[derive(EnumString)]
pub enum Command {
    Write
}
