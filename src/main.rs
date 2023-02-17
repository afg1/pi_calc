use core::f32::consts::PI;
use random_number::random;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::thread::available_parallelism;

/// Run N samples to calculate an estimate for pi
fn run_samples(num_samples: &u32) -> f32 {
    let mut inside_count: f32 = 0.0;

    for _iter in 0..*num_samples {
        let x: f32 = random!();
        let y: f32 = random!();

        if (x * x + y * y).sqrt() < 1.0 {
            inside_count += 1.0;
        }
    }

    4.0 * (inside_count / (*num_samples as f32))
}

fn update_estimate(estimate: &Arc<Mutex<f32>>, thread_est: f32) {
    let mut pi = estimate.lock().unwrap();
    if *pi == 0.0 {
        *pi = thread_est;
    } else {
        *pi += thread_est;
        *pi /= 2.0;
    }
}


fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let num_samples: u32 = args[1].parse().unwrap();
    let num_threads: u8 = args[2].parse().unwrap();
    let threshold: f32 = args[3].parse().unwrap();

    let num_cpus = available_parallelism().unwrap().get();
    println!("{}", num_cpus);

    let estimate = Arc::new(Mutex::<f32>::new(0.0));
    let mut handles = vec![];
    for _ in 0..num_threads {
        let estimate = Arc::clone(&estimate);
        let handle = thread::spawn(move || {
            let thread_est = run_samples(&num_samples);
            update_estimate(&estimate, thread_est);
            // evaluate absolute error, run until we get below threshold
            while (*estimate.lock().unwrap() - PI).abs() > threshold {
                let thread_est = run_samples(&num_samples);
                update_estimate(&estimate, thread_est);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}  {}", now.elapsed().as_secs(), (*estimate.lock().unwrap() - PI).abs() );
}
