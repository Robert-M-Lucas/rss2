use ratatui::widgets::ScrollbarState;
pub struct RScrollbarState {
    pub position: usize,
    pub view_height: usize,
    pub end: usize,
}

impl RScrollbarState {
    pub fn new(end: usize) -> RScrollbarState {
        RScrollbarState {
            position: 0,
            view_height: 1,
            end,
        }
    }

    pub fn down(&mut self) {
        self.position = self.position.saturating_add(1);
    }

    pub fn up(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    pub fn view_height_down(&mut self) {
        self.position = self.position.saturating_add(self.view_height);
    }

    pub fn view_height_up(&mut self) {
        self.position = self.position.saturating_sub(self.view_height);
    }

    pub fn top(&mut self) {
        self.position = 0;
    }

    pub fn bottom(&mut self) {
        self.position = self.end.saturating_sub(self.view_height);
    }
}

impl From<&mut RScrollbarState> for ScrollbarState {
    fn from(state: &mut RScrollbarState) -> ScrollbarState {
        ScrollbarState::new(state.end.saturating_sub(state.view_height)).position(state.position)
    }
}
