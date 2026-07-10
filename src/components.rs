use lotus_engine::Component;
use crate::enumerations::{icon_type::IconType, cell_type::CellType};

#[derive(Component)]
pub struct Fade(pub bool);

impl Default for Fade {
    fn default() -> Self {
        return Fade(false);
    }
}

#[derive(Component)]
pub struct Flag(pub bool);

impl Default for Flag {
    fn default() -> Self {
        return Flag(false);
    }
}

#[derive(Component)]
pub struct Icon {
    pub icon_type: IconType,
    pub is_pressed: bool
}

impl Default for Icon {
    fn default() -> Self {
        return Icon {
            icon_type: IconType::Smile,
            is_pressed: false
        };
    }
}

#[derive(Component)]
pub struct Cell {
    pub cell_type: CellType,
    pub is_available: bool,
    pub row_index: usize,
    pub column_index: usize
}

impl Default for Cell {
    fn default() -> Self {
        return Cell {
            cell_type: CellType::Numerical,
            is_available: true,
            row_index: 0,
            column_index: 0
        };
    }
}
