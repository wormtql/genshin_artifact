use std::any::Any;

pub trait OutputStream {
    fn append_str(&mut self, s: &str);

    fn as_any(&self) -> &dyn Any;
}

pub struct PrintOutputStream;

impl OutputStream for PrintOutputStream {
    fn append_str(&mut self, s: &str) {
        println!("{}", s);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StringOutputStream {
    value: String
}

impl OutputStream for StringOutputStream {
    fn append_str(&mut self, s: &str) {
        self.value.push_str(s);
        self.value.push_str("\n");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StringOutputStream {
    pub fn new() -> Self {
        StringOutputStream {
            value: String::new()
        }
    }

    pub fn get_string(&self) -> String {
        // println!("{}", self.value);
        self.value.clone()
    }
}
