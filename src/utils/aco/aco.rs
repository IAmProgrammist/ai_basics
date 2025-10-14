use std::{error::Error, sync::{mpsc::channel, RwLock}};

use dioxus::prelude::spawn;
use threadpool::ThreadPool;

use crate::utils::{ACOConfig, ACOPaths, Ant, AntFactory};

pub fn aco(config: &ACOConfig, paths: &mut dyn ACOPaths) -> Result<(), Box<dyn Error>> {
    let field = RwLock::new(paths);
    
    let cpu_count = num_cpus::get_physical();
    let pool = ThreadPool::new(cpu_count);
    let (tx, rx) = channel::<Ant>();
    for ant in AntFactory::create_ants(config.ants) {
        let tx = tx.clone();
        pool.execute(move || {
            
        });
    }

    rx.iter().take(config.ants).

    Ok(())
}