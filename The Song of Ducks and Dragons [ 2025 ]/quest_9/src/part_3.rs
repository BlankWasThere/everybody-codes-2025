use std::{collections::HashSet, ptr};

use anyhow::anyhow;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<(u32, Vec<char>)>> {
    input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (n, t) = s.split_once(':').ok_or(anyhow!("Invalid line `{s}`"))?;
            Ok((n.parse()?, t.chars().collect::<Vec<_>>()))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let dna_list = parse_input(input)?;
    let mut families: Vec<HashSet<u32>> = vec![];

    for &(fid, ref first_parent) in &dna_list {
        for &(sid, ref second_parent) in &dna_list {
            if ptr::eq(first_parent, second_parent) {
                continue;
            }

            'outer: for &(cid, ref child) in &dna_list {
                if ptr::eq(child, first_parent) || ptr::eq(child, second_parent) {
                    continue;
                }

                for ((a, b), c) in child.iter().zip(first_parent).zip(second_parent) {
                    if a != b && a != c {
                        continue 'outer;
                    }
                }

                let mut found = false;
                for family in &mut families {
                    if family.contains(&fid) && family.contains(&sid) {
                        family.insert(cid);
                        found = true;
                    }
                }

                // No common family
                if !found {
                    let mut new_family = families
                        .iter()
                        .filter(|&f| f.contains(&fid) || f.contains(&sid) || f.contains(&cid))
                        .flatten()
                        .copied()
                        .collect::<HashSet<_>>();

                    new_family.extend([cid, fid, sid]);

                    families.push(new_family);
                }
            }
        }
    }

    let biggest_family = families
        .iter()
        .max_by_key(|f| f.len())
        .map(|f| f.iter().sum::<u32>())
        .unwrap_or(0);

    println!("Answer: {biggest_family}");

    Ok(())
}
