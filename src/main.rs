mod enumerations;
mod resources;

use lotus_engine::*;
use crate::resources::{GameState, GridTracker};

your_game!(
    WindowConfiguration {
            icon_path: "textures/icon/yeah_cell.png".to_string(),
            title: "Minesweeper".to_string(),
            background_color: Some(Color::by_option(ColorOption::Black)),
            background_image_path: None,
            width: 385.0,
            height: 510.0,
            position_x: 200.0,
            position_y: 150.0,
            resizable: false,
            decorations: true,
            transparent: false,
            active: true,
            enabled_buttons: WindowButtons::CLOSE | WindowButtons::MINIMIZE,
            present_mode: PresentMode::AutoNoVsync
        },
        setup,
        update
);

fn setup(context: &mut Context) {
    context.commands.add_resources(vec![
        Box::new(GameState::default()),
        Box::new(GridTracker::default())
    ]);
}

fn update(context: &mut Context) {

}
