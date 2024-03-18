use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Account {}

impl Account {
    pub fn new() -> Self {
        Self {}
    }
}

