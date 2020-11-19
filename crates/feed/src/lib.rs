// pub trait StreamValue: Clone + Default + Copy {}

#[derive(Default)]
pub struct Stream {
    pub name: String,
    pub source: Vec<f64>,
    pub value: f64,

    size: usize,
    cursor: usize,
}

impl Stream {
    pub fn new(name: &str, source: &Vec<f64>) -> Self {
        Stream {
            name: name.to_string(),
            source: source.clone(),
            ..Default::default()
        }
    }

    pub fn rename(mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn forward(mut self) -> f64 {
        if self.cursor < self.size {
            panic!("stream exceeded end of array");
        }

        let v = self.source[self.cursor];
        self.cursor += 1;
        v
    }

    pub fn has_next(self) -> bool {
        self.cursor < self.size
    }

    pub fn reset(mut self) {
        self.size = self.source.len();
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //TODO: impl
    }
}
