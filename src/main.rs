use std::error::Error;

use clap::{self, ArgGroup, Parser};
use grid::{
    solver::{solve, Algo, Heuristic},
    Grid,
};

mod complex;
mod grid;

#[derive(clap::Parser)]
#[command(group(ArgGroup::new("input").required(true).args(&["generate", "filepath"])))]
struct Args {
    #[arg(long, value_parser = ["manhattan", "euclidian", "misplaced"])]
    heuristic: String,

    #[arg(long, requires = "generate_complexity")]
    generate: Option<usize>,

    #[arg(long, requires = "generate")]
    generate_complexity: Option<usize>,

    #[arg(long, conflicts_with_all = &["generate", "generate_complexity"])]
    filepath: Option<String>,

    #[arg(long, value_parser = ["astar", "idastar"], default_value = "astar")]
    algo: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let g = match (args.filepath, args.generate, args.generate_complexity) {
        (Some(filepath), None, None) => match std::fs::read_to_string(filepath) {
            Ok(raw) => match grid::parser::parse(raw.as_str()) {
                Ok(g) => g,
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e.into()),
        },
        (None, Some(size), Some(n)) => Grid::create_random_grid(size as i32, n as i32),
        _ => unreachable!(),
    };

    let h = match args.heuristic.as_str() {
        "manhattan" => Heuristic::Manhattan,
        "euclidian" => Heuristic::Euclidian,
        "misplaced" => Heuristic::Misplaced,
        _ => unreachable!(),
    };

    let algo = match args.algo.as_str() {
        "astar" => Algo::Astar,
        "idastar" => Algo::IDAstar,
        _ => unreachable!(),
    };

    match solve(&g, h, algo) {
        Ok(res) => println!("{}", res),
        Err(e) => return Err(e.into()),
    };

    Ok(())
}
