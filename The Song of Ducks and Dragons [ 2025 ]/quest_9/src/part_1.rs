use std::ptr;

use anyhow::{anyhow, ensure};

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<char>>> {
    input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (_, t) = s.split_once(':').ok_or(anyhow!("Invalid line `{s}`"))?;
            Ok(t.chars().collect::<Vec<_>>())
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let dna_list = parse_input(input)?;
    let mut product = 0;

    ensure!(
        dna_list.len() == 3,
        "Expected 3 DNA, found {}",
        dna_list.len()
    );

    'outer: for curr in &dna_list {
        let mut first_count = 0;
        let mut second_count = 0;

        let mut others = dna_list.iter().filter(|&e| !ptr::eq(curr, e));

        for ((&a, &b), &c) in curr
            .iter()
            .zip(others.next().unwrap())
            .zip(others.next().unwrap())
        {
            let mut matched = false;
            if a == b {
                first_count += 1;
                matched = true;
            }

            if a == c {
                second_count += 1;
                matched = true;
            }

            if !matched {
                continue 'outer;
            }
        }

        product = first_count * second_count;
    }

    println!("Answer: {product}");

    Ok(())
}
