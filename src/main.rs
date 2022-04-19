#![feature(unchecked_math)]

extern crate num_cpus;

use std::u128;
use std::{env, fs::File, path::Path};
use std::io::{self, Write, BufReader, BufRead, BufWriter};
use std::thread;
use std::sync::Arc;

static mut ANS: [u128; 101] = [0; 101];
static mut MIN: u128 = u128::MAX;

struct Input {
    n: usize,
    g: [u128; 100],
}

fn read_input(input: &str) -> io::Result<Input> {
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
        g[a] |= 1 << (b as u128);
        g[b] |= 1 << (a as u128);
    }

    Ok(Input { n, g })
}

fn write_output(output: &str, val: u128) -> io::Result<()> {
    let f = File::create(Path::new(output))?;
    let mut f = BufWriter::new(f);
    writeln!(f, "{}: {:064b}", val.count_ones(), val)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // input arguments from command line
    let nthreads = num_cpus::get(); // Number of lofical threads

    let args: Vec<String> = env::args().collect(); // Read input arguments

    // Read input file, n = number of nodes, m = number of edges, g = graph
    let input = Arc::new(read_input(&args[1])?); // Read input file
    let nn = (1u128 << input.n) - 1u128; // 2 pow n

    let mut workers = vec![];

    for i in 0..nthreads {
        let input = Arc::clone(&input);
        workers.push(thread::spawn(move || {
            for j in (((i as u128) + 1)..=nn).step_by(nthreads) {

                unsafe {
                    if MIN.count_ones() <= j.count_ones() {
                        continue;
                    }
                }

                let mut sum : usize = 0;
                for k in 0..(input.n) {
                    unsafe {
                        sum = sum.unchecked_add((( *input.g.get_unchecked(k) & j) != 0) as usize);
                    }
                }
                
                if sum == input.n {
                    unsafe {
                        MIN = j;
                        ANS[j.count_ones() as usize] = j;
                    }
                }
            }
        }));
    }

    for w in workers {
        w.join().expect("Join Failed");
    }

    unsafe {
        let ans = ANS[MIN.count_ones() as usize];
        write_output(&args[2], ans).ok();
    }

    Ok(())
}
