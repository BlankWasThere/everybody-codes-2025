use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    /// Path to the `everybody.codes` notes file.
    path: PathBuf,
}

#[derive(Debug)]
struct Problem {
    #[allow(unused)]
    identifier: u32,
    numbers: Vec<u32>,
}

impl Problem {
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();

        let identifier: u32;
        let mut numbers = Vec::new();

        // Parsing logic

        // 1 - Read identifier
        let n = reader.read_until(b':', &mut buf)?;
        if let Some(&b':') = buf.last() {
            identifier = str::from_utf8(&buf[..n - 1])?.parse::<u32>()?;
            buf.clear();
        } else {
            return Err(anyhow::anyhow!("Invalid notes."));
        }

        // 2 - Read numbers
        while let n = reader.read_until(b',', &mut buf)?
            && n > 0
        {
            let string = str::from_utf8(&buf)?.trim().trim_end_matches(',');

            numbers.push(string.parse::<u32>()?);
            buf.clear();
        }

        Ok(Problem {
            identifier,
            numbers,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let problem = Problem::from_file(args.path)?;

    println!("The solution is: {}", find_solution(problem)?);

    Ok(())
}

fn find_solution(problem: Problem) -> anyhow::Result<String> {
    #[derive(Debug, Default, Clone, Copy)]
    struct Segment {
        left: Option<u32>,
        mid: u32,
        right: Option<u32>,
    }

    let mut fishbone: Vec<Segment> = Vec::new();

    'outer: for number in problem.numbers {
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

    Ok(fishbone.into_iter().map(|s| s.mid.to_string()).collect())
}
