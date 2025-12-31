use std::ptr;

use anyhow::anyhow;

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
    let mut total = 0;

    for curr in &dna_list {
        for first_parent in dna_list.iter().filter(|&p| !ptr::eq(p, curr)) {
            'outer: for second_parent in dna_list
                .iter()
                .filter(|&p| !ptr::eq(p, curr) && !ptr::eq(p, first_parent))
            {
                let mut first_count = 0;
                let mut second_count = 0;

                for ((&a, &b), &c) in curr.iter().zip(first_parent).zip(second_parent) {
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

                total += first_count * second_count;
            }
        }
    }

    // Since a parent can match with the child and another parent, we get 2x the value
    // So we have to divide the total with 2 to get the real value.
    total /= 2;

    println!("Answer: {total}");

    Ok(())
}
