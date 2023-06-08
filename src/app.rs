use std::{
    time::{Duration, Instant}, 
    io
};

use crossterm::event::{Event, 
    self, 
    KeyCode
};
use tui::{backend::Backend, 
    Terminal, 
    Frame, 
    widgets::{ListItem, Block, List, Borders, BorderType}, 
    text::{Spans, Span}, style::{Style, Color, Modifier}, 
    layout::{Alignment, Layout, Direction, Constraint}
};

use crate::datatypes::datatypes::StatefulList;

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App<'a> {
    items: StatefulList<(&'a str, usize)>
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
    .items
    .items
    .iter()
    .map(|i| {
        let lines = vec![Spans::from(i.0)];
        ListItem::new(lines).style(Style::default())
    })
    .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default()
            .borders(Borders::RIGHT)
            .title("My Brews")
            .title_alignment(Alignment::Center)
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Brewtracker-rs v0.1.0")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    // Top right inner block with styled title aligned to the right
    let block = Block::default()
        .title(Span::styled(
            "Information",
            Style::default()
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(block, chunks[1]);
}