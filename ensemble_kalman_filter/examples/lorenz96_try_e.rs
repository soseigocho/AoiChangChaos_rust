use std::fs::File;
use std::io::BufWriter;

use ndarray::Array;
use ndarray::*;

use rand::SeedableRng;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

use ensemble_kalman_filter::*;
use read_and_write::*;
use lorenz96::*;

const S: u64 = 2039;
const SYSTEM_NOISE_VARIANCE: f64 = 4.0;
const OBSERVATION_NOISE_VARIANCE: f64 = 9.0;

fn warn() {
    println!("You need 3 args. Input observation file path and Two Output file path.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path: &str;
    let ensemble_members_variance_output_path: &str;
    let result_output_path: &str;
    match args.len() {
        4 => {
            input_path = &args[1];
            ensemble_members_variance_output_path = &args[2];
            result_output_path = &args[3];
        }
        _ => {
            warn();
            return;
        }
    }

    let x = Array::zeros((40, 300));
    let v = Array::from_elem((40, 300), 1.0f64);
    let w = Array::from_elem((40, 300), 1.0_f64);
    // let h = Array::eye(40);

    // let first_half = Array::ones(20);
    // let last_half = Array::zeros(20);
    // let h =
    //    Array::from_diag(&concatenate(Axis(0), &[first_half.view(), last_half.view()]).unwrap());

    let mut d = Vec::with_capacity(40);
    for i in 0..40 {
        if i%2 == 0 {
            d.push(1f64);
        } else {
            d.push(0f64);
        }
    }
    let h = Array::from_diag(&arr1(&d));

    let traj = read_trajectory_file(input_path);
    let mut ensemble_members_variance_output_buf =
        match File::create(ensemble_members_variance_output_path) {
            Ok(inner) => BufWriter::new(inner),
            Err(_) => panic!("cannot create: {}", ensemble_members_variance_output_path),
        };
    let mut result_output_buf = match File::create(result_output_path) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create: {}", result_output_path),
    };

    let mut enkf = EnsembleKalmanFilter {
        member: EnsembleKalmanFilterMember { x, v, w, h },
        observation_data: traj,
        observation_span: 0.2f64,
        simulation_time_length: 100f64,
        rng: Xoshiro256StarStar::seed_from_u64(S),
        predict_distribution: Normal::new(0.0, SYSTEM_NOISE_VARIANCE.sqrt()).unwrap(),
        filter_distribution: Normal::new(0.0, OBSERVATION_NOISE_VARIANCE.sqrt()).unwrap(),
    };

    enkf.run_and_write_with_ensemble_members_variance(
        &mut result_output_buf,
        &mut ensemble_members_variance_output_buf,
        |a, b| {
            let mut ret = Array::zeros((a.nrows(), a.ncols()));
            for i in 0..a.ncols() {
                let mut s = System{ x : a.column(i).to_owned() };
                s.step();
                ret.column_mut(i).assign(&s.x);
            }
            ret + b
        },
    );
}
