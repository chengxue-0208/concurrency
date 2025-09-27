use anyhow::Result;
use concurrency::metrics::MetricsKD;

fn main() -> Result<()> {
    let mut metrics = MetricsKD::new();
    metrics.inc("req.page.1");
    metrics.inc("call.thread.worker.1");
    metrics.dec("b");
    println!("{:?}", metrics.snapshot());
    Ok(())
}
