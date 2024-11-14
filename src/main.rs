use std::path::PathBuf;

use clap::{self, ArgGroup, Parser};
use grid::{
    solver::{print_res, solve, Heuristic},
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
    filepath: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.heuristic);
    let h = match args.heuristic.as_str() {
        "manhattan" => Heuristic::Manhattan,
        "euclidian" => Heuristic::Euclidian,
        "misplaced" => Heuristic::Misplaced,
        _ => unreachable!(),
    };
    let size = 3;
    for _ in 0..1 {
        let g = Grid::create_random_grid(size, 10000);
        let res = solve(g.clone(), h);
        print_res(res, &g);
    }
}
