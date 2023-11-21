/// Application.
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub score: u8,
    pub board_state: Vec<u8>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_score(&mut self, increase: u8) {
        if let Some(res) = self.score.checked_add(increase) {
            self.score = res;
        }
    }
    pub fn shift_left(&mut self) {}
    pub fn shift_right(&mut self) {}
    pub fn shift_up(&mut self) {}
    pub fn shift_down(&mut self) {}
}
