use std::fs::File;
use std::io::BufWriter;

use ndarray::*;
use ndarray::Array;

use rand::SeedableRng;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

use merging_particle_filter::*;
use read_and_write::*;

const S:u64 = 801;
const PS:f64 = 0.6;

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
    let h = Array::eye(1);
    let r = arr2(&[[10.0_f64]]);

    let (observation_data, observation_span, simulation_time_length) = read_observation_file(input);
    let mut buf = match File::create(output) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create: {}", output)
    };

    let mut mpf = MergingParticleFilter {
        member : MergingParticleFilterMember{x,v,h,r},
        observation_data,
        observation_span,
        simulation_time_length,
        merge_magnification : 3,
        merge_alpha : vec!{3.0_f64/4.0_f64, 
            (13.0_f64.sqrt()+1.0_f64)/8.0_f64,
            -(13.0_f64.sqrt()-1.0_f64)/8.0_f64},
        rng : Xoshiro256StarStar::seed_from_u64(S),
        predict_distribution : Normal::new(0.0, PS).unwrap()
    };

    mpf.run_and_write(&mut buf,|a,b|{a+b});
}