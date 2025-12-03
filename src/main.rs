use std::{time::Instant};
use indicatif::{ProgressBar, ProgressStyle};
use clap::Parser;
use colored::Colorize;

mod gpucompute;
mod multithread;

const TARGET_PROCS: usize = 177;

// This program allows the user to change the number of simulations, and to run
// either on the CPU or GPU
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Use GPU compute to perform the simulations, rather
    /// then multithreading. 
    #[arg(short, long, default_value_t = false)]
    gpu: bool,

    /// The number of simulations to run. 
    #[arg(short, long, default_value_t = 1_000_000_000)]
    simulations: usize,
}


fn main() {
    // To time the operation, we get the current time, and then compare
    // it to the time that it is after the operation is complete
    let now = Instant::now();

    let args = Args::parse();

    // Do some fancy printing for the user
    let algostring = if args.gpu {"the GPU".green()} else {"the CPU".blue()} ;

    println!("Hello <user:Austin>");
    println!("Please input simulation parameters: ...");
    println!("Simulation parameters received.\n");

    println!("Running simulation {} using {algostring}", "\"Graveler's Stall Party\"".yellow());

    // The status bar is a bit extra, but this is about flexing
    let bar = ProgressBar::new(args.simulations as u64);
    bar.set_style(ProgressStyle::with_template("Simulation {human_pos}/{human_len} \n{per_sec} simulations per second (eta: {eta}) \n[{wide_bar:.yellow/red}]\n").unwrap().progress_chars("━►░"));

    // This allows us to let each version of the program update
    // the progress bar without having to know what kind of progress
    // bar we're using
    let incfn = |amt: usize| {
        bar.inc(amt as u64);
    };

    // This is where the actual operation is ran
    let result = if args.gpu {
        gpucompute::run_on_gpu(args.simulations, incfn).ok()
    } else {
        // Note- this is by far the better one, and the only one that I've commented
        multithread::with_multithreading(args.simulations, incfn)
    };

    // Once the progress bar is done, the docs advise us to explicitely close it
    bar.finish();

    // If the operation found a result (which should be always)
    // unless something very strange happened, we report it
    // to the user here
    if let Some(successes) = result {

        let softlock_picked = successes >= TARGET_PROCS;

        let formatted_successes = if softlock_picked {
            successes.to_string().green()
        } else {
            successes.to_string().red()
        };

        println!("\n\nIn the simulations, the most that paralysis proc'd was {formatted_successes} times.");

        if softlock_picked {
            println!("Graveler {} in at least one simulation!", "won".green());
        } else {
            println!("Graveler, unfortunately, {} in every simulation", "exploded".red());
        }

        let elapsed = now.elapsed();
        let formatted_time = format!("{:.2?} seconds", elapsed).purple();
        println!("These simulations took {formatted_time} to complete\n");
    } else {
        // If something went wrong, we assume that Mewtwo did it
        println!("Somehow, Mewtwo returned.")
    }

}
