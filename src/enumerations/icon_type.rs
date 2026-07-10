#[derive(Clone, Default, Debug, PartialEq)]
pub enum IconType {
    #[default]
    Smile,
    Sad,
    Wow,
    Yeah
}

impl IconType {
    pub fn to_index(&self) -> u32 {
        return match self {
            IconType::Smile => 0,
            IconType::Sad => 1,
            IconType::Wow => 2,
            IconType::Yeah => 3
        }
    }
}
