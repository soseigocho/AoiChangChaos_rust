use std::fs::File;
use std::io::BufWriter;

use ndarray::Array;

use rand::SeedableRng;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

use ensemble_kalman_filter::*;
use read_and_write::*;

const S:u64 = 808;
const PS:f64 = 0.2;
const FS:f64 = 0.2;

fn warn() {
    println!("You need 2 args. Input observation file path and Output file path.");
}

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let input:&str;
    let output:&str;
    match args.len() {
        3 => {
            input = &args[1];
            output = &args[2];
        },
        _ => {
            warn();
            return
        }
    }

    let x = Array::zeros((1,1000));
    let v = Array::from_elem((1,1000), 1.0_f64);
    let w = Array::from_elem((1,1000), 1.0_f64);
    let h = Array::eye(1);

    let (observation_data, observation_span, simulation_time_length) = read_observation_file(input);
    let mut buf = match File::create(output) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create: {}", output)
    };

    let mut enkf = EnsembleKalmanFilter {
        member : EnsembleKalmanFilterMember{x,v,w,h},
        observation_data,
        observation_span,
        simulation_time_length,
        rng : Xoshiro256StarStar::seed_from_u64(S),
        predict_distribution : Normal::new(0.0, PS).unwrap(),
        filter_distribution : Normal::new(0.0, FS).unwrap()
    };

    enkf.run_and_write(&mut buf,|a,b|{a+b});
}