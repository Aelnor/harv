use tui::style::{Color, Style};

use crate::{model::Entry, util::StatefulList};

pub enum Window {
    RequestList,
    RequestDetails,
    ResponseDetails,
}

pub struct App {
    pub log: StatefulList<Entry>,
    pub show_headers: bool,
    pub style: StyleSettings,
    pub current_window: Window,
    pub request_vertical_offset: u16,
    pub response_vertical_offset: u16,
}

pub struct StyleSettings {
    pub section_highlight: Style,
}

impl App {
    pub fn new(path: &str) -> Self {
        let har = har::from_path(path).expect("can open file");
        let mut app = App {
            log: StatefulList::new(),
            show_headers: false,
            style: StyleSettings {
                section_highlight: Style::default().fg(Color::White),
            },
            current_window: Window::RequestList,
            request_vertical_offset: 0,
            response_vertical_offset: 0,
        };

        match har.log {
            har::Spec::V1_2(log) => {
                let items = log.entries.iter().map(|e| Entry::from(e)).collect();
                app.log = StatefulList::with_items(items);
            }
            har::Spec::V1_3(_) => {}
        }
        app.fill_request_list();
        app
    }

    pub fn toggle_headers(&mut self) {
        self.show_headers = !self.show_headers;
    }

    pub fn next_entry(&mut self) {
        self.log.next();
        self.request_vertical_offset = 0;
        self.response_vertical_offset = 0;
    }

    pub fn previous_entry(&mut self) {
        self.log.prev();
        self.request_vertical_offset = 0;
        self.response_vertical_offset = 0;
    }

    pub fn down(&mut self) {
        match self.current_window {
            Window::RequestList => self.next_entry(),
            Window::RequestDetails => self.request_vertical_offset += 1,
            Window::ResponseDetails => self.response_vertical_offset += 1,
        }
    }
    pub fn up(&mut self) {
        match self.current_window {
            Window::RequestList => self.previous_entry(),
            Window::RequestDetails => {
                if self.request_vertical_offset != 0 {
                    self.request_vertical_offset -= 1
                }
            }
            Window::ResponseDetails => {
                if self.response_vertical_offset != 0 {
                    self.response_vertical_offset -= 1
                }
            }
        }
    }

    fn fill_request_list(&mut self) {
        self.log.state.select(Some(0));
    }

    pub fn next_window(&mut self) {
        self.current_window = match self.current_window {
            Window::RequestList => Window::RequestDetails,
            Window::RequestDetails => Window::ResponseDetails,
            Window::ResponseDetails => Window::RequestList,
        };
    }
}
