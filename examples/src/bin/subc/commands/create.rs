use kurisu::*;

#[derive(Debug, Kurisu)]
pub struct Create {
    #[kurisu(pos = 1)]
    pub action: String,
    pub name1: String,
}
