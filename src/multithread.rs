
use rayon::prelude::*;
use fastrand;

const TURNS: usize = 231;
const OPS_PER_BYTE: usize = 4;
const NOBYTES: usize = ((TURNS as f32)/(OPS_PER_BYTE as f32)).ceil() as usize;

pub fn with_multithreading(rounds: usize, bar_incrementer: impl Fn(usize) + Send + Sync) -> Option<usize> {
    cpu_byte_rolling(rounds,bar_incrementer)
}

fn cpu_byte_rolling(rounds: usize, bar_incrementer: impl Fn(usize) + Send + Sync) -> Option<usize> {

    // To make it more exciting for the user, we want to report
    // progress 1000 times during the computation. 
    let done_per_tick = rounds / 1000;

    // Using multiple threads, get one billion vectors of random bytes
    (1..rounds+1).into_par_iter().map(|i| {
        let mut roundvec: [u8; NOBYTES] = [0u8; NOBYTES];
        fastrand::fill(&mut roundvec);
        (i, roundvec)
    // and then find out how many results are represented
    }).map(|(i, mut roundvec)| {

        // 231/4 remainer is 3, so we adjust the first byte so that
        // we only get three dice rolls out of it.
        roundvec[0] = roundvec[0] & 0b00111111;

        // Do the counting here
        let rollcount: usize = roundvec.iter().map(bitmask_d4).sum();
        
        // If we are on one of the steps where we should report back to the user,
        // we do so
        if i > 1 && i % done_per_tick == 0 || i == rounds {
            bar_incrementer(done_per_tick)
        }

        rollcount
    }).max()
}

// For each byte, bitmask it four times to find out of
// a pair of bits in the byte are '11', which is 3 out of
// 0,1,2,3. This represents a successful dice roll, and we
// roll four dice per byte.
fn bitmask_d4(byte: &u8) -> usize {
    let abits = (byte & 0b11000000 == 0b11000000) as usize;
    let bbits = (byte & 0b00110000 == 0b00110000) as usize;
    let cbits = (byte & 0b00001100 == 0b00001100) as usize;
    let dbits = (byte & 0b00000011 == 0b00000011) as usize;
    abits + bbits + cbits + dbits
}