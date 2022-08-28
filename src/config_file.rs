use std::path::PathBuf;

use crate::reminder::{Color, LocalReminder, Priority, Reminder};
// use serde::{Serialize, Deserialize};
use serde_derive::{Deserialize, Serialize};

pub(crate) struct ConfigSum {
    pub(crate) priorities: Vec<Priority>,
    pub(crate) reminders: Vec<LocalReminder>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigFile {
    pub(crate) config_paths: Vec<String>,
    //
    pub(crate) priorities: Vec<Priority>,
    pub(crate) reminders: Vec<Reminder>,
    //
    pub(crate) settings: Settings,
    //
    #[serde(skip_serializing)]
    pub(crate) path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Settings {
    pub(crate) name: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) email: Option<String>,
}

impl ConfigFile {
    pub(crate) fn default_global_config() -> Self {
        Self {
            config_paths: vec![],
            priorities: vec![Priority {
                name: String::from("Critical"),
                color: Color::Red,
                id: String::from("0"),
            }],
            reminders: vec![Reminder {
                priority: String::from("Critical"),
                author: String::from("John Doe, johndoe, johndoe@gmail.com"),
                text: String::from("This is a global critical reminder!"),
            }],
            settings: Settings {
                name: None,
                username: None,
                email: None,
            },
            path: None,
        }
    }

    pub(crate) fn default_local_config() -> ConfigFile {
        ConfigFile {
            config_paths: vec![],
            priorities: vec![],
            reminders: vec![Reminder {
                priority: "Critical".to_string(),
                author: String::from("John Doe, johndoe, johndoe@gmail.com"),
                text: String::from("This is a local critical reminder!"),
            }],
            settings: Settings {
                name: None,
                username: None,
                email: None,
            },
            path: None,
        }
    }
}

impl ConfigSum {
    pub(crate) fn new() -> Self {
        Self {
            priorities: vec![],
            reminders: vec![],
        }
    }
}
