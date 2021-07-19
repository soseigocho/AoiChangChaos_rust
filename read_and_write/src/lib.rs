use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Write};
use std::vec::*;

use ndarray::*;

pub fn read_observation_file(observation_file_path: &str) -> (Vec<(f64, Array1<f64>)>, f64, f64) {
    let observation_file = match File::open(observation_file_path) {
        Ok(x) => BufReader::new(x),
        Err(_) => panic!("cannot open:{}", observation_file_path),
    };

    let mut observation_span: f64 = 0.0;
    let mut observation_time_length: f64 = 0.0;
    let mut observation_data: Vec<(f64, Array1<f64>)> = Vec::new();
    for result in observation_file.lines() {
        match result {
            Ok(line) => {
                let words: Vec<&str> = line.split(' ').collect();
                if words[0] == "time_span" {
                    observation_span = words[1].parse().ok().unwrap();
                } else if words[0] == "simulation_time" {
                    observation_time_length = words[1].parse().ok().unwrap();
                } else {
                    let t: f64 = words[0].parse().ok().unwrap();
                    let mut obs_data = Vec::<f64>::new();
                    for obs_idx in 1..words.len() {
                        obs_data.push(words[obs_idx].parse().ok().unwrap());
                    }
                    let y = arr1(&obs_data);
                    observation_data.push((t, y));
                }
            }
            Err(_) => (),
        }
    }

    (observation_data, observation_span, observation_time_length)
}

pub fn write_x(buf: &mut BufWriter<File>, t: f64, x: &Array1<f64>) {
    write!(buf, "{}", t);
    for xi in x.iter() {
        write!(buf, " {}", xi);
    }
    write!(buf, "\n");
}

pub fn read_trajectory_file(input_path: &str) -> Vec<(f64,Array1<f64>)> {
    let input_buf = match File::open(input_path) {
        Ok(inner) => BufReader::new(inner),
        Err(_) => panic!("cannot open {}", input_path),
    };
    let mut times_and_vecs = Vec::new();
    for result in input_buf.lines() {
        match result {
            Ok(line) => {
                let words: Vec<&str> = line.split(' ').collect();
                let t = words[0].parse::<f64>().ok().unwrap();
                let v: Array1<f64> = arr1(
                    &words[1..]
                        .iter()
                        .map(|x| x.parse().ok().unwrap())
                        .collect::<Vec<f64>>(),
                );
                times_and_vecs.push((t,v));
            }
            Err(_) => (),
        }
    }
    times_and_vecs
}
