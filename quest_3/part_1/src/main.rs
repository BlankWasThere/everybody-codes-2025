static USAGE_STR: &str = concat!(
    "Usage: ",
    env!("CARGO_BIN_NAME"),
    " <crate1,crate2,crate3...>"
);

struct Problem(Vec<u32>);

fn main() -> anyhow::Result<()> {
    let problem = parse_args()?;

    println!(
        "The largest possible crate size is {}.",
        get_max_crate_size(problem)
    );

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

    let numbers = args
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(trimmed.to_owned())
            } else {
                None
            }
        })
        .map(|s| s.parse::<u32>().map_err(Into::into))
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(Problem(numbers))
}

fn get_max_crate_size(problem: Problem) -> u32 {
    let mut numbers = problem.0;

    if numbers.is_empty() {
        return 0;
    }

    numbers.sort_by_key(|&w| std::cmp::Reverse(w));

    let mut last_number = numbers[0];
    let mut max_size = numbers[0];

    for &i in &numbers[1..] {
        if i < last_number {
            last_number = i;
            max_size += i;
        }
    }

    max_size
}
