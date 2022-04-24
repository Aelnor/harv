use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::{backend::Backend, Frame};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
        .split(f.size());

    let items: Vec<_> = app
        .log
        .list
        .iter()
        .map(|e| ListItem::new(e.request.url.clone()))
        .collect();
    let list = List::new(items)
        .block(Block::default().title("Requests").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, chunks[0], &mut app.log.state);
    render_details_widget(f, app, chunks[1]);
}

fn render_details_widget<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let par = Paragraph::new(build_details_text(app))
        .block(Block::default().title("Entry").borders(Borders::ALL))
        .style(Style::default())
        .alignment(tui::layout::Alignment::Left)
        .wrap(tui::widgets::Wrap { trim: false });

    f.render_widget(par, area);
}

fn build_details_text(app: &App) -> Vec<Spans> {
    vec![
        Spans::from(vec![Span::raw(get_request_summary(app))]),
        Spans::from(vec![Span::raw(get_response_text(app))]),
    ]
}

fn get_request_summary(app: &App) -> String {
    let selected = app.log.state.selected();
    if selected.is_none() {
        return String::new();
    }
    let selected = selected.unwrap();

    let rq = &app.log.list[selected].request;
    return rq.method.clone() + " " + &rq.url;
}

fn get_response_text(app: &App) -> String {
    let selected = app.log.state.selected();
    if selected.is_none() {
        return String::new();
    }
    let selected = selected.unwrap();
    match &app.log.list[selected].response.content.text {
        None => return String::from("no text"),
        Some(t) => return t.clone(),
    };
}
