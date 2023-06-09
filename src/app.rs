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
    widgets::{ListItem, Block, List, Borders, BorderType, Paragraph, Wrap}, 
    text::{Spans, Span}, style::{Style, Color, Modifier}, 
    layout::{Alignment, Layout, Direction, Constraint}
};

use crate::datatypes::datatypes::{Brews};

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App {
    brews: Brews,
    scroll: u16
}

impl App {
    pub fn new() -> App {
        App {
            brews: Brews::load_brews_from_file(),
            scroll: 0
        }
    }

    fn scroll(&mut self, direction: i8) {
        if (direction == -1) & (self.scroll > 0) {
            self.scroll -= 1;
        }
        if (direction == 1) & (self.scroll < 9) {
            self.scroll += 1;
        }
        self.scroll %= 10;
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
                    KeyCode::Left => app.brews.unselect(),
                    KeyCode::Down => app.brews.next(),
                    KeyCode::Up => app.brews.previous(),
                    KeyCode::PageDown => app.scroll(1),
                    KeyCode::PageUp => app.scroll(-1),
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
    .brews
    .brews
    .iter()
    .map(|i| {
        let lines = vec![Spans::from(i.name.clone())];
        ListItem::new(lines).style(Style::default())
    })
    .collect();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Brewtracker-rs v0.2.2")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
    .block(Block::default()
        .title(Span::styled("My Brews",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
        ))
        .title_alignment(Alignment::Center)
    )
    .highlight_style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");
    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.brews.state);

    // Top right inner block with styled title aligned to the right
    let block = Block::default()
        .title(Span::styled(
            "Information",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        ))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    
    let selected_index = app.brews.state.selected();
    

    let mut text: Vec<Spans> = Vec::new(); 
    match selected_index {
        Some(x) => generate_details(&mut text, x, app),
        None => text.push(Spans::from("Nothing Selected"))
    }

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default())
        .block(
            block
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0));;
    f.render_widget(paragraph, chunks[1]);
}

fn generate_details(text: &mut Vec<Spans>, index: usize, app: &mut App) {
    text.push(Spans::from(generate_details_star_rating(app.brews.brews[index].rating)));
    text.push(Spans::from(""));
    text.push(Spans::from("Description"));
    text.push(Spans::from(app.brews.brews[index].description.clone()));
    text.push(Spans::from(""));
    text.push(Spans::from("Ingredients"));
    generate_details_list(text, &app.brews.brews[index].ingredients, false);
    text.push(Spans::from(""));
    text.push(Spans::from("Method"));
    generate_details_list(text, &app.brews.brews[index].method, true);
    text.push(Spans::from(""));
    text.push(Spans::from(format!("Initial Gravity: {}", app.brews.brews[index].gravity[0])));
    text.push(Spans::from(format!("Final Gravity: {}", app.brews.brews[index].gravity[1])));
}

fn generate_details_star_rating(rating: u8) -> String {
    let mut stars: String = "Rating: ".to_string();
    for _i in 0..rating {
        stars.push('★');
    }
    for _i in 0..(5-rating) {
        stars.push('☆');
    }
    return stars;
}

fn generate_details_list(text: &mut Vec<Spans>, ingredients: &Vec<String>, ordered: bool) {
    let mut ordered_value = 1;
    for ingredient in ingredients.iter() {
        let mut data = "".to_string();
        if ordered {
            data.push_str(&ordered_value.to_string());
            data.push_str(". ");
        } else {
            data.push_str("⬤ ");
        }
        data.push_str(ingredient);
        text.push(Spans::from(data));
        ordered_value += 1;
    }
}