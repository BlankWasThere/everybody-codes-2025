use std::collections::HashMap;

type Rules = HashMap<char, Vec<char>>;

fn parse_input(input: &str) -> anyhow::Result<(Vec<String>, Rules)> {
    let mut lines = input.trim().lines();
    let names = lines
        .next()
        .ok_or(anyhow::anyhow!("Missing names in input."))?
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() {
                Some(s.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let rules = lines
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (before, after) = s
                .split_once('>')
                .ok_or(anyhow::anyhow!("Invalid rule `{s}`"))?;

            let before = before
                .chars()
                .next()
                .ok_or(anyhow::anyhow!("Invalid rule `{s}`"))?;

            let after = after
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    if !s.is_empty() { Some(s) } else { None }
                })
                .map(|s| {
                    s.chars()
                        .next()
                        .ok_or(anyhow::anyhow!("Invalid rule `{s}`"))
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            Ok((before, after))
        })
        .collect::<anyhow::Result<HashMap<_, _>>>()?;

    Ok((names, rules))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let (names, rules) = parse_input(input)?;
    let mut sum = 0;

    'outer: for (index, name) in names.into_iter().enumerate() {
        for (prev, next) in name.chars().zip(name.chars().skip(1)) {
            if let Some(possible_nexts) = rules.get(&prev)
                && possible_nexts.contains(&next)
            {
                continue;
            }

            // Rules mismatch
            continue 'outer;
        }

        // Valid name found
        sum += index + 1;
    }

    println!("Answer: {sum}");

    Ok(())
}
