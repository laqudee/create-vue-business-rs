pub mod dialoguers;
pub mod render;
pub mod utils;

#[derive(Debug)]
pub struct ConfiguresSelected {
    pub target_tag: String,
}

impl ConfiguresSelected {
    pub fn new() -> Self {
        Self {
            target_tag: String::from("web"),
        }
    }

    pub fn set_target_tag(&mut self, value: &str) {
        self.target_tag = value.to_string();
    }
}

impl Default for ConfiguresSelected {
    fn default() -> Self {
        Self::new()
    }
}
