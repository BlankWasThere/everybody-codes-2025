static USAGE_STR: &str = concat!(
    "Usage: ",
    env!("CARGO_BIN_NAME"),
    " <name1,name2,...> <instr1,instr2,...>"
);

struct Problem {
    names: Vec<String>,
    instructions: Vec<(Action, usize)>,
}

enum Action {
    L,
    R,
}

fn main() -> anyhow::Result<()> {
    let problem = parse_args()?;
    match get_name(problem) {
        Some(name) => println!("The name is {:?}.", name),
        None => eprintln!("Couldn't find the name; The list is empty."),
    }

    Ok(())
}

fn parse_args() -> anyhow::Result<Problem> {
    let mut args = std::env::args().skip(1); // Skipping first because it will be the program name in most systems.
    if args.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid number of arguments.\n\n{}",
            USAGE_STR
        ));
    }

    let names = args
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
        .collect::<Vec<_>>();

    let instructions = args
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(trimmed)
            } else {
                None
            }
        })
        .map(|s| {
            let mut chars = s.trim().chars();
            let action = match chars.next() {
                Some(c) => match c {
                    'L' => Action::L,
                    'R' => Action::R,
                    unknown => return Err(anyhow::anyhow!("Invalid instruction `{}`.", unknown)),
                },
                None => unreachable!(),
            };

            let steps = chars.collect::<String>();
            let steps = match steps.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(anyhow::anyhow!("Unable to convert `{}` to number.", steps)),
            };

            Ok((action, steps))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(Problem {
        names,
        instructions,
    })
}

fn get_name(
    Problem {
        names,
        instructions,
    }: Problem,
) -> Option<String> {
    if names.is_empty() {
        return None;
    }

    let mut counter: usize = 0;
    for (action, steps) in instructions {
        counter = match action {
            Action::L => counter.saturating_sub(steps),
            Action::R => (counter + steps).min(names.len() - 1),
        }
    }

    Some(names[counter].clone())
}
