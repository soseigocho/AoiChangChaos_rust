use std::f64::consts::*;
use std::fs::File;
use std::io::BufWriter;
use std::vec::*;

use ndarray::prelude::*;
use ndarray_linalg::*;
use rand::distributions::*;
use rand::seq::SliceRandom;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256StarStar;

use read_and_write::*;

pub struct MergingParticleFilterMember {
    pub x: Array2<f64>,
    pub v: Array2<f64>,
    pub h: Array2<f64>,
    pub r: Array2<f64>,
}

pub struct MergingParticleFilter {
    pub member: MergingParticleFilterMember,
    pub observation_data: Vec<(f64, Array1<f64>)>,
    pub observation_span: f64,
    pub simulation_time_length: f64,
    pub merge_magnification: usize,
    pub merge_alpha: Vec<f64>,
    pub rng: Xoshiro256StarStar,
    pub predict_distribution: Normal<f64>,
}

impl MergingParticleFilter {
    const ΔT: f64 = 0.01;

    fn update_v(&mut self) -> () {
        for row in 0..self.member.v.nrows() {
            for col in 0..self.member.v.ncols() {
                self.member.v[[row,col]] = self.predict_distribution.sample(&mut self.rng);
            }
        }
    }

    fn predict<F>(&mut self, f: F) -> ()
    where
        F: Fn(&Array2<f64>, &Array2<f64>) -> Array2<f64>,
    {
        self.update_v();
        self.member.x = f(&self.member.x, &self.member.v);
    }

    fn calc_log_likelohood(&self, particle_index: usize, y: &Array1<f64>) -> f64 {
        let x = &self.member.x.column(particle_index);
        let term1 = (1.0_f64
            / ((2.0_f64 * PI).powf(y.len() as f64) * &self.member.r.det().unwrap()).sqrt())
        .ln();
        let diff = y - &self.member.h.dot(x);
        let term2 = -0.5_f64 * diff.t().dot(&self.member.r.inv().unwrap()).dot(&diff);
        term1 + term2
    }

    fn calc_betas(&self, y: &Array1<f64>) -> Vec<f64> {
        let log_likelihoods = (0..self.member.x.ncols())
            .map(|particle_index| self.calc_log_likelohood(particle_index, y))
            .collect::<Vec<f64>>();

        let max_log_likelihood = log_likelihoods
            .iter()
            .fold(0.0_f64 / 0.0_f64, |m, v| v.max(m));
        let ψs = log_likelihoods
            .iter()
            .map(|log_likelihood| (log_likelihood - max_log_likelihood).exp())
            .collect::<Vec<f64>>();

        let sum_ψ = ψs.iter().sum::<f64>();
        let βs = ψs.iter().map(|ψ| ψ / sum_ψ).collect();
        βs
    }

    fn choose_new_filters(&mut self, βs: &Vec<f64>) {
        let ms = βs.iter().map(|β| β.floor()).collect::<Vec<f64>>();

        let mut new_filter = Vec::new();
        for particle_index in 0..self.member.x.ncols() {
            let selected_num = self.member.x.ncols() * ms[particle_index] as usize;
            for _dependency in 0..selected_num {
                new_filter.push(self.member.x.column(particle_index).to_owned());
            }
        }

        let sum_m = ms.iter().sum::<f64>();
        let rest_probability = (0..self.member.x.ncols())
            .map(|particle_index| {
                (self.member.x.ncols() as f64 * βs[particle_index] - ms[particle_index])
                    / (self.member.x.ncols() as f64 - sum_m)
            })
            .collect::<Vec<f64>>();

        let rest_accum_probability = rest_probability
            .iter()
            .scan(0f64, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<f64>>();

        let distribution = Uniform::new(0.0_f64, rest_accum_probability.last().unwrap());
        while new_filter.len() != self.member.x.ncols() {
            let random_indicator = distribution.sample(&mut self.rng);
            let mut selected = 0;
            while random_indicator > rest_accum_probability[selected] {
                selected += 1;
            }
            new_filter.push(self.member.x.column(selected).to_owned());
        }

        for particle_index in 0..self.member.x.ncols() {
            self.member
                .x
                .column_mut(particle_index)
                .assign(&new_filter[particle_index]);
        }
    }

    fn merge_filter(&mut self) {
        let mut merge_indexes = Vec::with_capacity(self.member.x.ncols());
        for particle_index in 0..self.member.x.ncols() {
            for _merge_magnification_index in 0..self.merge_magnification {
                merge_indexes.push(particle_index);
            }
        }
        merge_indexes.shuffle(&mut self.rng);

        let old_x = self.member.x.clone();
        for particle_index in 0..self.member.x.ncols() {
            self.member.x.column_mut(particle_index).fill(0.0_f64);
            for merge_magnification_index in 0..self.merge_magnification {
                let new_xpi = &self.member.x.column(particle_index)
                    + &(self.merge_alpha[merge_magnification_index]
                        * &old_x.column(merge_indexes[particle_index + merge_magnification_index]));
                self.member.x.column_mut(particle_index).assign(&new_xpi);
            }
        }
    }

    fn filter(&mut self, observation_index: usize) {
        let y = &self.observation_data[observation_index].1;
        let βs = &self.calc_betas(y);
        self.choose_new_filters(βs);
        self.merge_filter();
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
