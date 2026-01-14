use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufReader};
use std::time::Instant;

use log_guard::{Cli, process_lines};

fn main() -> Result<()> {
    let args = Cli::parse();

    let start = Instant::now();

    let file = File::open(&args.path)
        .with_context(|| format!("No se pudo abrir el archivo: {:?}", args.path))?;
    
    let reader = BufReader::new(file);

    println!("Buscando nivel: {} en {:?}", args.level, args.path);
    println!("---");

    let result = process_lines(reader, args)?;

    println!("La palabra buscada se encontro {} veces", result);
    let duration = start.elapsed();
    println!("Tiempo de ejecución: {:?}", duration);
    Ok(())
}