//metrics data structure

//基本功能  inc  dec  snapshot

use std::{
    clone::Clone,
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},

};
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt;
#[derive(Debug, Clone)]
pub struct MetricsKD {
    data: Arc<RwLock<HashMap<String, i64>>>,
}
impl MetricsKD {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow::anyhow!("{:?}", e))?;
        *data.entry(key.into()).or_insert(0) += 1;
        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow::anyhow!("{:?}", e))?;
        *data.entry(key.into()).or_insert(0) -= 1;
        Ok(())
    }
    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        let data = self.data.read().map_err(|e| anyhow::anyhow!("{:?}", e))?;       
        Ok(data.clone())
    }
}

impl Display for MetricsKD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let data = self.data.read().map_err(|e| fmt::Error)?;  
        //let data = self.snapshot().unwrap();
        for (k,v) in data.iter() {
            writeln!(f, "{}: {}", k, v)?;
        }
        Ok(())
    }
}