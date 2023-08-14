use chrono::{DateTime, Local};
use clap::Parser;
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    duration: String,
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    let seconds = parse_duration_from_str(&args.duration);
    let pb = ProgressBar::new(seconds);
    let local: DateTime<Local> = Local::now();

    if let Some(name) = args.name {
        println!(
            "{}: {} - {}",
            local.format("%I:%M %p"),
            name,
            &args.duration
        );
    }

    for _ in 0..seconds {
        thread::sleep(Duration::from_secs(1));
        pb.inc(1);
    }
}

fn parse_duration_from_str(input: &str) -> u64 {
    let last_char = input.chars().last().unwrap_or_else(|| 's');
    let numeric_part: &str = &input[..input.len() - 1];
    let value: u64 = numeric_part.parse().ok().unwrap();

    match last_char {
        's' => value,
        'm' => value * 60,
        'h' => value * 60 * 60,
        _ => 0,
    }
}
