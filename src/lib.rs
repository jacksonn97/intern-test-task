use sha256::digest;
use anyhow::Result;

use std::{num::NonZero, sync::{mpsc, Arc, Barrier}, thread};

const START_VALUE: u128 = 1;

// reading and writing in only one place
static mut TRAILING_ZEROES: usize = 0; // aka N
// read-only
static mut AMOUNT_OF_RESULTS: usize = 0; // aka F

pub struct Answer {
    pub number: u128,
    pub digest: String,
}

pub fn run() {
    if unsafe { AMOUNT_OF_RESULTS } == 0 {
        return;
    }

    let cpus: usize = thread::available_parallelism()
        .unwrap_or(NonZero::new(1).unwrap())
        .into();
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(cpus);
    let (tx, rx) = mpsc::channel::<Answer>();
    let barrier = Arc::new(Barrier::new(cpus));
    let cpus: u128 = cpus as u128;

    for i in 0..cpus {
        let tx = tx.clone();
        let barrier = Arc::clone(&barrier);
        let start_from = START_VALUE + i;

        threads.push(thread::spawn(move || {
            let _ = find_hashes_by_step(tx, barrier, start_from, cpus).unwrap();
        }));
    }

    // "unsafe" keyword is lying! this code is absolutely safe!
    unsafe {
        while AMOUNT_OF_RESULTS != 0 {
            let a = rx.recv().unwrap();
            println!("{}, \"{}\"", a.number, a.digest);
            AMOUNT_OF_RESULTS -= 1;
        }
    }
}

fn find_hashes_by_step(tx: mpsc::Sender<Answer>, barrier: Arc<Barrier>, mut start_from: u128, step: u128) -> Result<()> {
    let end: String;
    unsafe {
        end = "0".to_string().repeat(TRAILING_ZEROES);
    }

    loop {
        barrier.wait();
        let digest: String = digest(start_from.to_string());
        if digest.ends_with(&end) {
            tx.send(Answer {
                number: start_from,
                digest,
            })?;
        }

        start_from += step;
    }
}

pub fn set_statics(aor: usize, tz: usize) {
    unsafe {
        AMOUNT_OF_RESULTS = aor;
        TRAILING_ZEROES = tz;
    }
}
