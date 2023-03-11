use text_embedding::{process_wiki_text, read_lines, StringTable};

type StringIndex = usize;

fn build_string_table(path: &str) -> StringTable {
    let mut string_table = StringTable::new();

    // Ensure the vocab has some common punctuation. This should be more complete, but
    // it gets the basic idea across.
    string_table.index(" ");
    string_table.index(".");
    string_table.index("!");
    string_table.index(",");

    string_table.index("1");
    string_table.index("2");
    string_table.index("3");
    string_table.index("4");
    string_table.index("5");
    string_table.index("6");
    string_table.index("7");
    string_table.index("8");
    string_table.index("9");

    for line in read_lines(path) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(word) = parts.get(1) {
            string_table.index(*word);
        }
    }

    string_table
}

fn add_symbols(string_table: &StringTable, text: &str) -> Vec<StringIndex> {
    let mut symbols: Vec<StringIndex> = Vec::new();
    let mut start_index = 0;
    let mut end_index = 1;
    let mut prev_symbol: Option<StringIndex> = None;

    loop {
        let slice = &text[start_index..end_index];
        if let Some(index) = string_table.maybe_index(slice) {
            end_index += 1;
            if end_index > text.len() {
                // This is the last symbol.
                symbols.push(index);
                break;
            } else {
                prev_symbol = Some(index);
                continue;
            }
        }
        // There was no match for the symbol.
        if let Some(prev_symbol) = prev_symbol {
            symbols.push(prev_symbol);
        }
        if end_index - start_index == 1 {
            // This is an unknown symbols
            end_index += 1;
        }
        prev_symbol = None;
        start_index = end_index - 1;
        // end_index += 1;
    }

    symbols
}

fn main() {
    println!("Building the string table:");
    let string_table = build_string_table("./data/text/en-dictionary-symbols.txt");

    println!("Processing the wiki text:");
    let mut i = 0;
    process_wiki_text("./data/text/wiki-en.txt", |_, text| {
        println!("Processing line {i}:");
        if i == 10 {
            return false;
        }
        let text = text.to_lowercase();

        let symbols = add_symbols(&string_table, &text);

        for symbol in &symbols[0..100] {
            print!("{}", string_table.string(*symbol));
        }
        println!();

        i += 1;
        true
    });
}
