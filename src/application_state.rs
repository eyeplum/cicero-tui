pub struct ApplicationState {
    pub keep_running: bool,
    pub user_input: String,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            keep_running: true,
            user_input: String::default(),
        }
    }
}
