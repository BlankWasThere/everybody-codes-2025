use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

static USAGE_STR: &str = concat!("Usage: ", env!("CARGO_BIN_NAME"), " <input file>");

struct Problem(Vec<u32>);

fn main() -> anyhow::Result<()> {
    let problem = parse_args()?;
    match get_max_full_turns(problem) {
        Some(turns) => println!("The number of full turns is {}.", turns),
        None => eprintln!("The gear list is empty."),
    }

    Ok(())
}

fn parse_args() -> anyhow::Result<Problem> {
    let mut args = std::env::args().skip(1); // Skipping first because it will be the program name in most systems.
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

    let gears = {
        let mut buffer = String::new();
        let mut gears: Vec<u32> = Vec::new();
        let mut reader = BufReader::new(File::open(path)?);

        loop {
            match reader.read_line(&mut buffer) {
                Ok(0) => break, // EOL
                Ok(_) => {
                    let buffer = buffer.trim();
                    if !buffer.is_empty() {
                        gears.push(buffer.parse::<u32>().map_err(|_| {
                            anyhow::anyhow!("Unable to parse {:?} as integer.", buffer)
                        })?);
                    }
                }
                Err(e) => return Err(e.into()),
            }

            buffer.clear();
        }

        gears
    };

    Ok(Problem(gears))
}

fn get_max_full_turns(Problem(gears): Problem) -> Option<u32> {
    if gears.is_empty() {
        return None;
    }

    let first = gears.first().copied().unwrap();
    let last = gears.last().copied().unwrap();
    Some((2025 as f64 * (first as f64 / last as f64)).ceil() as u32)
}
