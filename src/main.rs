use std::{iter::repeat_with, sync::atomic::{AtomicUsize, Ordering}, time::Instant};
use rayon::prelude::*;
use fastrand;

const TURNS: usize = 231;
const OPS_PER_BYTE: usize = 4;
//const OPS_PER_BYTE: usize = 7;
const NOBYTES: usize = (TURNS/OPS_PER_BYTE);
const TARGET: usize = 177;

fn main() {
    let now = Instant::now();
    let rounds = 1000000000;
    //let mastervec: Vec<u128> = repeat_with(|| fastrand::u128(..)).take((1000000000 / 64) * turns).collect();
    //let masterlength = mastervec.len();
    //println!("{masterlength}");
    let progress = AtomicUsize::new(0);
    let maybe_result  = (0..rounds).into_par_iter().map(|i| {
        let mut roundvec: [u8; NOBYTES] = [0u8; NOBYTES];
        fastrand::fill(&mut roundvec);

        roundvec[0] = roundvec[0] & 0b00111111;

        // next to do- scan byte array for 11 bit pairs, return count
        let rollcount: usize = roundvec.iter().map(|b| {
            let abits = (b & 0b11000000 == 0b11000000) as usize;
            //let bbits = (b & 0b01100000 == 0b01100000) as usize;
            let cbits = (b & 0b00110000 == 0b00110000) as usize;
            //let dbits = (b & 0b00011000 == 0b00011000) as usize;
            let ebits = (b & 0b00001100 == 0b00001100) as usize;
            //let fbits = (b & 0b00000110 == 0b00000110) as usize;
            let gbits = (b & 0b00000011 == 0b00000011) as usize;
            //abits + bbits + cbits+ dbits + ebits + fbits + gbits
            abits + cbits + ebits + gbits
        }).sum();
        if i % 10000000 == 0 {
            let current_progress = progress.fetch_add(1, Ordering::Relaxed) + 1;
            println!("{current_progress}%");
        }
        rollcount
    }).max();
    if let Some(result) = maybe_result {
        let elapsed = now.elapsed();
        println!("The maximum turns that paralysis proc'd during the battle was {result}. The simulation took {:.2?}", elapsed);
    } else {
        println!("Somehow, Mewtwo returned. And then everything went weird.");
    }
}
