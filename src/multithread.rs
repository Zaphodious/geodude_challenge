use rayon::prelude::*;
use fastrand;

const TURNS: usize = 231;
//const OPS_PER_BYTE: usize = 4;
const OPS_PER_BYTE: usize = 7;
const NOBYTES: usize = TURNS/OPS_PER_BYTE;

pub fn with_multithreading(rounds: usize, bar_incrementer: impl Fn(usize) + Send + Sync) -> Option<usize> {
    cpu_byte_rolling(rounds,bar_incrementer)
}

fn cpu_byte_rolling(rounds: usize, bar_incrementer: impl Fn(usize) + Send + Sync) -> Option<usize> {
    let done_per_tick = rounds / 1000;
    (1..rounds+1).into_par_iter().map(|i| {
        let mut roundvec: [u8; NOBYTES] = [0u8; NOBYTES];
        fastrand::fill(&mut roundvec);
        (i, roundvec)
    }).map(|(i, roundvec)| {

        //roundvec[0] = roundvec[0] & 0b00111111;

        // next to do- scan byte array for 11 bit pairs, return count
        let rollcount: usize = roundvec.iter().map(bitmask_d4).sum();
        if i > 1 && i % done_per_tick == 0 || i == rounds {
            //let current_progress = progress.fetch_add(1, Ordering::Relaxed) + 1;
            //println!("{current_progress}%");
            bar_incrementer(done_per_tick)
        }
        rollcount
    }).max()
}

fn bitmask_d4(byte: &u8) -> usize {
    let abits = (byte & 0b11000000 == 0b11000000) as usize;
    //let bbits = (byte & 0b01100000 == 0b01100000) as usize;
    let cbits = (byte & 0b00110000 == 0b00110000) as usize;
    //let dbits = (byte & 0b00011000 == 0b00011000) as usize;
    let ebits = (byte & 0b00001100 == 0b00001100) as usize;
    //let fbits = (byte & 0b00000110 == 0b00000110) as usize;
    let gbits = (byte & 0b00000011 == 0b00000011) as usize;
    //abits + bbits + cbits+ dbits + ebits + fbits + gbits
    abits + cbits + ebits + gbits
}