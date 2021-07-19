use std::fs::File;
use std::io::BufWriter;

use ndarray::*;
use ndarray::Array;

use kalman_filter::*;
use read_and_write::*;

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

    let x = Array::zeros(1);
    let v = Array::eye(1);
    let f = Array::eye(1);
    let g = Array::eye(1);
    let h = Array::eye(1);
    let q = Array::eye(1);
    let r = arr2(&[[10.0]]);

    let (observation_data, observation_span, simulation_time_length) = read_observation_file(input);
    let mut buf = match File::create(output) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) => panic!("cannot create: {}", output)
    };
    let mut kf = KalmanFilter{
        member : KalmanFilterMember{x,v,f,g,h,q,r},
        observation_data,
        observation_span,
        simulation_time_length};

    kf.run_and_write(&mut buf);
}