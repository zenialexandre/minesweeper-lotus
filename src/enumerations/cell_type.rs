#[derive(Clone, Default, Debug, PartialEq)]
pub enum CellType {
    #[default]
    Numerical,
    Blank,
    Mine
}

impl CellType {
    pub fn to_index(&self) -> u32 {
        return match self {
            CellType::Blank => 0,
            CellType::Numerical => 1,
            CellType::Mine => 2
        }
    }
}
