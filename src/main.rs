use std::{time::Instant};
use indicatif::{ProgressBar, ProgressStyle};
use clap::Parser;
use colored::Colorize;

mod gpucompute;
mod multithread;

const TARGET_PROCS: usize = 177;

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
    let now = Instant::now();
    let args = Args::parse();

    let algostring = if args.gpu {"the GPU".green()} else {"the CPU".blue()} ;

    println!("\nRunning {} using {algostring}", "battle simulations".yellow() );

    let bar = ProgressBar::new(args.simulations as u64);
    bar.set_style(ProgressStyle::with_template("Simulation {human_pos}/{human_len} \n{per_sec} simulations per second (eta: {eta}) \n[{wide_bar:.yellow/red}]\n").unwrap().progress_chars("━►░"));

    let incfn = |amt: usize| {
        bar.inc(amt as u64);
    };

    
    let result = if args.gpu {
        gpucompute::run_on_gpu(args.simulations, incfn).ok()
    } else {
        multithread::with_multithreading(args.simulations, incfn)
    };

    bar.finish();

    if let Some(successes) = result {
        let elapsed = now.elapsed();
        let softlock_picked = successes >= TARGET_PROCS;
        let formatted_successes = if softlock_picked {
            successes.to_string().green()
        } else {
            successes.to_string().red()
        };
        let formatted_time = format!("{:.2?} seconds", elapsed).purple();
        println!("\n\nIn the simulations, the most that paralysis proc'd was {formatted_successes} times.");
        if softlock_picked {
            println!("Graveler {} in at least one simulation!", "won".green());
        } else {
            println!("Graveler, unfortunately, {} in every simulation", "exploded".red());
        }
        println!("These simulations took {formatted_time} to complete\n");
    } else {
        println!("Somehow, Mewtwo returned.")
    }

}
