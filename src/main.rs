use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::Index,
    ops::IndexMut,
};

use itertools::Itertools;
use pathfinding::directed::astar::astar;
use smallvec::{Array, SmallVec};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Board {
    len_i: usize,
    len_j: usize,
    board: Vec<u8>,
}

impl Board {
    fn new(from: Vec<Vec<u8>>) -> Board {
        let len_i = from.len();
        let len_j = from[0].len(); // TODO: check all lengths for consistency
        let board = from.iter().flatten().copied().collect();

        let mut result = Board {
            len_i,
            len_j,
            board,
        };
        normalize(&mut result);
        result
    }

    fn empties(&self) -> [(usize, usize); 2] {
        let mut result = [(0, 0); 2];
        let mut result_ix = 0;
        for ix in 0..self.board.len() {
            if self.board[ix] == SPACE {
                result[result_ix] = (ix / self.len_j, ix % self.len_j);
                result_ix += 1;
                if result_ix == 2 {
                    break;
                }
            }
        }

        result
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for line in self.board.chunks(self.len_j) {
            if !first {
                f.write_char('\n')?;
            } else {
                first = false;
            }

            for c in line {
                f.write_char(if *c == SPACE || *c == WALL || c.is_ascii_uppercase() {
                    *c as char
                } else {
                    (c - 1 + b'a') as char
                })?;
            }
        }

        Ok(())
    }
}

impl Index<(usize, usize)> for Board {
    type Output = u8;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.board[index.0 * self.len_j + index.1]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.board[index.0 * self.len_j + index.1]
    }
}

fn parse_board(b: &str) -> Board {
    let board: Vec<Vec<u8>> = b
        .trim()
        .lines()
        .map(|row| row.as_bytes().iter().copied().collect())
        .collect();

    Board::new(board)
}

// TODO: is there a way to do this without allocating? or maybe compiler optimizes it away?
fn from_to_delta(from: usize, to: usize, delta: i32) -> Box<dyn Iterator<Item = usize>> {
    match delta {
        0 | 1 => Box::new((from..to).rev()),
        -1 => Box::new(from..to),
        _ => unreachable!(),
    }
}

const SPACE: u8 = b' ';
const WALL: u8 = b'#';

fn slide<const N: usize>(b: &Board, to_move: u8, delta: (i32, i32), out: &mut SmallVec<[Board; N]>)
where
    [Board; N]: Array<Item = Board>,
{
    let mut result = None;
    for i in from_to_delta(0, b.len_i, delta.0) {
        for j in from_to_delta(0, b.len_j, delta.1) {
            if b[(i, j)] == to_move {
                let target = ((i as i32 + delta.0) as usize, (j as i32 + delta.1) as usize);
                // NB: requiring WALLS around outside means this can’t go OOB
                let value = b[target];
                if value == SPACE || value == to_move {
                    let r = result.get_or_insert_with(|| b.clone());
                    r[target] = to_move;
                    r[(i, j)] = SPACE;
                } else {
                    return;
                }
            }
        }
    }

    if let Some(mut v) = result {
        // now try to push the block further
        // don’t normalize yet as that could rename the block
        for d in DELTAS {
            // don’t go back to previous position
            // NB: if there are enough spaces on the board this doesn’t work
            // and we would need to check what’s already in `out`
            if (-d.0, -d.1) == delta {
                continue;
            }

            slide(&v, to_move, *d, out);
        }

        normalize(&mut v);
        out.push(v);
    }
}

fn normalize(b: &mut Board) {
    let mut count: u8 = 0;
    let mut lookup = HashMap::new();
    for c in &mut b.board {
        if *c == SPACE || *c == WALL || c.is_ascii_uppercase() {
            continue;
        }

        let value = lookup.entry(*c).or_insert_with(|| {
            count += 1;
            count
        });

        *c = *value;
    }
}

const DELTAS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

fn perform_moves<const N: usize>(board: &Board, out: &mut SmallVec<[Board; N]>)
where
    [Board; N]: Array<Item = Board>,
{
    let locations = board.empties();

    let mut examined = SmallVec::<[(u8, (i32, i32)); 10]>::new();

    for loc in locations {
        for &delta in DELTAS {
            // NB: requiring WALLS around outside means this can’t index out-of-bounds
            let look_at = (
                (loc.0 as i32 + delta.0) as usize,
                (loc.1 as i32 + delta.1) as usize,
            );
            let value = board[look_at];
            if value != WALL && value != SPACE {
                let examine = (board[look_at], (-delta.0, -delta.1));
                if !examined.contains(&examine) {
                    examined.push(examine);
                }
            }
        }
    }

    for (c, delta) in examined {
        slide(board, c, delta, out);
    }
}

// Format:
// a # is a wall, must be around outside,
// a space is free space,
// capital letters are blocks with identity (non-fungible),
// lowercase letters or numbers are blocks without identity (can be interchanged).
//
// In the target, spaces are ignored.
const TESTS: &[(&str, &str)] = &[
    (
        "
######
#1AA2#
#1AA2#
#4335#
#4675#
#8  9#
######
",
        "
######
#    #
#    #
#    #
# AA #
# AA #
######
",
    ),
    (
        "
######
#MAAN#
#MAAN#
#OWWP#
#ObcP#
#a  d#
######
",
        "
######
# MN #
#OMNP#
#OWWP#
#aAAb#
#cAAd#
######
",
    ),
    (
        "
######
#AA11#
#AA22#
#34  #
#5677#
#5688#
######
",
        "
######
#    #
#    #
#    #
#AA  #
#AA  #
######",
    ),
];

fn matches_target(source: &Board, target: &Board) -> bool {
    for (b, t) in source.board.iter().zip_eq(&target.board) {
        if *t == SPACE {
            continue;
        }

        if b != t {
            return false;
        }
    }

    true
}

fn solve(source: &Board, target: &Board) -> (usize, usize, Option<(Vec<Board>, i32)>) {
    assert!(source.len_i == target.len_i);
    assert!(source.len_j == target.len_j);

    println!("----");
    println!("Source:");
    println!("{}", source);
    println!("----");

    println!("Target:");
    println!("{}", target);
    println!("----");

    let mut visited = 0;
    let mut generated = 0;

    let result: Option<(_, i32)> = astar(
        source,
        |b| {
            let mut buffer = SmallVec::<[Board; 10]>::new();
            perform_moves(b, &mut buffer);

            visited += 1;
            generated += buffer.len();

            buffer.into_iter().map(|b| (b, 1))
        },
        |_| 0, // brute-force search, no heuristic
        |b| matches_target(b, target),
    );

    (visited, generated, result)
}

fn main() {
    for (source, dest) in TESTS {
        let board = parse_board(source);
        let target = parse_board(dest);

        let (visited, generated, result) = solve(&board, &target);

        if let Some((_boards, cost)) = result {
            println!("Found a solution in {} moves:", cost);
            println!(
                "Visited {} board positions (generated {} total).",
                visited, generated
            );

            println!("----");
            println!();
        } else {
            println!("No solution found");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_expected() {
        let source = parse_board(TESTS[0].0);
        let target = parse_board(TESTS[0].1);

        let (_, _, result) = solve(&source, &target);

        assert!(result.is_some());
        assert!(result.unwrap().1 == 81); // minimal moves is 81
    }
}
