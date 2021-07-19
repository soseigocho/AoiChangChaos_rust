use std::fs::File;
use std::io::BufWriter;

use lorenz96;

fn warn() {
    println!("You need an argument. Output File name and Paht.");
}

fn main() {
    let args:Vec<String> = std::env::args().collect();

    let output_path = match args.len() {
        2 => &args[1],
        _ => {
            warn();
            return
        }
    };

    let mut output_buf = match File::create(output_path) {
        Ok(inner) => BufWriter::new(inner),
        Err(_) =>  panic!("cannot create: {}", output_path)
    };

    let loop_num: usize = 10000;
    let center: f64 = 0.0f64;
    let perturbation: f64 = 1e-3f64;
    let all_perturbation: f64 = 0.0f64;
    let mut system = lorenz96::System::new(center, perturbation, all_perturbation);
    system.run_and_write(&mut output_buf, loop_num);
}