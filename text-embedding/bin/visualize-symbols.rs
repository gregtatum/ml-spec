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

fn convert_to_symbols(symbol_table: &SymbolTable, text: &str) -> Vec<SymbolIndex> {
    let mut symbols: Vec<SymbolIndex> = Vec::new();
    let mut start_index = 0;
    let mut end_index = 1;
    let mut prev_symbol: Option<SymbolIndex> = None;

    loop {
        let slice = &text[start_index..end_index];
        println!("slice {:?}", slice);
        if let Some(index) = symbol_table.maybe_index(slice) {
            end_index += 1;
            if end_index > text.len() {
                // This is the last symbol.
                println!("last symbol {:?}", slice);
                symbols.push(index);
                break;
            } else {
                prev_symbol = Some(index);
                continue;
            }
        }
        // There was no match for the symbol.
        if let Some(index) = prev_symbol {
            println!(
                "No match, adding previous symbol {:?}",
                symbol_table.symbol(index)
            );
            symbols.push(index);
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
    let symbol_table = build_symbol_table("./data/text/en-dictionary-symbols.txt");

    println!("Processing the wiki text:");
    let mut i = 0;
    process_wiki_text("./data/text/wiki-en.txt", |_, text| {
        println!("Processing line {i}:");
        if i == 10 {
            return false;
        }
        let text = text.to_lowercase();

        let symbols = convert_to_symbols(&symbol_table, &text);

        for symbol in &symbols[0..100] {
            print!("{}", symbol_table.symbol(*symbol));
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
        let symbols = convert_to_symbols(&symbol_table, &text);
        symbols
            .iter()
            .map(|index| symbol_table.symbol(*index).to_string())
            .collect()
    }

    #[test]
    fn test_convert_to_symbols() {
        assert_eq!(build_test("The"), vec!["The"]);
    }
}
