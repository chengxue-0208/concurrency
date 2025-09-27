use anyhow::{Ok, Result};
use std::{sync::mpsc, thread, time::Duration};

const N: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}
impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
fn main() -> Result<()> {

    let (tx, rx) = mpsc::channel();
    for i in 0..N {
      let tx = tx.clone();
        
        thread::spawn(move || producer(i,tx));
    }
    drop(tx);
    //创建CONSUMER线程
    let consumer =  thread::spawn(move || {
        for msg in rx {
            println!("{:?}", msg);
        }
        println!("consumer线程退出");
        666
    });

    let secret = consumer
    .join()
    .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    println!("{:?}", secret);
    
    //  thread::sleep(Duration::from_millis(10000));


    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) ->Result<()> {
    
    loop{
       let value = rand::random::<u32>() as usize;
       let msg = Msg::new(idx, value);
       tx.send(msg)?;
       thread::sleep(Duration::from_millis(1000));
       if rand::random::<u32>() as usize % 2 == 0 {
        println!("{}线程退出", idx);
           break;
       }
       

    }
    Ok(())
}