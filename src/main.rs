use std::env;
use anyhow::{bail, Result};
use itertools::*;
use mod_exp::mod_exp;
use rayon::prelude::*;

type Group = Vec<usize>;

fn get_input() -> Result<usize> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Invalid args. Only expect 1 as usize (for p)")
    }
    Ok(args
        .get(1)
        .unwrap()
        .trim()
        .parse()
        .expect("Couldn't parse argument! Expect usize"))
}

fn main() {
    let p = get_input().expect("Couldn't get input!");
    let vec: Vec<(usize,Group)> = (2..p)
        .into_par_iter()
        .map(|x| (x,create_group(p, x)))
        // filter out non generator subgroups
        .filter(|(_,g)| g.iter().count() == p-1)
        .collect();

    // print generator with their group
    vec.iter().for_each(|(x,g)| println!("{x}:{:?}", g));
}

fn create_group(p: usize, a: usize) -> Group {
    (1..p)
        .map(|i| mod_exp(a, i, p))
        .unique()
        .collect()
}
