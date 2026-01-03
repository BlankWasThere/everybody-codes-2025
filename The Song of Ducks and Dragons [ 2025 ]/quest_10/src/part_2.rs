use anyhow::bail;
use std::collections::HashSet;

type Board = Vec<Vec<bool>>;
type Point = (usize, usize);
type Hideouts = HashSet<Point>;

fn parse_input(input: &str) -> anyhow::Result<(Point, Board, Hideouts)> {
    let mut dragon = None;
    let mut hideouts = HashSet::new();

    let board = input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Ok(false),
                    'S' => Ok(true),
                    '#' => {
                        hideouts.insert((row, col));
                        Ok(false)
                    }
                    'D' => {
                        if dragon.is_none() {
                            dragon = Some((row, col))
                        } else {
                            bail!("Multiple dragons found.")
                        };
                        Ok(false)
                    }
                    other => bail!("Invalid character found `{other}`"),
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let Some(dragon) = dragon else {
        bail!("Dragon not found.")
    };

    Ok((dragon, board, hideouts))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const MOVES: u32 = 20;
    let (dragon, board, hideouts) = parse_input(input)?;

    let mut visited = HashSet::new();
    let mut sheeps = HashSet::new();

    let mut stack = vec![((dragon.0 as isize, dragon.1 as isize), MOVES, 0)];
    while let Some(((row, col), remaining_moves, mut sheep_movement)) = stack.pop() {
        if row < 0
            || col < 0
            || row as usize >= board.len()
            || col as usize >= board[row as usize].len()
        {
            continue;
        }

        if !visited.insert(((row, col), remaining_moves)) {
            continue;
        }

        if remaining_moves != MOVES {
            // Before sheep's turn
            if let Some(moved_row) = (row as usize).checked_sub(sheep_movement) {
                if board[moved_row as usize][col as usize]
                    && !hideouts.contains(&(row as usize, col as usize))
                {
                    sheeps.insert((moved_row as isize, col));
                }
            }

            sheep_movement += 1;

            // After sheep's turn
            if let Some(moved_row) = (row as usize).checked_sub(sheep_movement) {
                if board[moved_row as usize][col as usize]
                    && !hideouts.contains(&(row as usize, col as usize))
                {
                    sheeps.insert((moved_row as isize, col));
                }
            }
        }

        if remaining_moves == 0 {
            continue;
        }

        stack.extend([
            ((row - 2, col - 1), remaining_moves - 1, sheep_movement),
            ((row - 2, col + 1), remaining_moves - 1, sheep_movement),
            ((row + 2, col - 1), remaining_moves - 1, sheep_movement),
            ((row + 2, col + 1), remaining_moves - 1, sheep_movement),
            ((row - 1, col - 2), remaining_moves - 1, sheep_movement),
            ((row + 1, col - 2), remaining_moves - 1, sheep_movement),
            ((row - 1, col + 2), remaining_moves - 1, sheep_movement),
            ((row + 1, col + 2), remaining_moves - 1, sheep_movement),
        ]);
    }

    println!("Answer: {}", sheeps.len());

    Ok(())
}
