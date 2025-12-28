pub fn solve(input: &str) -> anyhow::Result<()> {
    let mut count = 0;

    let mut temp_count = 0;
    for c in input.chars() {
        if c == 'A' {
            temp_count += 1;
        } else if c == 'a' {
            count += temp_count;
        }
    }

    println!("Answer: {count}");
    Ok(())
}
