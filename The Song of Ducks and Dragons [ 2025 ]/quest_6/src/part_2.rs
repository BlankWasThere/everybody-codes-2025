use std::collections::HashMap;

pub fn solve(input: &str) -> anyhow::Result<()> {
    let mut count = 0;
    let mut collection = HashMap::new();

    for c in input.chars() {
        if c.is_uppercase() {
            collection.entry(c).and_modify(|e| *e += 1).or_insert(1);
        } else if c.is_lowercase() {
            count += collection.get(&c.to_ascii_uppercase()).unwrap_or(&0);
        }
    }

    println!("Answer: {count}");
    Ok(())
}
