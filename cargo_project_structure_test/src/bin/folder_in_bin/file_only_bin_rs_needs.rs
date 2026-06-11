pub struct Cat {
    pub opinion: String,
}

impl Cat {
    pub fn new() -> Self {
        Self {
            opinion: "they suck".to_string(),
        }
    }
}
