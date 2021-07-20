use std::fs::File;
use std::io::BufWriter;

use ndarray::prelude::*;
use ndarray::Array;
use ndarray_linalg::*;

use rand::distributions::*;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

use read_and_write::*;

pub struct EnsembleKalmanFilterMember {
    pub x: Array2<f64>,
    pub v: Array2<f64>,
    pub w: Array2<f64>,
    pub h: Array2<f64>,
}

pub struct EnsembleKalmanFilter {
    pub member: EnsembleKalmanFilterMember,
    pub observation_data: Vec<(f64, Array1<f64>)>,
    pub observation_span: f64,
    pub simulation_time_length: f64,
    pub rng: Xoshiro256StarStar,
    pub predict_distribution: Normal<f64>,
    pub filter_distribution: Normal<f64>,
}

impl EnsembleKalmanFilter {
    const ΔT: f64 = 0.01;

    fn update_v(&mut self) -> () {
        for row in 0..self.member.v.nrows() {
            for col in 0..self.member.v.ncols() {
                self.member.v[[row, col]] = self.filter_distribution.sample(&mut self.rng);
            }
        }
    }

    pub fn predict<F>(&mut self, f: F) -> ()
    where
        F: Fn(&Array2<f64>, &Array2<f64>) -> Array2<f64>,
    {
        self.update_v();
        self.member.x = f(&self.member.x, &self.member.v);
    }

    fn update_w(&mut self) -> () {
        for row in 0..self.member.w.nrows() {
            for col in 0..self.member.w.ncols() {
                self.member.w[[row, col]] = self.filter_distribution.sample(&mut self.rng);
            }
        }
    }

    pub fn filter(&mut self, observation_index: usize) -> () {
        self.update_w();
        let yi = &self.observation_data[observation_index].1;
        let mut y = Array::zeros((yi.len(), self.member.x.ncols()));
        for ensemble_index in 0..self.member.x.ncols() {
            y.column_mut(ensemble_index).assign(yi);
        }
        let x_mean = self.member.x.mean_axis(Axis(1)).unwrap();
        let x_diff =
            Array::from_shape_fn((self.member.x.nrows(), self.member.x.ncols()), |(i, j)| {
                self.member.x[[i, j]] - x_mean[i]
            });

        let w_mean = self.member.w.mean_axis(Axis(1)).unwrap();
        let w_diff =
            Array::from_shape_fn((self.member.w.nrows(), self.member.w.ncols()), |(i, j)| {
                self.member.w[[i, j]] - w_mean[i]
            });
        let v_mean = x_diff.dot(&x_diff.t()) / (self.member.x.ncols() - 1) as f64;
        let r_mean = w_diff.dot(&w_diff.t()) / (self.member.w.ncols() - 1) as f64;

        let factor1 = v_mean.dot(&self.member.h.t());
        let factor2 = (self.member.h.dot(&v_mean).dot(&self.member.h.t()) + r_mean)
            .inv()
            .unwrap();
        let factor3 = y + w_diff - self.member.h.dot(&self.member.x);
        self.member.x = self.member.x.clone() + factor1.dot(&factor2).dot(&factor3);
    }

    pub fn run_and_write<F>(&mut self, output_buf: &mut BufWriter<File>, f: F) -> ()
    where
        F: Fn(&Array2<f64>, &Array2<f64>) -> Array2<f64> + Copy,
    {
        let mut predict_time_length: f64 = 0.0;
        let mut observation_index: usize = 0;
        for loop_index in 0..=(self.simulation_time_length / Self::ΔT) as usize {
            self.predict(f);
            if predict_time_length >= self.observation_span {
                predict_time_length = 0.0;
                let observation_datum = &self.observation_data[observation_index];
                let observation_time = observation_datum.0;
                if (observation_time as f64 - loop_index as f64 * Self::ΔT).abs() > Self::ΔT / 10.0
                {
                    panic!("inconsistent observation datum");
                }
                self.filter(observation_index);
                observation_index += 1;
            }
            predict_time_length += Self::ΔT;
            write_x(
                output_buf,
                loop_index as f64 * Self::ΔT,
                &self.member.x.mean_axis(Axis(1)).unwrap(),
            );
        }
    }

    pub fn run_and_write_with_ensemble_members_variance<F>(
        &mut self,
        x_buf: &mut BufWriter<File>,
        variance_buf: &mut BufWriter<File>,
        f: F,
    ) -> ()
    where
        F: Fn(&Array2<f64>, &Array2<f64>) -> Array2<f64> + Copy,
    {
        let mut predict_time_length: f64 = 0.0;
        // 時間スキップ幅を変える時は書き換えること 0.01-0 0.2-19
        let mut observation_index: usize = 19;
        for loop_index in 0..=(self.simulation_time_length / Self::ΔT) as usize {
            self.predict(f);
            if predict_time_length >= self.observation_span {
                predict_time_length = 0.0;
                let observation_datum = &self.observation_data[observation_index];
                let observation_time = observation_datum.0;
                if (observation_time as f64 - loop_index as f64 * Self::ΔT).abs() > Self::ΔT / 10.0
                {
                    panic!("inconsistent observation datum");
                }
                self.filter(observation_index);
                // 時間スキップ幅を変える時は書き換えること 0.01-1 0.2-20
                observation_index += 20;
            }
            predict_time_length += Self::ΔT;
            let time = loop_index as f64 * Self::ΔT;
            write_x(variance_buf, time, &self.member.x.var_axis(Axis(1), 0f64));
            write_x(x_buf, time, &self.member.x.mean_axis(Axis(1)).unwrap());
        }
    }
}
