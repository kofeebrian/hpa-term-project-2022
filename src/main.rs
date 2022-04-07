// extern crate blas_src;

use std::{env, fs::File, path::Path};
use std::io::{BufReader, self, BufRead};
use ndarray::{prelude::*, OwnedRepr};

// const NTHREADS: u32 = 8;
struct Input {}
impl Input {
    fn new(input: &String) -> io::Result<(usize, usize, ArrayBase<OwnedRepr<i64>, Dim<[usize; 2]>>)> {
        let f = File::open(Path::new(input))?;
        let f = BufReader::new(f);
        let mut lines = f.lines().map(|line| line.unwrap());
        let n = lines.next().unwrap().parse::<usize>().unwrap();
        let m = lines.next().unwrap().parse::<usize>().unwrap();
        
        let mut g = Array2::<i64>::zeros(Ix2(n, n));

        for line in lines {
            let temp = line.split_whitespace().flat_map(|num| num.parse::<usize>()).collect::<Vec<usize>>();
            let a = temp[0];
            let b = temp[1];
            g[[a, b]] = 1;
            g[[b, a]] = 1;
        }

        Ok((n, m, g))
    }
}

fn main() -> io::Result<()> {
    // input arguments from command line
    let args: Vec<String> = env::args().collect();
    let (n, m, g) = Input::new(&args[1])?;
    let v = Array1::<i64>::from_vec(vec![0, 0, 0, 1, 1, 0]);

    println!("{:?}", g.dot(&v));

    Ok(())
}
