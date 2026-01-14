use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;
use std::io::BufRead;

#[derive(Serialize)]
pub struct LogEntry {
    pub line_number: usize,
    pub content: String,
}

#[derive(Serialize)]
pub struct LogReport {
    pub target_level: String,
    pub total_found: usize,
    pub results: Vec<LogEntry>,
}
/// Programa para filtrar logs de forma eficiente
#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
    #[arg(short, long, default_value = "ERROR")]
    pub level: String,
}

pub fn process_lines<R: BufRead>(reader: R, args: Cli) -> Result<LogReport> {
    let uppercase_level = args.level.to_uppercase();
    let mut results = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let content = line.context("Error al leer una línea del archivo")?;

        if content.contains(&args.level) || content.contains(&uppercase_level) {
            println!("[Línea {}] {}", index + 1, content);
            results.push(LogEntry {
                line_number: index + 1,
                content,
            });
        }
    }
    Ok(LogReport {
        target_level: args.level,
        total_found: results.len(),
        results,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_case_insensitivity() {
        let content = "ERROR: Fallo en la base de datos\nERROR: Conexión perdida\n";
        let reader = Cursor::new(content);

        let args = Cli {
            path: std::path::PathBuf::from("memoria"),
            level: "ERROR".to_string(),
        };
        let result = process_lines(reader, args).unwrap();
        assert_eq!(result.total_found, 2);
    }
}
