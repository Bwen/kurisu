use kurisu::*;

#[derive(Debug, Kurisu)]
pub struct Delete {
    #[kurisu(pos = 1)]
    pub action: String,
    pub name2: Option<String>,
}
