mod cellular;
mod terminal;

use terminal::MakeBufferCell;
use cellular::*;

use std::{time::Duration, io};
use tui::{
    backend::CrosstermBackend,
    widgets::Widget,
    Terminal
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

impl<T, const WIDTH: usize, const HEIGHT: usize> Widget for &Board<T,WIDTH, HEIGHT>
where
    T: Copy + Default + cellular::Cell + terminal::MakeBufferCell,
{
    fn render(self, r: tui::layout::Rect, b: &mut tui::buffer::Buffer){

        let min_i = r.width.min(self.width() as u16);
        let min_j = r.height.min(self.height() as u16);


        for i in 0..min_i{
            for j in 0..min_j{
                let x = i + r.x;
                let y = j + r.y;

                let new_cell = self.get(x as isize, y as isize).make_buffer_cell();
                *b.get_mut(2 * x + 0, y) = new_cell.clone();
                *b.get_mut(2 * x + 1, y) = new_cell.clone();

            }
        }

    }
}

fn main() -> Result<(), io::Error> {
    let mut board : Board<cellular::LifeCell, 32, 32> = Board::random();

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
