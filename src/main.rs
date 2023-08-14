use chrono::{DateTime, Local};
use clap::Parser;
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    duration: u64,
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let pb = ProgressBar::new(args.duration);
    let local: DateTime<Local> = Local::now();

    if let Some(name) = args.name {
        println!(
            "{}: {} - {}s",
            local.format("%I:%M %p"),
            name,
            args.duration
        );
    }

    for _ in 0..args.duration {
        thread::sleep(Duration::from_secs(1));
        pb.inc(1);
    }
}
