
trait XMLParser {
    fn begin(elem:Element);
    fn end(elem:Element);
}


struct RustParser<'a> {
    depth : usize,
    style : &'a StyleSheet<'a>,
    parsed : String
}

impl <'a> RSParser<'a> {
    pub fn new(css:&'a StyleSheet) {
        Self {
            depth : 0,
            style : css,
            parsed : String::new()
        }
    }

    pub fn get_parsed(&self) -> &str {
        self.parsed.as_str()
    }
}

impl XMLParser for RSParser {
    fn begin(elem:Element) {

    }
}