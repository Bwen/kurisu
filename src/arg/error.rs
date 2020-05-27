use crate::Arg;

#[derive(Debug)]
pub enum Error {
    NoArgs,
    Invalid(String),
    RequiresValue(Arg<'static>),
}
