#![feature(box_syntax)]
extern crate netbricks;
use nf::mme;
use netbricks::config::{basic_opts, read_matches};
use netbricks::interface::*;
use netbricks::operators::*;
use netbricks::scheduler::*;
use std::env;
use std::fmt::Display;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
mod nf;

const SLEEP_TIME: u64 = 2;

fn test<T, S>(ports: Vec<T>, sched: &mut S)
where
    T: PacketRx + PacketTx + Display + Clone + 'static,
    S: Scheduler + Sized,
{
    println!("[MME test] receiving started...");
    let pipelines: Vec<_> = ports
        .iter()
        .map(|port| mme(ReceiveBatch::new(port.clone())).send(port.clone()))
        .collect();

    println!("[MME test] running {} pipelines...", pipelines.len());
    for pipeline in pipelines {
        sched.add_task(pipeline).unwrap();
    }
}

fn main() {
    println!("[MME test] MAIN!");

    // parse command-line arguments
    let opts = basic_opts();
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    // build netbricks configuration and context
    let configuration = read_matches(&matches, &opts);

    println!("47");

    match initialize_system(&configuration) {
        Ok(mut context) => {
            println!("Context OK");
            context.start_schedulers();
            context.add_pipeline_to_run(Arc::new(move |p, s: &mut StandaloneScheduler| test(p, s)));
            context.execute();

            loop {
                println!("Loop!");
                thread::sleep(Duration::from_secs(SLEEP_TIME));
            }
        }
        Err(e) => {
            panic!(e.to_string());
        }
    }
}
