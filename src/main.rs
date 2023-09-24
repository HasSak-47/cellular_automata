mod cellular;
mod terminal;

use terminal::MakeBufferCell;
use cellular::foggy; 

use std::{time::Duration, io};
use tui::{
    backend::CrosstermBackend,
    Terminal
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


fn main() -> Result<(), io::Error> {
    let mut board : foggy::Board<foggy::LifeCell, 32, 32> = foggy::Board::random();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let start = std::time::SystemTime::now();
    loop{
        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(&board, size);
        })?;

        if start.elapsed().unwrap().as_secs() > 10 {
            break;
        }
        std::thread::sleep(Duration::from_millis(300));
        board.update();
    }


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;


    Ok(())
}
