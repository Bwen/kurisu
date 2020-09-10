use kurisu::*;

#[derive(Debug, Kurisu)]
pub struct Delete {
    #[kurisu(pos = 1)]
    pub action: String,
    #[kurisu(pos = 2)]
    pub name2: String,
}
