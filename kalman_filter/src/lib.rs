use std::io::BufWriter;
use std::fs::File;
use std::vec::*;

use ndarray::*;
use ndarray_linalg::*;

use read_and_write::*;


#[derive(Clone)]
pub struct KalmanFilterMember{
pub x: Array1<f64>,
pub v: Array2<f64>,
pub f: Array2<f64>,
pub g: Array2<f64>,
pub h: Array2<f64>,
pub q: Array2<f64>,
pub r: Array2<f64>
}

pub struct KalmanFilter {
    pub member: KalmanFilterMember,
    pub observation_data: Vec<(f64,Array1<f64>)>,
    pub observation_span:f64,
    pub simulation_time_length:f64
}

impl KalmanFilter {
    const ΔT: f64 = 0.01;

    pub fn predict(&mut self) -> () {
        let mut next_member = self.member.clone();
        next_member.x = self.member.f.dot(&self.member.x);
        next_member.v = self.member.f.dot(&self.member.v).dot(&self.member.f.t())
            + self.member.g.dot(&self.member.q).dot(&self.member.g.t());
        self.member = next_member;
    }

    pub fn filter(&mut self, observation_index: usize) -> () {
        let y = &self.observation_data[observation_index].1;
        let k = self.member.v.dot(&self.member.h.t()).dot(
            &(&self.member.h.dot(&self.member.v).dot(&self.member.h.t()) + &self.member.r).inv().unwrap());
        let mut next_member = self.member.clone();
        next_member.x = &self.member.x + &k.dot(&(y - &self.member.h.dot(&self.member.x)));
        next_member.v = &self.member.v - &k.dot(&self.member.h.dot(&self.member.v));
        self.member = next_member;
    }
    
    pub fn run_and_write(&mut self, output_buf:&mut BufWriter<File>) -> () {
        let mut observation_index = 0;
        let mut predict_time_length = 0.0;
        for loop_index in 0..(self.simulation_time_length / Self::ΔT) as usize {
            self.predict();
            if predict_time_length >= self.observation_span {
                predict_time_length = 0.0;
                let observation_datum = &self.observation_data[observation_index];
                let observation_time = observation_datum.0;
                if (observation_time as f64 - loop_index as f64 * Self::ΔT).abs() > Self::ΔT / 10.0 {
                    panic!("inconsistent observation datum");
                }
                self.filter(observation_index);
                observation_index += 1;
            }
            predict_time_length += Self::ΔT;
            write_x(output_buf, loop_index as f64 * Self::ΔT, &self.member.x);
        }
        
    }
}
