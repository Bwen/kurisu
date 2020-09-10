use kurisu::*;

#[derive(Debug, Kurisu)]
pub struct Create {
    #[kurisu(pos = 1)]
    pub action: String,
    #[kurisu(pos = 2)]
    pub name1: String,
}
