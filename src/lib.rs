use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufRead};

/// Programa para filtrar logs de forma eficiente
#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
    #[arg(short, long, default_value = "ERROR")]
    pub level: String,
}

pub fn process_lines<R: BufRead>(reader: R, args: Cli) -> Result<usize> {
    let mut count = 0;
    let uppercase= args.level.to_uppercase();
    for (index, line) in reader.lines().enumerate() {
        let content = line.context("Error al leer una línea del archivo")?;        
        if content.contains(&args.level) || content.contains(&uppercase) {
            count += 1;
            println!("[Línea {}] {}", index + 1, content);
        }
    }
    Ok(count)
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
        assert_eq!(result, 2);
    }
}