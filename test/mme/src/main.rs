#![feature(box_syntax)]
extern crate netbricks;
use mme::mme;
use netbricks::config::{basic_opts, read_matches};
use netbricks::interface::*;
use netbricks::operators::*;
use netbricks::scheduler::*;
use std::env;
use std::fmt::Display;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
mod mme;

const TEST_DUR_SECS: u64 = 2;

fn test<T, S>(ports: Vec<T>, sched: &mut S) where
    T: PacketRx + PacketTx + Display + 'static,
    S: Scheduler + Sized
{
    println!("[MME test] receiving started...");
    let pipelines: Vec<T> = ports
        .iter()
        .map(|port| mme(ReceiveBatch::new(port.clone()), sched)
        .send(port.clone()))
        .collect();

    println!("[MME test] running {} pipelines...", pipelines.len());
    for pipeline in pipelines {
        sched.add_task(pipeline).unwrap();
    }
}

fn main() {
    // parse command-line arguments
    let opts = basic_opts();
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    let configuration = read_matches(&matches, &opts);
    // build netbricks configuration and context
    let configuration = read_matches(&matches, &opts);
    match initialize_system(&configuration) {
        Ok(mut context) => {
            context.start_schedulers();
            context.add_pipeline_to_run(Arc::new(move |p, s: &mut StandaloneScheduler| test(p, s)));
            context.execute();

            // run test
            loop {
                thread::sleep(Duration::from_secs(TEST_DUR_SECS));
            }
        },
        Err(e) => {
            panic!(e.to_string());
        }
    }
}