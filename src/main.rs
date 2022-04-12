extern crate num_cpus;

use std::time::Instant;
use std::u128;
use std::{env, fs::File, path::Path};
use std::io::{BufReader, self, BufRead};
use std::thread;

// const NTHREADS: u32 = 8;
struct Input {}
impl Input {
    fn new(input: &String) -> io::Result<(usize, usize, [u128; 100])> {
        let f = File::open(Path::new(input))?;
        let f = BufReader::new(f);
        let mut lines = f.lines().map(|line| line.unwrap());
        let n = lines.next().unwrap().parse::<usize>().unwrap();
        let m = lines.next().unwrap().parse::<usize>().unwrap();
        
        let mut g= [0; 100]; 

        for line in lines {
            let temp = line.split_whitespace().flat_map(|num| num.parse::<usize>()).collect::<Vec<usize>>();
            let a = temp[0];
            let b = temp[1];
            g[a] = g[a] | (1 as u128) << (b as u128);
            g[b] = g[b] | (1 as u128) << (a as u128);
        }

        Ok((n, m, g))
    }
}

fn main() -> io::Result<()> {
    // input arguments from command line
    let now = Instant::now();
    let nthreads = num_cpus::get(); // Number of lofical threads

    let args: Vec<String> = env::args().collect(); // Read input arguments

    // Read input file, n = number of nodes, m = number of edges, g = graph
    let (n, _, g) = Input::new(&args[1])?; // Read input file
    let nn = 1u128 << n - 1;

    // let workers = vec![];
    for i in 1..=nn {

        let sum = g.into_iter().take(n).map(|e| (((e & i) != 0) as usize)).sum::<usize>();

        if sum == n {
            println!("found {:#b}", i);
        }

    }

    // let _ = workers.into_iter().map(|w| w.join());

    println!("{}ns", now.elapsed().as_nanos());

    Ok(())
}
