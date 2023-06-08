use app::run_app;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::{Duration}};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};

pub mod app;
pub mod datatypes;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    // create app and run it
    let tick_rate: Duration = Duration::from_millis(250);
    let app: app::App = app::App::new();
    let res: Result<(), io::Error> = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

