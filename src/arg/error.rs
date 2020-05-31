use crate::Arg;

#[derive(Debug, PartialEq)]
pub enum Error {
    Custom(String),
    CustomArg(Arg<'static>, String),
    NoArgs,
    Invalid(String),
    RequiresValue(Arg<'static>),
}
