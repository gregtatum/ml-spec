use text_embedding::{process_wiki_text, read_lines, SymbolIndex, SymbolTable};

/// Ensure the vocab has some common punctuation. This should be more complete, but
/// it gets the basic idea across.
fn add_punctuation_symbols(symbol_table: &mut SymbolTable) {
    symbol_table.index(" ");
    symbol_table.index(".");
    symbol_table.index("!");
    symbol_table.index(",");

    symbol_table.index("1");
    symbol_table.index("2");
    symbol_table.index("3");
    symbol_table.index("4");
    symbol_table.index("5");
    symbol_table.index("6");
    symbol_table.index("7");
    symbol_table.index("8");
    symbol_table.index("9");
}

fn build_symbol_table(path: &str) -> SymbolTable {
    let mut symbol_table = SymbolTable::new();

    add_punctuation_symbols(&mut symbol_table);

    for line in read_lines(path) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(word) = parts.get(1) {
            symbol_table.index(*word);
        }
    }

    symbol_table
}

fn tokenize(symbol_table: &SymbolTable, text: &str) -> Vec<SymbolIndex> {
    let mut symbols: Vec<SymbolIndex> = Vec::new();
    let mut start_index = 0;

    // Start at the max symbol size, and work backwards to find the longest symbol
    // that's in the symbol table.
    let max_symbol_len = symbol_table.max_symbol_len();

    loop {
        let end_search_index = (start_index + max_symbol_len).min(text.len());
        if end_search_index == start_index {
            break;
        }
        let prev_start_index = start_index;
        // Search backwards.
        // "The quick brown fox"
        //      ^------^    "quick br"
        //      ^-----^     "quick r"
        //      ^----^      "quick "
        //      ^---^       "quick"
        for offset in 0..(end_search_index - start_index) {
            let end_index = end_search_index - offset;
            let slice = &text[start_index..end_index];
            if let Some(index) = symbol_table.maybe_index(slice) {
                symbols.push(index);
                start_index = end_index;
                break;
            }
        }
        if prev_start_index == start_index {
            start_index += 1;
        }
    }

    symbols
}

fn main() {
    println!("Building the string table:");
    let symbol_table = build_symbol_table("./data/text/en-dictionary-symbols.txt");

    println!("Symbolizing the wiki text:");
    let mut i = 1;
    process_wiki_text("./data/text/wiki-en.txt", |_, text| {
        if i > 100000 {
            return false;
        }
        print!("Processing line {i}: ");
        let text = text.to_lowercase();

        let symbols = tokenize(&symbol_table, &text);

        let mut character_count = 0;
        for index in &symbols {
            let symbol = symbol_table.symbol(*index);
            character_count += symbol.len();
            if character_count > 80 {
                break;
            }
            print!("{}", symbol);
        }
        println!();

        i += 1;
        true
    });
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_symbol_table() -> SymbolTable {
        let words = vec![
            "the", "quick", "br", "own", "fox", "jump", "s", "over", "the", "la", "zy", "dog", "a",
            "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
            "s", "t", "u", "v", "w", "x", "y", "z",
        ];
        let mut symbol_table = SymbolTable::new();
        for word in &words {
            symbol_table.index(*word);
        }
        add_punctuation_symbols(&mut symbol_table);
        symbol_table
    }

    fn build_test(text: &str) -> Vec<String> {
        let symbol_table = build_symbol_table();
        let text = text.to_lowercase();
        let symbols = tokenize(&symbol_table, &text);
        symbols
            .iter()
            .map(|index| symbol_table.symbol(*index).to_string())
            .collect()
    }

    #[test]
    fn test_convert_to_symbols() {
        assert_eq!(
            build_test("The quick brown fox jumps over the lazy dog"),
            vec![
                "the", " ", "quick", " ", "br", "own", " ", "fox", " ", "jump", "s", " ", "over",
                " ", "the", " ", "la", "zy", " ", "dog"
            ]
        );
        assert_eq!(
            build_test("No symbols match here"),
            vec![
                "n", "o", " ", "s", "y", "m", "b", "o", "l", "s", " ", "m", "a", "t", "c", "h",
                " ", "h", "e", "r", "e"
            ]
        );
        assert_eq!(
            build_test("The (quick) brown fox."),
            vec!["the", " ", "quick", " ", "br", "own", " ", "fox", "."]
        );
    }
}
