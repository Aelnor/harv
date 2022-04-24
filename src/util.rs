use tui::widgets::ListState;
pub struct StatefulList<T> {
    pub list: Vec<T>,
    pub state: ListState,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            list: Vec::new(),
            state: ListState::default(),
        }
    }
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            list: items,
            state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i != 0 {
                    i - 1
                } else {
                    0
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
