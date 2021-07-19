use std::fs::File;
use std::io::{BufWriter, Write};

use ndarray::*;

use read_and_write;

fn warn() {
    println!("You need 4 args. Input two trajectory file path, Output file path and the flag if 2nd trajectory file is estimated.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let true_trajectory_path: &str;
    let estimate_trajectory_path: &str;
    let output_path: &str;
    let is_estimated: bool;
    match args.len() {
        5 => {
            true_trajectory_path = &args[1];
            estimate_trajectory_path = &args[2];
            output_path = &args[3];
            is_estimated = &args[4] == "estimated";
        }
        _ => {
            warn();
            return;
        }
    }

    let y_true = read_and_write::read_trajectory_file(true_trajectory_path);
    let y_estimate = read_and_write::read_trajectory_file(estimate_trajectory_path);

    let length = y_true.len();
    let dimension = y_true[0].1.len();
    let mut result: Array1<f64> = Array::zeros(dimension);
    if is_estimated {
        for i in 0..length {
            result += &(&y_true[i].1 - &y_estimate[i + 1].1).map(|x| x.powf(2f64));
        }
    } else {
        for i in 0..length {
            result += &(&y_true[i].1 - &y_estimate[i].1).map(|x| x.powf(2f64));
        }
    }
    result = (result / length as f64).map(|x| x.sqrt());

    let mut output_buf = match File::create(output_path) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create {}", output_path),
    };
    for x_d in result.iter() {
        writeln!(&mut output_buf, "{}", x_d);
    }
}
