use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub type StringIndex = usize;

/// Stores a unique list of strings, so that strings can be operated up via stable
/// indexes. This makes for cheap comparisons and storage of references to strings.
#[derive(Debug)]
pub struct StringTable {
    strings: Vec<String>,
}

impl StringTable {
    /// Create a new StringTable.
    pub fn new() -> StringTable {
        StringTable {
            strings: Vec::new(),
        }
    }

    /// Interns a string if it exists, and returns the index. Otherwise it discards the
    /// string and returns an existing index. O(n)
    pub fn index<T: Into<String> + AsRef<str>>(&mut self, string: T) -> StringIndex {
        if let Some(index) = self.maybe_index(string.as_ref()) {
            return index;
        }
        let index = self.strings.len();
        self.strings.push(string.into());
        index
    }

    /// Gets an index for a string if it exists.
    pub fn maybe_index<T: AsRef<str>>(&self, string: T) -> Option<StringIndex> {
        self.strings.iter().position(|s| s == string.as_ref())
    }

    /// Returns a string from an index.
    pub fn string(&self, index: StringIndex) -> &str {
        match self.strings.get(index) {
            Some(string) => &**string,
            None => "",
        }
    }
}

pub fn read_lines(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap())
}

/// Process the wikipedia text
pub fn process_wiki_text(file: &str, mut callback: impl FnMut(u32, &str) -> bool) {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut id_string = String::new();
    for raw_line in line_iter(reader) {
        let raw_line = raw_line.expect("Unable to get next line from the text.");

        // Start processing the string line
        // "@@1514 Albert of Prussia..."
        //  ^^
        if !raw_line.starts_with("@@") {
            println!("{}", raw_line);
            panic!("Line does not start with @@");
        }

        // "1514 Albert of Prussia..."
        let raw_line = &raw_line[2..];

        // Extract the number part of the string.
        // "@@1514 Albert of Prussia..."
        //    ^^^^
        id_string.clear();
        for ch in raw_line.chars() {
            if ch.is_ascii_digit() {
                id_string.push(ch);
            } else {
                break;
            }
        }
        let id = id_string.parse::<u32>().expect("Could not parse a string");

        // Skip the space.
        // "@@1514 Albert of Prussia..."
        //        ^
        if raw_line.as_bytes()[id_string.len()] != ' ' as u8 {
            panic!("Expected a space after the id");
        }
        // "Albert of Prussia..."
        let text = &raw_line[(id_string.len() + 1)..];
        if callback(id, text) == false {
            return;
        };
    }
}

/// Skips the preamble, and returns an iterator over the lines.
fn line_iter(reader: BufReader<File>) -> impl Iterator<Item = std::io::Result<String>> {
    let mut lines = reader.lines().peekable();

    while let Some(Ok(line)) = lines.peek() {
        if line.starts_with("@@") {
            break;
        } else {
            lines.next();
        }
    }
    lines.into_iter()
}
