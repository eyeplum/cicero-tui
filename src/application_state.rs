use crate::InputMode;

pub struct ApplicationState {
    pub keep_running: bool,
    pub active_input_mode: InputMode,
    pub user_input: String,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            keep_running: true,
            active_input_mode: InputMode::Normal,
            user_input: String::default(),
        }
    }
}
