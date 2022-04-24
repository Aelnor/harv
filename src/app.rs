use crate::{model::Entry, util::StatefulList};

pub struct App {
    pub log: StatefulList<Entry>,
}

impl App {
    pub fn new(path: &str) -> Self {
        let har = har::from_path(path).expect("can open file");
        let mut app = App {
            log: StatefulList::new(),
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

    pub fn next_entry(&mut self) {
        self.log.next();
    }

    pub fn previous_entry(&mut self) {
        self.log.prev();
    }
    fn fill_request_list(&mut self) {
        self.log.state.select(Some(0));
    }
}
