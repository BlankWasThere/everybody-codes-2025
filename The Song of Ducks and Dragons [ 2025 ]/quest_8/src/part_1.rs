fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .trim()
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| Ok(s.parse()?))
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const TOTAL: u32 = 32;

    let numbers = parse_input(input)?;
    let mut count = 0;

    // Only even pins circle has threads passing through center.
    if TOTAL.is_multiple_of(2) {
        for (&prev, &next) in numbers.iter().zip(numbers.iter().skip(1)) {
            let diff = next.abs_diff(prev);
            if diff as f64 * (360.0 / TOTAL as f64) == 180.0 {
                count += 1;
            }
        }
    }

    println!("Answer: {count}");

    Ok(())
}
