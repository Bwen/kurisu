use crate::Arg;

#[derive(Debug, PartialEq)]
pub enum Error {
    Custom(String),
    CustomArg(Arg<'static>, String),
    NoArgs,
    Invalid(String),
    RequiresPositional(Arg<'static>),
    RequiresValue(Arg<'static>),
    RequiresValueIf(Arg<'static>, Arg<'static>),
}
