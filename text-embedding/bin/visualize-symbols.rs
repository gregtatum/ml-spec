use colored::*;
use text_embedding::{process_wiki_text, read_lines, SymbolIndex, SymbolTable};

/// Ensure the vocab has some common punctuation. This should be more complete, but
/// it gets the basic idea across.
fn add_punctuation_symbols(symbol_table: &mut SymbolTable) {
    symbol_table.index(" ");
    symbol_table.index(".");
    symbol_table.index("!");
    symbol_table.index(",");
    symbol_table.index("\"");
    symbol_table.index("'");

    symbol_table.index("1");
    symbol_table.index("2");
    symbol_table.index("3");
    symbol_table.index("4");
    symbol_table.index("5");
    symbol_table.index("6");
    symbol_table.index("7");
    symbol_table.index("8");
    symbol_table.index("9");

    symbol_table.remap_symbol("\t", " ");
    symbol_table.remap_symbol("\n", " ");
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

fn is_whitespace(slice: &[u8]) -> bool {
    slice.len() == 1 && slice[0] == b' '
}

fn is_valid_word_byte(byte: u8) -> bool {
    byte < 0b1000_0000 && byte >= b'a' && byte <= b'z'
}

fn symbolize(symbol_table: &SymbolTable, text: &str) -> Vec<SymbolIndex> {
    let mut symbols: Vec<SymbolIndex> = Vec::new();
    let mut start_index = 0;

    // Start at the max symbol size, and work backwards to find the longest symbol
    // that's in the symbol table.
    let max_symbol_len = symbol_table.max_symbol_len();

    let text = text.as_bytes();
    let mut prev_is_whitespace = false;

    loop {
        // By default search ahead the max symbol length.
        let mut end_search_index = start_index + max_symbol_len;
        if start_index >= text.len() {
            break;
        }

        if is_valid_word_byte(text[start_index]) {
            // This is a word, find the word boundary.
            for index in start_index + 1..start_index + max_symbol_len {
                if index >= text.len() {
                    break;
                }
                if !is_valid_word_byte(text[index]) {
                    end_search_index = index;
                    break;
                }
            }
        } else {
            // This is not a word.
            end_search_index = start_index + 1
        }
        // Keep the index in bounds.
        end_search_index = end_search_index.min(text.len());
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
            let index = unsafe {
                // The slice may be invalid UTF-8, but it is only used for looking up
                // symbols, so it's OK to unsafely treat it as valid UTF-8 here.
                symbol_table.maybe_index(std::str::from_utf8_unchecked(&slice))
            };
            if let Some(index) = index {
                start_index = end_index;

                // Do not add two concurrent pieces of whitespace.
                if is_whitespace(slice) {
                    if prev_is_whitespace {
                        break;
                    }
                    prev_is_whitespace = true;
                } else {
                    prev_is_whitespace = false;
                }

                // The symbol was found, add it.
                symbols.push(index);
                break;
            }
        }

        if prev_start_index == start_index {
            // This is an unknown character, such as "(" or an emoji.
            start_index += 1;
        }
    }

    symbols
}

fn main() {
    println!("Building the string table:");
    let symbol_table = build_symbol_table("./data/text/en-dictionary-symbols.txt");
    let whitespace = symbol_table
        .maybe_index(" ")
        .expect("Could not find whitespace");

    println!("Symbolizing the wiki text:");
    let mut i = 1;
    process_wiki_text("./data/text/wiki-en.txt", |id, text| {
        print!("{id}: ");
        let text = text.to_lowercase();

        let symbols = symbolize(&symbol_table, &text);

        let mut character_count = 0;
        let mut flip_flop = true;
        for index in &symbols {
            let symbol = symbol_table.symbol(*index);
            character_count += symbol.len();
            if character_count > 80 {
                break;
            }
            if *index == whitespace {
                print!("{}", symbol);
            } else {
                flip_flop = !flip_flop;
                if flip_flop {
                    print!("{}", symbol.cyan());
                } else {
                    print!("{}", symbol.magenta());
                }
            }
        }
        println!();

        i += 1;
        true
    });

    println!("");
    println!("This utility visualizes symbols. The number is the ID of the text.");
    println!("Each symbol in the vocab is displayed in either cyan or magenta.");
    println!("Common words should be all one color, and uncommon words should");
    println!("be composed of multiple symbol parts.");
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
        let symbols = symbolize(&symbol_table, &text);
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
            ],
            "Basic symbolizing works."
        );
        assert_eq!(
            build_test("No symbols match here"),
            vec![
                "n", "o", " ", "s", "y", "m", "b", "o", "l", "s", " ", "m", "a", "t", "c", "h",
                " ", "h", "e", "r", "e"
            ],
            "No symbol matches breaks down to individual letters."
        );
        assert_eq!(
            build_test("The (quick) brown🦊 fox."),
            vec!["the", " ", "quick", " ", "br", "own", " ", "fox", "."],
            "Unknown characters are ignored."
        );

        assert_eq!(
            build_test("The   quick    brown fox  "),
            vec!["the", " ", "quick", " ", "br", "own", " ", "fox", " "],
            "Spaces are combined together."
        );
        assert_eq!(
            build_test("The\tquick\tbrown\nfox"),
            vec!["the", " ", "quick", " ", "br", "own", " ", "fox"],
            "Some symbols are remapped"
        );
        assert_eq!(
            build_test("Thé quíck brøwn fox"),
            vec!["t", "h", " ", "q", "u", "c", "k", " ", "br", "w", "n", " ", "fox"],
            "Diacritical marks are not currently suppported"
        );
    }
}
