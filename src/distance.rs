#![allow(dead_code, non_snake_case)]

/// Compute the levenstein distance between two numbers, also known as the edit distance.
///
/// Edit types:
///   Insertions, deletions, substitutes
///
/// Algorithmic Complexity:
///   O(m*n) time because we fill an (m+1) by (n+1) DP table.
///
///   Memory space is O(m*n) because we store the full table to reuse
///   neighbors when computing each entry, plus O(m+n) to materialize the input
///   strings as `Vec<char>` for indexing.
///
/// Note:
///   DP (dynamic programming) means we solve larger subproblems by combining
///   already-solved smaller ones; here each cell depends on its left, top, and
///   top-left neighbors.
pub fn levenstein_distance_ref(s: &str, t: &str) -> i32 {
    // Let:
    // s = source string of length m
    // t = target string of length n
    let s: Vec<char> = s.chars().collect(); // We need to be able to index into the chars.
    let t: Vec<char> = t.chars().collect();
    let m = s.len();
    let n = t.len();

    // Define a matrix D of size (m + 1) × (n + 1) where:
    // D[i][j] = Levenshtein distance between s[0:i] and t[0:j]

    // Create the DP matrix: (m+1) x (n+1)
    let mut D: Vec<Vec<i32>> = Vec::new();
    for _ in 0..(m + 1) {
        D.push(vec![0; n + 1]);
    }

    // Set the base cases.
    for i in 0..=m {
        // Transforming a non-empty string into an empty string requires deleting every
        // character.
        D[i][0] = i as i32;
    }
    for j in 0..=n {
        // Transforming an empty string into a non-empty string requires inserting every
        // character.
        D[0][j] = j as i32;
    }

    // The matrix now looks something like this:
    //     h e l l o
    //   ┌───────────┐
    // w │ 0 1 2 3 4 │
    // o │ 1 0 0 0 0 │
    // r │ 2 0 0 0 0 │
    // l │ 3 0 0 0 0 │
    // d │ 4 0 0 0 0 │
    //   └───────────┘

    // Fill in the full matrix of values.
    // Examples:
    //
    //   'stone' → 'stony'      'hello' → 'world'
    //       s t o n y              h e l l o
    //     ┌───────────┐          ┌───────────┐
    //     │ 1 2 3 4 5 │          │ 1 2 3 4 5 │
    //   s │ 0 1 2 3 4 │        w │ 1 2 3 4 5 │
    //   t │ 1 0 1 2 3 │        o │ 2 2 3 4 4 │
    //   o │ 2 1 0 1 2 │        r │ 3 3 3 4 5 │
    //   n │ 3 2 1 0 1 │        l │ 4 4 3 3 4 │
    //   e │ 4 3 2 1 1 │        d │ 5 5 4 4 4 │
    //     └───────────┘          └───────────┘
    //     distance = 1            distance = 4
    //
    // To do this we only need the three previously computed neighbords.
    //   D[i-1][j  ]
    //   D[i  ][j-1]
    //   D[i-1][j-1]
    for i in 1..=m {
        for j in 1..=n {
            // Look up the potential delete, substitue, and insert cost by looking
            // up values determined previously in the matrix.
            let delete_cost = D[i - 1][j] + 1;
            let insert_cost = D[i][j - 1] + 1;
            let substitute_cost = D[i - 1][j - 1]
                + if s[i - 1] == t[j - 1] {
                    // The characters are the same, no edit is needed.
                    0
                } else {
                    // The character are different, a substitution is needed.
                    1
                };

            D[i][j] = delete_cost.min(insert_cost).min(substitute_cost);
        }
    }

    return D[m][n];
}

/// An optimized version of the levenstein distance function.
///
/// Algorithmic Complexity:
///   O(m*n) time because we still evaluate each cell in the DP grid.
///
///   Memory space is O(n) because we reuse a single DP row (plus O(m+n) to
///   materialize the input strings as `Vec<char>` for indexing).
pub fn levenstein_distance_opt(s: &str, t: &str) -> i32 {
    // Fast path: identical strings, no DP needed. Runs in O(n) byte comparison.
    if s == t {
        return 0;
    }

    // Fast path: empty input reduces to the other string length (O(k) for char count).
    if s.is_empty() {
        return t.chars().count() as i32;
    }

    // Same as above, symmetric case.
    if t.is_empty() {
        return s.chars().count() as i32;
    }

    // Keep the DP row sized to the shorter string to reduce space to O(min(m, n)).
    let (s, t) = if s.chars().count() <= t.chars().count() {
        (s, t)
    } else {
        (t, s)
    };

    // Materialize Unicode scalar values for O(1) indexing during DP.
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let m = s.len();
    let n = t.len();
    // One row of the DP table; the ref version stores (m+1)*(n+1) cells.
    // This keeps only O(n) cells (O(min(m, n)) after the swap) instead of O(m*n).
    let mut row: Vec<i32> = (0..=n).map(|j| j as i32).collect();

    for i in 1..=m {
        // Save the top-left neighbor before overwriting row[0].
        let mut prev_diag = row[0];
        row[0] = i as i32;
        for j in 1..=n {
            // Preserve the old row[j] (the "top" neighbor) for next iteration.
            let old = row[j];
            // Use the top, left, and top-left neighbors to compute this cell.
            let delete_cost = row[j] + 1;
            let insert_cost = row[j - 1] + 1;
            let substitute_cost = prev_diag + if s[i - 1] == t[j - 1] { 0 } else { 1 };

            row[j] = delete_cost.min(insert_cost).min(substitute_cost);
            // Shift the diagonal for the next column.
            prev_diag = old;
        }
    }

    return row[n];
}

/// An optimized byte-based version of the levenstein distance function.
///
/// Algorithmic Complexity:
///   O(m*n) time because we still evaluate each cell in the DP grid.
///
///   Memory space is O(n) because we reuse a single DP row, and we avoid
///   materializing `Vec<char>` by operating directly on byte slices.
///
/// Note:
///   Operates on UTF-8 bytes, not Unicode scalar values, so results can differ
///   from the ref implementation for non-ASCII input.
pub fn levenstein_distance_byte_opt(s: &str, t: &str) -> i32 {
    // Fast path: identical strings, no DP needed. Runs in O(n) byte comparison.
    if s == t {
        return 0;
    }

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    // Fast path: empty input reduces to the other string length in bytes.
    if s_bytes.is_empty() {
        return t_bytes.len() as i32;
    }
    if t_bytes.is_empty() {
        return s_bytes.len() as i32;
    }

    // Keep the DP row sized to the shorter byte slice to reduce space to O(min(m, n)).
    let (s_bytes, t_bytes) = if s_bytes.len() <= t_bytes.len() {
        (s_bytes, t_bytes)
    } else {
        (t_bytes, s_bytes)
    };
    let m = s_bytes.len();
    let n = t_bytes.len();

    // One row of the DP table; the ref version stores (m+1)*(n+1) cells.
    // This keeps only O(n) cells (O(min(m, n)) after the swap) instead of O(m*n).
    let mut row: Vec<i32> = (0..=n).map(|j| j as i32).collect();

    for i in 1..=m {
        // Save the top-left neighbor before overwriting row[0].
        let mut prev_diag = row[0];
        row[0] = i as i32;
        for j in 1..=n {
            // Preserve the old row[j] (the "top" neighbor) for next iteration.
            let old = row[j];
            // Use the top, left, and top-left neighbors to compute this cell.
            let delete_cost = row[j] + 1;
            let insert_cost = row[j - 1] + 1;
            let substitute_cost =
                prev_diag + if s_bytes[i - 1] == t_bytes[j - 1] { 0 } else { 1 };

            row[j] = delete_cost.min(insert_cost).min(substitute_cost);
            // Shift the diagonal for the next column.
            prev_diag = old;
        }
    }

    return row[n];
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LevensteinCases {
        description: &'static str,
        cases: Vec<(&'static str, &'static str, i32)>,
    }

    fn get_cases() -> Vec<LevensteinCases> {
        vec![
            LevensteinCases {
                description: "Identity",
                cases: vec![("", "", 0), ("a", "a", 0), ("hello", "hello", 0)],
            },
            LevensteinCases {
                description: "Single edit",
                cases: vec![
                    ("a", "b", 1),
                    ("hello", "hallo", 1),
                    ("stone", "stony", 1),
                    ("sound", "found", 1),
                ],
            },
            LevensteinCases {
                description: "Insertion / deletion.",
                cases: vec![
                    ("", "a", 1),
                    ("a", "", 1),
                    ("test", "tests", 1),
                    ("tests", "test", 1),
                ],
            },
            LevensteinCases {
                description: "Various distances",
                cases: vec![
                    ("ab", "ba", 2),
                    ("crate", "trace", 2),
                    ("stone", "money", 3),
                    ("plane", "plans", 1),
                    ("kitten", "sitting", 3),
                    ("sunday", "saturday", 3),
                    ("apple", "grape", 4),
                ],
            },
            LevensteinCases {
                description: "Prefix/suffix behavior.",
                cases: vec![("abc", "abcde", 2), ("abcde", "abc", 2)],
            },
            LevensteinCases {
                description: "Symmetry checks.",
                cases: vec![("hello", "world", 4), ("world", "hello", 4)],
            },
        ]
    }

    #[test]
    fn test_levenstein_distance_ref() {
        for LevensteinCases { description, cases } in get_cases() {
            println!("\nCases: {}", description);
            for (str_a, str_b, value) in cases {
                println!(" - {} vs {}", str_a, str_b);
                assert_eq!(
                    levenstein_distance_ref(str_a, str_b),
                    value,
                    "{description}: {str_a} vs {str_b}"
                );
            }
        }
    }

    #[test]
    fn test_levenstein_distance_opt() {
        for LevensteinCases { description, cases } in get_cases() {
            println!("\nCases: {}", description);
            for (str_a, str_b, value) in cases {
                println!(" - {} vs {}", str_a, str_b);
                assert_eq!(
                    levenstein_distance_opt(str_a, str_b),
                    value,
                    "{description}: {str_a} vs {str_b}"
                );
            }
        }
    }

    #[test]
    fn test_levenstein_distance_byte_opt() {
        for LevensteinCases { description, cases } in get_cases() {
            println!("\nCases: {}", description);
            for (str_a, str_b, value) in cases {
                println!(" - {} vs {}", str_a, str_b);
                assert_eq!(
                    levenstein_distance_byte_opt(str_a, str_b),
                    value,
                    "{description}: {str_a} vs {str_b}"
                );
            }
        }
    }
}
