#![deny(missing_docs)]
#![deny(deprecated)]
#![warn(missing_doc_code_examples)]

//! `chesspace` calculates timestamps to pace yourself in chess games.

use std::time::Duration;
use structopt::StructOpt;

/// Command line arguments
#[derive(StructOpt, Debug)]
#[structopt(about = "calculates timestamps to pace yourself in a chess game")]
struct Opt {
    /// Timecontrol start time in minutes
    minutes: u64,

    /// Increment in seconds
    increment: u64,

    /// Rounds to be played
    #[structopt(short, long, default_value = "40")]
    rounds: u32,

    /// lichess (doesn't apply increment at first round)
    #[structopt(short, long)]
    lichess: bool,

    /// Display every <display> round
    #[structopt(short, long, default_value = "1")]
    display: u32,

    /// Use <percentage> of time for the first <opening> rounds
    #[structopt(short, long)]
    percentage: Option<u32>,

    /// Use <percentage> of time for the first <opening> rounds (if skipped openings are played twice as fast)
    #[structopt(short, long)]
    opening: Option<u32>,
}

/// Extracts the minutes out of a Duration.
///
/// Returns the extracted minutes and remaining seconds as fraction.
fn break_duration_to_min(d: Duration) -> (i32, f64) {
    let minutes = (d.as_secs_f64() / 60.0) as i32;
    let seconds = d.as_secs_f64() - minutes as f64 * 60.0;
    (minutes, seconds)
}

fn main() {
    let opt = Opt::from_args();

    let start_time = Duration::new(opt.minutes * 60, 0);
    let increment = Duration::new(opt.increment, 0);
    let total_time = start_time
        + increment
            * if opt.lichess {
                opt.rounds - 1
            } else {
                opt.rounds
            };

    let opening_rounds: u32;
    let opening_time_per_round: Duration;
    let remaining_time_per_round: Duration;

    print!("timecontrol: {}+{}", opt.minutes, opt.increment);
    if opt.lichess {
        print!(" (lichess)");
    }
    println!();

    match (opt.opening, opt.percentage) {
        (Some(opening), Some(percentage)) => {
            opening_rounds = opening;
            let first_duration = total_time * percentage / 100;
            opening_time_per_round = first_duration / opening;
            remaining_time_per_round = (total_time - first_duration) / (opt.rounds - opening);
        }
        (Some(opening), None) => {
            opening_rounds = opening;
            // opening rounds are played twice as fast
            // total_time = opening_time_per_round * opening_rounds + (opt.rounds - opening_rounds) * 2 * opening_time_per_round
            opening_time_per_round = total_time / (2 * opt.rounds - opening_rounds);
            remaining_time_per_round = 2 * opening_time_per_round;
        }
        _ => {
            opening_rounds = 0;
            opening_time_per_round = Duration::ZERO;
            remaining_time_per_round = total_time / opt.rounds;
        }
    }

    let (total_minutes, total_seconds) = break_duration_to_min(total_time);
    if opening_rounds > 0 {
        println!("total time: {}:{:0>2}min", total_minutes, total_seconds);
        println!(
            "time per opening round: {:.1}s ({} rounds)",
            opening_time_per_round.as_secs_f32(),
            opening_rounds
        );
        println!(
            "time per remaining round: {:.1}s",
            remaining_time_per_round.as_secs_f32()
        );
    } else {
        println!("total time: {}:{:0>2}min", total_minutes, total_seconds);
        println!(
            "time per round: {:.1}s",
            remaining_time_per_round.as_secs_f32()
        );
    }

    let mut time_remaining = start_time;
    for i in 1..=opt.rounds {
        if !opt.lichess || i != 1 {
            time_remaining += increment;
        }

        time_remaining -= if i <= opening_rounds {
            opening_time_per_round
        } else {
            remaining_time_per_round
        };

        if i % opt.display == 0 {
            let (minutes, seconds) = break_duration_to_min(time_remaining);
            println!("{:>2}: {:>2}:{:0>2.0}", i, minutes, seconds);
        }
    }
}
