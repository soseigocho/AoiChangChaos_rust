use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use rand::distributions::*;
use rand::SeedableRng;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

fn warn() {
    println!("You need 3 inputs. Output file path, Time interval, Number of data and Seed of randg.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_file_path: &str;
    let t: f64;
    let n: f64;
    let s: u64;
    match args.len() {
        5 => {
            output_file_path = &args[1];
            t = args[2].parse().ok().unwrap();
            n = args[3].parse().ok().unwrap();
            s = args[4].parse().ok().unwrap();
        },
        _ => {
            warn();
            return
        }
    }

    let mut buf = match File::create(output_file_path) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create: {}", output_file_path)
    };
    writeln!(buf, "{} {}", "time_span", t);
    writeln!(buf, "{} {}", "simulation_time", n);

    let mut rng = Xoshiro256StarStar::seed_from_u64(s);
    let normal = Normal::new(0.0, 0.5).unwrap();
    for i in 1..(n/t) as usize + 1 {
        let current_time = t*i as f64;
        writeln!(buf, "{:e} {:e}", current_time, current_time.sin()
            + normal.sample(&mut rng));
    }
}
