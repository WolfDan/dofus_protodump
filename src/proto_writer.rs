pub struct ProtoWriter {
    pub indent: usize,
    result: String,
}

impl ProtoWriter {
    pub fn new() -> Self {
        Self {
            indent: 0,
            result: String::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.result.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.result.push_str(string);
    }

    pub fn push_str_indented(&mut self, string: &str) {
        self.result
            .push_str(&format!("{:indent$}{}", "", string, indent = self.indent));
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn deindent(&mut self) {
        self.indent -= 1;
    }

    pub fn result(&mut self) -> String {
        self.result.clone()
    }
}
