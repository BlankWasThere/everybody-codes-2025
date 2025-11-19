use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

static USAGE_STR: &str = concat!("Usage: ", env!("CARGO_BIN_NAME"), " <file path>");

struct Problem(Vec<u32>);

fn main() -> anyhow::Result<()> {
    let problem = parse_args()?;
    println!("The min amount of sets is {}.", get_min_sets(problem));

    Ok(())
}

fn parse_args() -> anyhow::Result<Problem> {
    let mut args = std::env::args_os().skip(1); // Skipping first because it will be the program name in most systems.
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            "Invalid number of arguments.\n\n{}",
            USAGE_STR
        ));
    }

    let path = args.next().unwrap();
    let path = Path::new(&path);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Unable to access the path `{}`.",
            path.to_string_lossy()
        ));
    }

    let mut buffer: Vec<u8> = Vec::new();
    let mut numbers: Vec<u32> = Vec::new();
    let mut reader = BufReader::new(File::open(path)?);

    loop {
        match reader.read_until(b',', &mut buffer) {
            Ok(0) => break, // EOL
            Ok(n @ 1..) => {
                let string = str::from_utf8(&buffer[..n - 1])?.trim();
                if !string.is_empty() {
                    numbers.push(string.parse::<u32>().map_err(|_| {
                        anyhow::anyhow!("Unable to parse {:?} as integer.", string)
                    })?);
                }
            }
            Err(e) => return Err(e.into()),
        }

        buffer.clear();
    }

    Ok(Problem(numbers))
}

fn get_min_sets(problem: Problem) -> usize {
    let mut numbers = problem.0;

    if numbers.is_empty() {
        return 0;
    }

    numbers.sort_by_key(|&w| std::cmp::Reverse(w));

    let mut sets = vec![numbers[0]];

    'outer: for &i in &numbers[1..] {
        for last_number in sets.iter_mut() {
            if i < *last_number {
                *last_number = i;
                continue 'outer;
            }
        }

        sets.push(i);
    }

    sets.len()
}
