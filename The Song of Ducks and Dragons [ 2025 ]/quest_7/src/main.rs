use std::io::ErrorKind;

mod part_1;
mod part_2;
mod part_3;

fn read_input(path: &str) -> anyhow::Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn main() -> anyhow::Result<()> {
    // Part 1
    if let Some(input) = read_input("input_1.txt")? {
        println!("========== Part 1 ==========");
        part_1::solve(&input)?;
    }

    // Part 2
    if let Some(input) = read_input("input_2.txt")? {
        println!("========== Part 2 ==========");
        part_2::solve(&input)?;
    }

    // Part 3
    if let Some(input) = read_input("input_3.txt")? {
        println!("========== Part 3 ==========");
        part_3::solve(&input)?;
    }

    Ok(())
}
