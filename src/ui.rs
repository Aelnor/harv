use serde_json::Result;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::{backend::Backend, Frame};

use crate::app::{App, Window};
use crate::model::{Entry, Header};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
        .split(f.size());

    let chunks2 = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
        .split(chunks[1]);

    render_list_widget(f, app, chunks[0]);
    render_request_widget(f, app, chunks2[0]);
    render_response_widget(f, app, chunks2[1]);
}

fn render_list_widget<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let items: Vec<_> = app
        .log
        .list
        .iter()
        .map(|e| {
            let mut res =
                ListItem::new(e.request.url.clone()).style(Style::default().fg(Color::Gray));
            if e.response.status >= 301 && e.response.status < 400 {
                res = res.style(Style::default().fg(Color::Blue));
            } else if e.response.status >= 400 {
                res = res.style(Style::default().fg(Color::Red));
            }

            res
        })
        .collect();

    let style = match app.current_window {
        Window::RequestList => app.style.section_highlight,
        _ => Style::default(),
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title("Requests")
                .borders(Borders::ALL)
                .style(style),
        )
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.log.state);
}

fn render_request_widget<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let style = match app.current_window {
        Window::RequestDetails => app.style.section_highlight,
        _ => Style::default(),
    };
    let par = Paragraph::new(build_request_text(app))
        .block(
            Block::default()
                .title("Request")
                .borders(Borders::ALL)
                .style(style),
        )
        .style(Style::default().fg(Color::Gray))
        .alignment(tui::layout::Alignment::Left)
        .wrap(tui::widgets::Wrap { trim: false })
        .scroll((app.request_vertical_offset, 0));

    f.render_widget(par, area);
}

fn render_response_widget<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let style = match app.current_window {
        Window::ResponseDetails => app.style.section_highlight,
        _ => Style::default(),
    };
    let par = Paragraph::new(build_response_text(app))
        .block(
            Block::default()
                .title("Response")
                .borders(Borders::ALL)
                .style(style),
        )
        .style(Style::default().fg(Color::Gray))
        .alignment(tui::layout::Alignment::Left)
        .wrap(tui::widgets::Wrap { trim: false })
        .scroll((app.response_vertical_offset, 0));

    f.render_widget(par, area);
}

fn build_request_text(app: &App) -> Vec<Spans> {
    let selected = app.log.state.selected();
    if selected.is_none() {
        return Vec::new();
    }

    let e = &app.log.list[selected.unwrap()];

    let mut resp = vec![Spans::from(vec![Span::raw(get_request_summary(e))])];

    add_headers(&mut resp, &e.request.headers, app.show_headers);
    /*
        if !e.request.query_string.is_empty() {
            resp.push(Spans::from(vec![Span::raw("Query String:")]));
        }
        for qs in &e.request.query_string {
            resp.push(Spans::from(vec![Span::raw(format!(
                "{}: {}",
                qs.name, qs.value
            ))]));
        }
    */

    match &e.request.post_data {
        None => {}
        Some(data) => {
            add_raw_text_lines(
                &mut resp,
                data.text.as_ref().unwrap_or(&String::new()).as_str(),
            );
        }
    }

    resp
}

fn build_response_text(app: &App) -> Vec<Spans> {
    let selected = app.log.state.selected();
    if selected.is_none() {
        return Vec::new();
    }

    let e = &app.log.list[selected.unwrap()];

    let mut resp = vec![Spans::from(vec![
        Span::raw(get_response_summary(e)),
        Span::raw(", "),
        Span::raw(get_response_content_type(e)),
    ])];

    add_headers(&mut resp, &e.response.headers, app.show_headers);

    add_raw_text_lines(&mut resp, "");
    add_raw_text_lines(&mut resp, get_response_text(e).as_str());

    resp
}

fn get_request_summary(entry: &Entry) -> String {
    let rq = &entry.request;
    format!("{} {}", rq.method, rq.url)
}

fn get_response_text(entry: &Entry) -> String {
    let t = match &entry.response.content.text {
        None => String::from("no text"),
        Some(t) => t.clone(),
    };

    let appjson = "application/json";

    match entry.response.content.mime_type {
        None => t,
        Some(ref val) => {
            if val == appjson || val.starts_with("application/json;") {
                let obj: Result<serde_json::Value> = serde_json::from_str(&t);
                if obj.is_err() {
                    return format!("Can't Parse JSON!\n{}", t);
                }
                serde_json::to_string_pretty(&obj.unwrap()).unwrap_or(t)
            } else {
                t
            }
        }
    }
}

fn get_response_summary(entry: &Entry) -> String {
    format!("{} {}", entry.response.status, entry.response.status_text)
}

fn get_response_content_type(entry: &Entry) -> String {
    match &entry.response.content.mime_type {
        None => String::from("none"),
        Some(t) => t.clone(),
    }
}

fn add_raw_text_lines(spans: &mut Vec<Spans>, line: &str) {
    let split: Vec<&str> = line.split('\n').collect();

    for s in split {
        spans.push(Spans::from(Span::raw(String::from(s))))
    }
}

fn add_headers(spans: &mut Vec<Spans>, headers: &[Header], show_headers: bool) {
    if show_headers {
        for h in headers {
            spans.push(Spans::from(vec![Span::raw(format!(
                "{}: {}",
                h.name, h.value
            ))]));
        }
    } else {
        spans.push(Spans::from(vec![
            Span::raw("Headers: "),
            Span::styled(
                format!("({}, hidden)", headers.len()),
                Style::default().add_modifier(Modifier::ITALIC),
            ),
        ]));
    }
}
