use std::{iter::repeat_with, sync::atomic::{AtomicUsize, Ordering}, time::Instant};
use rayon::prelude::*;
use fastrand;

mod compute;

const TURNS: usize = 231;
//const OPS_PER_BYTE: usize = 4;
const OPS_PER_BYTE: usize = 7;
const NOBYTES: usize = (TURNS/OPS_PER_BYTE);
const TARGET: usize = 177;

fn main() {
    let now = Instant::now();
    let rounds = 1000000000;
    //let mastervec: Vec<u128> = repeat_with(|| fastrand::u128(..)).take((1000000000 / 64) * turns).collect();
    //let masterlength = mastervec.len();
    //println!("{masterlength}");
    let progress = AtomicUsize::new(0);
    //let maybe_result = cpu_byte_rolling(rounds, progress);
    let maybe_result = parallel_gen_u32(rounds, progress);
    let reslen = maybe_result.len();
    let elapsed = now.elapsed();
    /*
    if let Some(result) = maybe_result {
        println!("The maximum turns that paralysis proc'd during the battle was {result}. The simulation took {:.2?}", elapsed);
    } else {
        println!("Somehow, Mewtwo returned. And then everything went weird.");
    }
    */
    println!("length is {reslen}, took {:.2?}", elapsed);
}

fn just_generate_u32s(rounds: usize, progress: AtomicUsize) -> Option<u32> {
    let mut u: u32 = 0;
    for i in (0..rounds) {
        u = fastrand::u32(0..u32::MAX);
    }
    Some(u)
}

fn parallel_gen_u32(rounds: usize, progress: AtomicUsize) -> Vec<u128> {
    (1..rounds).into_par_iter().map(|i| {
                fastrand::u128(0..u128::MAX)
            }).collect()
}

fn cpu_byte_rolling(rounds: usize, progress: AtomicUsize) -> Option<usize> {
    (1..rounds+1).into_par_iter().map(|i| {
        let mut roundvec: [u8; NOBYTES] = [0u8; NOBYTES];
        fastrand::fill(&mut roundvec);
        if i % 10000000 == 0 {
            let current_progress = progress.fetch_add(1, Ordering::Relaxed) + 1;
            println!("{current_progress}%");
        }
        roundvec
    }).map(|roundvec| {

        //roundvec[0] = roundvec[0] & 0b00111111;

        // next to do- scan byte array for 11 bit pairs, return count
        let rollcount: usize = roundvec.iter().map(|b| {
            let abits = (b & 0b11000000 == 0b11000000) as usize;
            let bbits = (b & 0b01100000 == 0b01100000) as usize;
            let cbits = (b & 0b00110000 == 0b00110000) as usize;
            let dbits = (b & 0b00011000 == 0b00011000) as usize;
            let ebits = (b & 0b00001100 == 0b00001100) as usize;
            let fbits = (b & 0b00000110 == 0b00000110) as usize;
            let gbits = (b & 0b00000011 == 0b00000011) as usize;
            abits + bbits + cbits+ dbits + ebits + fbits + gbits
            //abits + cbits + ebits + gbits
        }).sum();
        rollcount
    }).max()
}

