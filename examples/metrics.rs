use anyhow::{Error, Result};
use concurrency::metrics::MetricsKD;
use std::{ thread, time::Duration};
const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = MetricsKD::new();
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }
    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}", metrics);
    }   
   
}


fn task_worker(idx: usize, metrics:  MetricsKD) -> Result<()> {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(rand::random_range(1..5)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    });
    Ok(())
}

fn request_worker(metrics:  MetricsKD) -> Result<()> {
    thread::spawn(move || {
        loop {
        thread::sleep(Duration::from_secs(rand::random_range(1..5)));
        let page = rand::random_range(1..256);
        metrics.inc(format!("req.page.{}", page)).unwrap();
        }

    });

    
    Ok(())
}