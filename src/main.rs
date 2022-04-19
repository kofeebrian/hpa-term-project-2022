#![feature(unchecked_math)]

extern crate num_cpus;

use std::u128;
use std::{env, fs::File, path::Path};
use std::io::{self, Write, BufReader, BufRead, BufWriter};
use std::thread;

static mut N: usize = 0;
static mut G: [u128; 100] = [0; 100];
static mut ANS: [u128; 101] = [0; 101];
static mut MIN: u128 = u128::MAX;

unsafe fn read_input(input: &str) -> io::Result<()> {
    let f = File::open(Path::new(input))?;
    let f = BufReader::new(f);
    let mut lines = f.lines().map(|line| line.unwrap());
    N = lines.next().unwrap().parse::<usize>().unwrap();
    let _ = lines.next();

    for i in 0..N {
        G[i] = 1u128 << i;
    }

    for line in lines {
        let temp = line.split_whitespace().flat_map(|num| num.parse::<usize>()).collect::<Vec<usize>>();
        let a = temp[0];
        let b = temp[1];
        G[a] |= 1 << (b as u128);
        G[b] |= 1 << (a as u128);
    }

    Ok(())
}

unsafe fn write_output(output: &str) -> io::Result<()> {
    let ans = ANS[MIN.count_ones() as usize];
    let f = File::create(Path::new(output))?;
    let mut f = BufWriter::new(f);
    let bitstring_result = format!("{:0128b}", ans.reverse_bits());

    writeln!(f, "{}:{}", ans.count_ones(), &bitstring_result[0..N])?;

    Ok(())
}

unsafe fn solve() {
    let nthreads = num_cpus::get();
    let nn = (1u128 << N) - 1u128;
    let mut workers = vec![];

    for i in 0..nthreads {
        workers.push(thread::spawn(move || {
            for j in (((i as u128) + 1)..=nn).step_by(nthreads) {
                if MIN.count_ones() <= j.count_ones() {
                    continue;
                }

                let mut sum: usize = 0;
                for k in 0..(N) {
                    sum = sum.unchecked_add((( *G.get_unchecked(k) & j) != 0) as usize);
                }

                if sum == N {
                    MIN = j;
                    ANS[j.count_ones() as usize] = j;
                }
            }

        }));
    }

    for w in workers {
        w.join().unwrap();
    }
}

fn main() {

    let args: Vec<String> = env::args().collect(); // Read input arguments

    unsafe {
        read_input(&args[1]).unwrap(); // Read input file
    }

    unsafe {
        solve();
    }

    unsafe {
        write_output(&args[2]).unwrap();
    }
}
