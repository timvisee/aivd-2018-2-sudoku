extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate permutator;

use std::cmp::Ord;
use std::fmt;

use itertools::Itertools;
use permutator::Permutation;

/// Coordinate;
type Coord = (usize, usize);

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
    static ref SERIES: Vec<Vec<Coord>> = [
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

    /// All the words that should be put over the series, flipped.
    static ref WORDS_FLIP: Vec<String> = WORDS
        .iter()
        .map(|w| w.chars().rev().collect())
        .collect();
}

fn main() {
    // Build the list of clusters
    let clusters = clusters();

    (0..WORDS.len())
        .collect::<Vec<usize>>()
        .permutation()
        .filter(|words| WORDS[words[8]].len() == SERIES[8].len())
        .flat_map(|words| {
            (0..words.len())
                .map(|_| [false, true].iter())
                .multi_cartesian_product()
                .map(|flips| flips
                    .into_iter()
                    .enumerate()
                    .map(|(i, flip)| if *flip {
                            WORDS_FLIP[words[i]].as_str()
                        } else {
                            WORDS[words[i]]
                        })
                    .collect::<Vec<&'static str>>()
                )
                .collect::<Vec<Vec<&'static str>>>()
        })
        .map(|words| {
            let mut f = Field::default(EMPTY);

            for (word, serie) in words.iter().zip(SERIES.iter()) {
                for (i, c) in word.chars().enumerate() {
                    f.f[serie[i].0][serie[i].1] = c;
                }
            }

            f
        })
        .filter(|field| is_field_valid(&field, &clusters))
        .for_each(|field| {
            println!("Possibility:\n{}", field);
        });
}

/// Collect all clusters (rows, columns and squares) of unique cells to check for full field
/// validation.
///
/// This returns only the clusters with cells that need to be checked, some fields are omitted for
/// this puzzle as they are not used.
fn clusters() -> Vec<Vec<Coord>> {
    // Define a bool field, mark all cells part of a series
    let mut f = Field::default(false);
    SERIES.iter()
        .flatten()
        .for_each(|c| f.f[c.0][c.1] = true);

    // Collect clusters of unique cells to check for full field validation
    (0..9usize)
        .map(|col| (0..9usize)
             .filter(|row| f.f[col][*row])
             .map(|row| (col, row))
             .collect::<Vec<Coord>>()
        )
        .chain((0..9usize)
            .map(|row| (0..9usize)
                .filter(|col| f.f[*col][row])
                .map(|col| (col, row))
                .collect::<Vec<Coord>>()
            )
        )
        .chain((0..3usize)
            .cartesian_product(0..3usize)
            .map(|(x, y)| (x * 3, y * 3))
            .map(|(x, y)| (0..3usize)
                .cartesian_product(0..3usize)
                .filter(|(col, row)| f.f[x + col][y + row])
                .map(|(col, row)| (x + col, y + row))
                .collect::<Vec<Coord>>()
            )
        )
        .filter(|c| c.len() >= 2)
        .collect::<Vec<Vec<Coord>>>()
}

/// Check whether the given field is valid for the current list of clusters.
fn is_field_valid<T>(field: &Field<T>, clusters: &Vec<Vec<Coord>>) -> bool
    where T: Ord + Copy
{
    clusters
        .iter()
        .all(|cluster| {
            let mut cells: Vec<T> = cluster
                .iter()
                .map(|c| field.f[c.0][c.1])
                .collect();
            cells.sort_unstable();
            cells.dedup();
            cells.len() == cluster.len()
        })
}

#[derive(Clone)]
struct Field<T> {
    f: [[T; 9]; 9],
}

impl<T> Field<T>
    where T: Copy
{
    fn default(value: T) -> Field<T> {
        Field {
            f: [[value; 9]; 9],
        }
    }
}

/// Humanly display sudoku field
impl<T> fmt::Display for Field<T>
    where T: fmt::Display
{
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
