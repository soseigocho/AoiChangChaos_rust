use std::fs::File;
use std::io::{BufWriter, Write};

use read_and_write;

fn warn() {
    println!("You need 3 args. Input two trajectory file path and Output file path.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let original_trajectory_path: &str;
    let delta_trajectory_path: &str;
    let output_path: &str;
    match args.len() {
        4 => {
            original_trajectory_path = &args[1];
            delta_trajectory_path = &args[2];
            output_path = &args[3];
        }
        _ => {
            warn();
            return;
        }
    }

    let orig = read_and_write::read_trajectory_file(original_trajectory_path);
    let delt = read_and_write::read_trajectory_file(delta_trajectory_path);

    const ΔT: f64 = 0.01f64;
    let tgt_dim_idx = 0;

    let mut output_buf = match File::create(output_path) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create {}", output_path),
    };
    for t_diff_idx in 1..1000 {
        let t_diff = t_diff_idx as f64 / 10f64;
        let t2_idx = (t_diff / ΔT) as usize;

        let orig_t1 = orig[0].1[tgt_dim_idx];
        let delt_t1 = delt[0].1[tgt_dim_idx];
        let diff_t1 = (orig_t1 - delt_t1).abs();

        let orig_t2 = orig[t2_idx].1[tgt_dim_idx];
        let delt_t2 = delt[t2_idx].1[tgt_dim_idx];
        let diff_t2 = (orig_t2 - delt_t2).abs();

        let lyapunov_exponent = (1.0f64 / t_diff ) * (diff_t2 / diff_t1).ln();

        writeln!(&mut output_buf, "{} {}", t_diff, lyapunov_exponent);
    }
}
