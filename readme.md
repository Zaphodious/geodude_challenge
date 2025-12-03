# Geodude Softlock Paralysis Simulator

This repo is my answer to the challenge from (Austin's Graveler Soft Lock video)[https://www.youtube.com/watch?v=M8C8dHQE2Ro]. It was released last year. I'm a bit late to the party. But, it nerd sniped me hard, so here it is.

## How to run

Install Rust if you haven't. Then, from the root of the repo, enter `cargo run --release`. If you want to change the number of simulations, its `cargo run --release -- -s <number>`. If you want to run via the (somewhat janky and not optimized) GPU-compute version, its `cargo run --release -- -g`. 

On my machine (Ryzen 9 3900XT), running one billion simulations via the CPU takes roughly 3.5 seconds. That's a little bit lower then Austin's 12 days.
