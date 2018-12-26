#[macro_use]
extern crate lazy_static;

use std::fmt;

/// The empty cell value
const EMPTY: char = '.';

/// All the words that should be put over the series.
const WORDS: [&'static str; 9] = [
    "KAARS",
    "KANIS",
    "KERST",
    "KOKOS",
    "KRANS",
    "KRENT",
    "SNOER",
    "STOEI",
    "TREITEREN",
];

lazy_static!{
    /// The series over the field
    static ref SERIES: Vec<Vec<(usize, usize)>> = [
            vec![(1, 1), (2, 1), (3, 2), (2, 2), (3, 1)],
            vec![(3, 4), (2, 3), (3, 3), (4, 4), (4, 3)],
            vec![(2, 7), (1, 6), (1, 5), (2, 4), (2, 5)],
            vec![(3, 6), (4, 6), (3, 7), (4, 7), (5, 8)],
            vec![(6, 7), (6, 8), (5, 9), (6, 9), (7, 9)],
            vec![(9, 7), (9, 8), (8, 8), (8, 9), (9, 9)],
            vec![(7, 4), (8, 5), (8, 6), (7, 7), (6, 6)],
            vec![(8, 1), (7, 2), (8, 3), (9, 4), (9, 5)],
            vec![(7, 5), (6, 4), (5, 4), (5, 3), (6, 2), (7, 1), (8, 2), (7, 3), (8, 4)],
        ]
        .into_iter()
        .map(|s| s.into_iter().map(|(x, y)| (x - 1usize, y - 1usize)).collect())
        .collect();
}

fn main() {
    let mut f = Field::default();

    SERIES[0].iter().zip(WORDS[0].chars())
        .for_each(|((x, y), c)| {
            f.f[*x][*y] = c;
        });

    println!("{}", f);
}

#[derive(Clone)]
struct Field {
    f: [[char; 9]; 9],
}

impl Default for Field {
    fn default() -> Self {
        Field {
            f: [[EMPTY; 9]; 9],
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            for col in 0..9 {
                write!(f, "{}", self.f[col][row])?;
                if col == 2 || col == 5 {
                    write!(f, "|")?;
                }
            }
            write!(f, "\n")?;
            if row == 2 || row == 5 {
                write!(f, "---+---+---\n")?;
            }
        }

        Ok(())
    }
}
