use std::fs::File;
use std::io::BufWriter;

use ndarray::Array;
use ndarray::*;

use read_and_write::*;

pub struct System {
    pub x: Array1<f64>,
}

impl System {
    const ΔT: f64 = 0.01f64;
    const DIMENSION_SIZE: usize = 40usize;
    const F: f64 = 8.0f64;

    pub fn new(center: f64, perturbation: f64, all_perturbation: f64) -> System {
        let mut x = Array::from_elem(Self::DIMENSION_SIZE, center + all_perturbation);
        x[0] += perturbation;
        System { x }
    }

    pub fn run_and_write(&mut self, output_buf: &mut BufWriter<File>, loop_num: usize) {
        for loop_index in 1..=loop_num {
            self.step();
            write_x(output_buf, loop_index as f64 * Self::ΔT, &self.x);
        }
    }

    pub fn step(&mut self) {
        self.runge_kutta_44();
    }

    fn dx_dt_for_one_dim(
        x: &Array1<f64>,
        dim_idx: usize,
        dim_idx_p1: usize,
        dim_idx_m1: usize,
        dim_idx_m2: usize,
    ) -> f64 {
        (x[dim_idx_p1] - x[dim_idx_m2]) * x[dim_idx_m1] - x[dim_idx] + Self::F
    }

    fn calc_dx_dt(x: &Array1<f64>) -> Array1<f64> {
        let mut k: Array1<f64> = Array::zeros(Self::DIMENSION_SIZE);
        for dim_idx in 0..Self::DIMENSION_SIZE {
            let dim_idx_p1 = dim_idx + 1;
            let dim_idx_m1 = dim_idx as isize - 1;
            let dim_idx_m2 = dim_idx as isize - 2;

            k[dim_idx] = match dim_idx_p1 {
                1 => Self::dx_dt_for_one_dim(
                    x,
                    dim_idx,
                    dim_idx_p1,
                    Self::DIMENSION_SIZE - 1,
                    Self::DIMENSION_SIZE - 2,
                ),
                2 => Self::dx_dt_for_one_dim(
                    x,
                    dim_idx,
                    dim_idx_p1,
                    dim_idx_m1 as usize,
                    Self::DIMENSION_SIZE - 1,
                ),
                Self::DIMENSION_SIZE => {
                    Self::dx_dt_for_one_dim(x, dim_idx, 0, dim_idx_m1 as usize, dim_idx_m2 as usize)
                }
                _ => Self::dx_dt_for_one_dim(
                    x,
                    dim_idx,
                    dim_idx_p1,
                    dim_idx_m1 as usize,
                    dim_idx_m2 as usize,
                ),
            };
        }
        k
    }

    fn runge_kutta_44(&mut self) {
        let k1 = Self::calc_dx_dt(&self.x);
        let k2 = Self::calc_dx_dt(&(&self.x + &(Self::ΔT * &k1 / 2.0f64)));
        let k3 = Self::calc_dx_dt(&(&self.x + &(Self::ΔT * &k2 / 2.0f64)));
        let k4 = Self::calc_dx_dt(&(&self.x + &(Self::ΔT * &k3)));
        self.x += &((k1 + 2.0f64 * k2 + 2.0f64 * k3 + k4) * (Self::ΔT / 6.0f64));
    }
}
