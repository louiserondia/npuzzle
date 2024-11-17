use super::Grid;
use crate::complex::Complex;
use std::{collections::HashSet, error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ParsingError;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parsing error")
    }
}

impl Error for ParsingError {}

#[cfg(test)]
mod tests;

pub fn parse(raw: &str) -> Result<Grid, ParsingError> {
    let mut it = raw
        .lines()
        .map(str::trim)
        .filter(|s| s.len() > 0)
        .filter(|s| !s.starts_with("#"))
        .map(|s| s.split("#").next().unwrap().split_whitespace());

    if it.clone().count() < 2 {
        return Err(ParsingError);
    }

    let size_it = it.next().unwrap();
    if size_it.clone().count() != 1 {
        return Err(ParsingError);
    }
    let size = match size_it.clone().next().unwrap().parse::<i32>() {
        Ok(n) if n >= 1 => n,
        _ => return Err(ParsingError),
    };

    if it.clone().any(|l| l.count() != size as usize) {
        return Err(ParsingError);
    }

    let it = it.flat_map(|l| l.map(|w| w.parse::<i32>()));

    if it.clone().any(|e| e.is_err()) {
        return Err(ParsingError);
    }

    let v = it.map(|e| e.unwrap()).collect::<Vec<_>>();

    if v.len() != size.pow(2) as usize {
        return Err(ParsingError);
    }

    let hs = v.iter().collect::<HashSet<_>>();
    if hs.len() != v.len() {
        return Err(ParsingError);
    }
    for n in 0..size.pow(2) {
        if !hs.contains(&n) {
            return Err(ParsingError);
        }
    }

    let (zero_i, _) = v.iter().enumerate().find(|(_, n)| **n == 0).unwrap();
    let zero = Complex::new(zero_i as i32 % size, zero_i as i32 / size);

    Ok(Grid { size, zero, v })
}
