pub struct Words {
    pub words: Vec<String>,
    pub filename: String,
    pub raw_content: String,
}

impl Words {
    pub fn new(path: String) -> Self {
        let file = std::fs::read_to_string(path.clone()).unwrap();
        Self {
            raw_content: file,
            filename: path,
            words: Vec::new(),
        }
    }
    pub fn sort_words(&mut self) {
        let words: Vec<&str> = self.raw_content.split("\r\n").collect();
        let mut returnvec: Vec<String> = Vec::new();
        for i in words {
            returnvec.push(i.to_string());
        }
        self.words = returnvec;
    }
}
