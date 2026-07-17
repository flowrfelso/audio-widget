// #[derive(Serialize, Deserialize)]
// pub struct Settings {
//     pub always_on_top: bool,

//     pub launch_at_startup: bool,

//     pub auto_hide: bool,

//     pub theme: String,
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub always_on_top: bool,
    pub launch_at_startup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            always_on_top: false,
            launch_at_startup: false,
        }
    }
}
