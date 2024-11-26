use clap::Parser;

mod day;
mod util;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long)]
    day: Option<i32>,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    let day = args.day.unwrap_or(util::get_latest_day()?);
    let lines = util::get_lines(day)?;
    let solution = util::solve(day, lines)?;
    println!("{}", solution);
    Ok(())
}
