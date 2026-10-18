mod app;
mod vec3;
mod object;
mod input_handler;
mod matrix;
mod display_manager;
mod camera;

use color_eyre::Result;
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture,
    },
    execute
};
use std::io::stdout;
use app::App;


fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::default();
    let mut stdout = stdout();

    execute!(stdout, EnableMouseCapture)?;

    let app_result = app.run(&mut terminal);

    execute!(stdout, DisableMouseCapture)?;
    ratatui::restore();
    app_result
}

