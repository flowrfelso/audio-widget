use std::sync::Mutex;

pub struct WindowState {
    pub pinned: Mutex<bool>,
}
