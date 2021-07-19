use std::fs::File;
use std::io::BufWriter;

use rand::distributions::*;
use rand::SeedableRng;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;
use rand_mt::Mt19937GenRand64;

use read_and_write;

fn warn() {
    println!("You need 2 args. Input trajectory file path and Output file path.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path: &str;
    let output_path: &str;
    match args.len() {
        3 => {
            input_path = &args[1];
            output_path = &args[2];
        }
        _ => {
            warn();
            return;
        }
    }

    let traj = read_and_write::read_trajectory_file(input_path);
    let mut output_buf = match File::create(output_path) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create: {}", output_path),
    };

    let seed = 101;
    let variance = 1.0f64;
    // let mut rng = Xoshiro256StarStar::seed_from_u64(seed);
    let mut rng = Mt19937GenRand64::seed_from_u64(seed);
    let normal = Normal::new(0.0, variance.sqrt()).unwrap();
    for i in 0..traj.len() {
        read_and_write::write_x(
            &mut output_buf,
            traj[i].0,
            &traj[i].1.map(|x| x + normal.sample(&mut rng)),
        );
    }
}
