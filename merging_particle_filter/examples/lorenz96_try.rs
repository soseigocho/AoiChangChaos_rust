use std::fs::File;
use std::io::BufWriter;

use ndarray::Array;

use rand::SeedableRng;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

use merging_particle_filter::*;
use read_and_write::*;

const S: u64 = 2039;
const SYSTEM_NOISE_VARIANCE: f64 = 4.0;
const OBSERVATION_NOISE_VARIANCE: f64 = 50.0;

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

    let x = Array::zeros((40, 1000));
    let v = Array::from_elem((40, 1000), 1.0f64);
    let h = Array::eye(40);
    // Rは観測ノイズの分散共分散行列
    let r = OBSERVATION_NOISE_VARIANCE * Array::eye(40);

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

    let mut mpf = MergingParticleFilter {
        member: MergingParticleFilterMember { x, v, h, r },
        observation_data: traj,
        observation_span: 0.01f64,
        simulation_time_length: 100f64,
        merge_magnification: 3,
        merge_alpha: vec![
            3.0_f64 / 4.0_f64,
            (13.0_f64.sqrt() + 1.0_f64) / 8.0_f64,
            -(13.0_f64.sqrt() - 1.0_f64) / 8.0_f64,
        ],
        rng: Xoshiro256StarStar::seed_from_u64(S),
        predict_distribution: Normal::new(0.0, SYSTEM_NOISE_VARIANCE.sqrt()).unwrap(),
    };

    mpf.run_and_write_with_ensemble_members_variance(
        &mut result_output_buf,
        &mut ensemble_members_variance_output_buf,
        |a, b| a + b,
    );
}
