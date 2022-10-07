#![deny(missing_docs)]
#![deny(deprecated)]
#![warn(missing_doc_code_examples)]

//! `chesspace` calculates timestamps to pace yourself in chess games.

use clap::Parser;
use std::time::Duration;

/// calculates timestamps to pace yourself in a chess game
#[derive(Parser, Debug)]
struct Opt {
    /// Timecontrol start time in minutes
    minutes: u64,

    /// Increment in seconds
    increment: u64,

    /// Moves to be played
    #[arg(short, long, default_value_t = 40)]
    moves: u32,

    /// lichess (doesn't apply increment at first move)
    #[arg(short, long)]
    lichess: bool,

    /// Display every <display> move
    #[arg(short, long, default_value_t = 1)]
    display: u32,

    /// Use <percentage> of total time for the opening (if skipped opening moves are played twice as fast as normal ones)
    #[arg(short, long)]
    percentage: Option<u32>,

    /// Number of moves considered to be opening moves
    #[arg(short, long)]
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
    let opt = Opt::parse();

    let start_time = Duration::new(opt.minutes * 60, 0);
    let increment = Duration::new(opt.increment, 0);
    let total_time = start_time
        + increment
            * if opt.lichess {
                opt.moves - 1
            } else {
                opt.moves
            };

    let opening_moves: u32;
    let opening_time_per_move: Duration;
    let remaining_time_per_move: Duration;

    print!("timecontrol: {}+{}", opt.minutes, opt.increment);
    if opt.lichess {
        print!(" (lichess)");
    }
    println!();

    match (opt.opening, opt.percentage) {
        (Some(opening), Some(percentage)) => {
            opening_moves = opening;
            let first_duration = total_time * percentage / 100;
            opening_time_per_move = first_duration / opening;
            remaining_time_per_move = (total_time - first_duration) / (opt.moves - opening);
        }
        (Some(opening), None) => {
            opening_moves = opening;
            // opening moves are played twice as fast
            // total_time = opening_time_per_move * opening_moves + (opt.moves - opening_moves) * 2 * opening_time_per_move
            opening_time_per_move = total_time / (2 * opt.moves - opening_moves);
            remaining_time_per_move = 2 * opening_time_per_move;
        }
        _ => {
            opening_moves = 0;
            opening_time_per_move = Duration::ZERO;
            remaining_time_per_move = total_time / opt.moves;
        }
    }

    let (total_minutes, total_seconds) = break_duration_to_min(total_time);
    if opening_moves > 0 {
        println!("total time: {}:{:0>2}min", total_minutes, total_seconds);
        println!(
            "time per opening move: {:.1}s ({} moves)",
            opening_time_per_move.as_secs_f32(),
            opening_moves
        );
        println!(
            "time per remaining move: {:.1}s",
            remaining_time_per_move.as_secs_f32()
        );
    } else {
        println!("total time: {}:{:0>2}min", total_minutes, total_seconds);
        println!(
            "time per move: {:.1}s",
            remaining_time_per_move.as_secs_f32()
        );
    }

    let mut time_remaining = start_time;
    for i in 1..=opt.moves {
        if !opt.lichess || i != 1 {
            time_remaining += increment;
        }

        time_remaining -= if i <= opening_moves {
            opening_time_per_move
        } else {
            remaining_time_per_move
        };

        if i % opt.display == 0 {
            let (minutes, seconds) = break_duration_to_min(time_remaining);
            println!("{:>2}: {:>2}:{:0>2.0}", i, minutes, seconds);
        }
    }
}
