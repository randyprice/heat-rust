use std::{
    error::Error,
    io,
};

use crossterm::{
    execute,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use tui::{
    Frame,
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},

};

struct App {
    pub hand_index: usize,
    pub hand_state: ListState,
    pub max_cards_in_hand: usize,
}

impl App {
    // ---------------------------------------------------------------------------------------------
    fn new() -> App {
        App {
            hand_index: 0,
            hand_state: ListState::default(),
            max_cards_in_hand: 7,
        }
    }

    // ---------------------------------------------------------------------------------------------
    fn hand_next(&mut self) {
        let idx = if let Some(num) = self.hand_state.selected() {
            if num >= self.max_cards_in_hand {
                0
            } else {
                num + 1
            }
        } else {
            0
        };
        self.hand_state.select(Some(idx));
    }

    // ---------------------------------------------------------------------------------------------
    fn hand_previous(&mut self) {
        let idx = if let Some(num) = self.hand_state.selected() {
            if num == 0 {
                self.max_cards_in_hand - 1
            } else {
                num - 1
            }
        } else {
            0
        };
        self.hand_state.select(Some(idx));
    }
}

// -------------------------------------------------------------------------------------------------
fn draw_ui<T: Backend>(frame: &mut Frame<T>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(frame.size());
    let items: Vec<ListItem> = vec![1, 2, 3, 4, 5, 6, 7]
        .iter()
        .map(|item| {
            ListItem::new(item.to_string()).style(Style::default().fg(Color::White).bg(Color::Black))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Cards"))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED),
        );
    frame.render_stateful_widget(list, chunks[1], &mut app.hand_state);
}

// -------------------------------------------------------------------------------------------------
fn run_app_loop<T: Backend>(terminal: &mut Terminal<T>, mut app: App) -> io::Result<()>{
    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => app.hand_previous(),
                KeyCode::Right => app.hand_next(),
                KeyCode::Char('q') => break,
                _ => (),
            }
        }
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------
pub fn run() -> Result<(), Box<dyn Error>>{
    // Initialize terminal.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application.
    let app = App::new();
    let result = run_app_loop(&mut terminal, app);

    // Restore terminal.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}