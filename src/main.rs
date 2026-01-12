mod app;
mod color_data;
mod ui;

use app::App;
use ui::ui;

use crossterm::{
    event::{read as read_event, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    Terminal,
};
use std::{error::Error, io};

use crate::ui::get_color_from_coordinator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();

    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        EnableMouseCapture,
    )?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), <B as Backend>::Error>
where
    <B as Backend>::Error: From<io::Error>,
{
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        match read_event()? {
            Event::Key(KeyEvent{code: KeyCode::Esc, ..}) => {
                return Ok(());
            }
            Event::Key(KeyEvent{code: KeyCode::Backspace, ..}) => app.delete_input(),
            Event::Key(KeyEvent{code: KeyCode::Char(c), ..}) => app.handle_input(c),
            Event::Mouse(e @ MouseEvent{kind: MouseEventKind::Down(MouseButton::Left), ..}) => {
                let frame = terminal.get_frame();
                let color = get_color_from_coordinator(&frame, &app, e.row, e.column);
                if let Some(color) = color {
                    app.select_color(&color.0, color.1);
                }
            }
            _ => {}
        }
    }
}
