use app::App;
use crossterm::event;
use crossterm::event::Event as CEvent;
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod model;
mod ui;
mod util;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Expected exactly one command line argument: the file name");
        return Ok(());
    }

    let mut app = App::new(args[1].as_str());
    enable_raw_mode().expect("can use raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    terminal.clear()?;
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('j') | KeyCode::Down => app.down(),
                KeyCode::Char('k') | KeyCode::Up => app.up(),
                KeyCode::Char('h') => app.toggle_headers(),
                KeyCode::Tab => app.next_window(),
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
