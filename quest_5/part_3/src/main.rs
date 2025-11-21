use anyhow::Ok;
use clap::Parser;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    /// Path to the `everybody.codes` notes file.
    path: PathBuf,
}

#[derive(Debug)]
struct Problem(Vec<SubProblem>);

#[derive(Debug)]
struct SubProblem {
    identifier: u64,
    numbers: Vec<u64>,
}

impl Problem {
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Parsing logic

        let mut subproblems = Vec::new();
        let mut line = String::new();

        while reader.read_line(&mut line)? > 0 {
            let inner_line = line.trim();
            if inner_line.is_empty() {
                continue; // Skip empty lines
            }

            let mut reader = Cursor::new(inner_line.as_bytes());
            let mut buf = Vec::new();

            let identifier: u64;
            let mut numbers = Vec::new();

            // 1 - Read identifier
            let n = reader.read_until(b':', &mut buf)?;
            if let Some(&b':') = buf.last() {
                identifier = str::from_utf8(&buf[..n - 1])?.parse::<u64>()?;
                buf.clear();
            } else {
                return Err(anyhow::anyhow!("Invalid notes."));
            }

            // 2 - Read numbers
            while let n = reader.read_until(b',', &mut buf)?
                && n > 0
            {
                let string = str::from_utf8(&buf)?.trim().trim_end_matches(',');

                numbers.push(string.parse::<u64>()?);
                buf.clear();
            }

            subproblems.push(SubProblem {
                identifier,
                numbers,
            });

            line.clear();
        }

        Ok(Problem(subproblems))
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let problem = Problem::from_file(args.path)?;

    println!("The solution is: {}", find_solution(problem)?);

    Ok(())
}

fn find_solution(problem: Problem) -> anyhow::Result<u64> {
    #[derive(Debug, Default, Clone, Copy)]
    struct Segment {
        left: Option<u64>,
        mid: u64,
        right: Option<u64>,
    }

    if problem.0.is_empty() {
        return Err(anyhow::anyhow!("Empty list."));
    }

    let mut sub_solutions = Vec::new();

    for subproblem in problem.0 {
        let mut fishbone: Vec<Segment> = Vec::new();

        'outer: for number in subproblem.numbers {
            for segment in &mut fishbone {
                if number > segment.mid {
                    if segment.right.is_none() {
                        segment.right = Some(number);
                        continue 'outer;
                    }
                } else if number < segment.mid {
                    if segment.left.is_none() {
                        segment.left = Some(number);
                        continue 'outer;
                    }
                }
            }

            fishbone.push(Segment {
                mid: number,
                ..Default::default()
            });
        }

        let quality = fishbone
            .iter()
            .map(|s| s.mid)
            .reduce(|acc, e| {
                if e != 0 {
                    acc * 10_u64.pow(e.ilog10() + 1) + e
                } else {
                    acc
                }
            })
            .unwrap_or(0);

        sub_solutions.push((subproblem.identifier, fishbone, quality))
    }

    sub_solutions.sort_by(|a, b| match a.2.cmp(&b.2) {
        Ordering::Equal => {
            for index in 0..a.1.len().min(b.1.len()) {
                let first = {
                    let Segment { left, mid, right } = a.1[index];
                    [left.unwrap_or(0), mid, right.unwrap_or(0)]
                        .into_iter()
                        .reduce(|acc, e| {
                            if e != 0 {
                                acc * 10_u64.pow(e.ilog10() + 1) + e
                            } else {
                                acc
                            }
                        })
                        .unwrap_or(0)
                };
                let second = {
                    let Segment { left, mid, right } = b.1[index];
                    [left.unwrap_or(0), mid, right.unwrap_or(0)]
                        .into_iter()
                        .reduce(|acc, e| {
                            if e != 0 {
                                acc * 10_u64.pow(e.ilog10() + 1) + e
                            } else {
                                acc
                            }
                        })
                        .unwrap_or(0)
                };

                match first.cmp(&second) {
                    Ordering::Equal => (),
                    other => return other,
                }
            }

            a.0.cmp(&b.0)
        }
        other => other,
    });

    let checksum = sub_solutions
        .iter()
        .rev()
        .enumerate()
        .map(|(index, sword)| (index + 1) as u64 * sword.0)
        .sum::<u64>();

    Ok(checksum)
}
