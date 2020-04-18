pub struct ApplicationState {
    pub keep_running: bool,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState { keep_running: true }
    }
}
