use lotus_engine::Resource;

#[derive(Clone, Resource)]
pub struct GameState {
    pub started: bool,
    pub running: bool,
    pub ended: bool,
    pub won: bool
}

impl Default for GameState {
    fn default() -> Self {
        return GameState {
            started: false,
            running: false,
            ended: false,
            won: false
        };
    }
}

#[derive(Clone, Resource)]
pub struct GridTracker {
    pub available_grid_cells: u32,
    pub available_blank_grid_cells: u32,
    pub available_mine_grid_cells: u32,
    pub end_game_mine_cell_row_index: usize,
    pub end_game_mine_cell_column_index: usize
}

impl Default for GridTracker {
    fn default() -> Self {
        return GridTracker {
            available_grid_cells: 81,
            available_blank_grid_cells: 15,
            available_mine_grid_cells: 20,
            end_game_mine_cell_row_index: 0,
            end_game_mine_cell_column_index: 0
        };
    }
}

// TODO: TIMER
