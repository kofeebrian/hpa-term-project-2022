extern crate num_cpus;

use std::time::Instant;
use std::u128;
use std::{env, fs::File, path::Path};
use std::io::{BufReader, self, BufRead, Write};
use std::thread;
use std::sync::{Arc};

struct Input {
    n: usize,
    g: [u128; 100],
}

fn read_input(input: &String) -> io::Result<Input> {
    let f = File::open(Path::new(input))?;
    let f = BufReader::new(f);
    let mut lines = f.lines().map(|line| line.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let _ = lines.next();
    
    let mut g= [0; 100]; 

    for i in 0..n {
        g[i] = 1u128 << i;
    }

    for line in lines {
        let temp = line.split_whitespace().flat_map(|num| num.parse::<usize>()).collect::<Vec<usize>>();
        let a = temp[0];
        let b = temp[1];
        g[a] = g[a] | 1 << (b as u128);
        g[b] = g[b] | 1 << (a as u128);
    }

    Ok(Input { n, g })
}

fn write_output(output: &String, val: u128) -> io::Result<()> {
    let mut f = File::create(Path::new(output))?;
    write!(f, "{}: {:0b}", val.count_ones(), val)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // input arguments from command line
    let now = Instant::now();
    let nthreads = num_cpus::get(); // Number of lofical threads

    let args: Vec<String> = env::args().collect(); // Read input arguments

    // Read input file, n = number of nodes, m = number of edges, g = graph
    let input = Arc::new(read_input(&args[1])?); // Read input file
    let nn = (1u128 << input.n) - 1u128; // 2 pow n

    let mut workers = vec![];

    for i in 0..nthreads {
        let input = Arc::clone(&input);
        workers.push(thread::spawn(move || -> Option<u128> {

            let mut ans = u128::MAX;
            let mut found = false;

            for j in (((i as u128) + 1)..=nn).step_by(nthreads) {
                let sum = input.g.into_iter()
                    .take(input.n)
                    .map(|e| ((e & j) != 0) as usize)
                    .sum::<usize>();

                if sum == input.n {
                    found = true;
                    ans = u128::min(ans, j);
                }
            }

            if found {
                return Some(ans);
            }

            None
        }));
    }

    let ans = workers.into_iter()
        .filter_map(|w| w.join().unwrap())
        .reduce(|acc, cur| {
            if acc.count_ones() > cur.count_ones() {
                return cur;
            }
            acc
        });
    
    if let Some(val) = ans {
        write_output(&args[2], val).ok();
    }

    println!("{}s", now.elapsed().as_secs_f64());

    Ok(())
}
