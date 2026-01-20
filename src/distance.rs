#![allow(dead_code, non_snake_case)]

/// Compute the levenstein distance between two numbers, also known as the edit distance.
/// Edit types:
///   Insertions, deletions, substitutes
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
pub fn levenstein_distance_opt(s: &str, t: &str) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let m = s.len();
    let n = t.len();
    let mut D: Vec<Vec<i32>> = Vec::new();
    for _ in 0..(m + 1) {
        D.push(vec![0; n + 1]);
    }
    for i in 0..=m {
        D[i][0] = i as i32;
    }
    for j in 0..=n {
        D[0][j] = j as i32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let delete_cost = D[i - 1][j] + 1;
            let insert_cost = D[i][j - 1] + 1;
            let substitute_cost = D[i - 1][j - 1] + if s[i - 1] == t[j - 1] { 0 } else { 1 };

            D[i][j] = delete_cost.min(insert_cost).min(substitute_cost);
        }
    }

    return D[m][n];
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
}
