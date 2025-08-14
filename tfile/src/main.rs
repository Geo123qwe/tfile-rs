use std::path::Path;
use std::{fs};
use clap::Parser;

/// Утилита для вывода дерева папок
#[derive(Parser)]
#[clap(version = "1.0", author = "Geo123qwe")]
struct Args {
    /// Путь к папке (по умолчанию — текущая)
    path: Option<String>,

    /// Глубина рекурсии
    #[clap(short, long, default_value = "10")]
    depth: usize,
}

fn main() {
    let args = Args::parse();
    let path = args.path.as_deref().unwrap_or(".");

    println!("Структура папки: {}\n", Path::new(path).display());
    if let Err(e) = print_tree(path, "", 0, args.depth) {
        eprintln!("Ошибка: {}", e);
    }
}

fn print_tree(path: &str, prefix: &str, depth: usize, max_depth: usize) -> std::io::Result<()> {
    if depth >= max_depth {
        return Ok(());
    }

    let dir = fs::read_dir(path)?;
    let mut entries: Vec<_> = dir.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let line = if is_last { "└── " } else { "├── " };
        let new_prefix = if is_last { "    " } else { "│   " };

        println!("{}{}{}", prefix, line, entry.file_name().to_string_lossy());

        if entry.file_type()?.is_dir() {
            print_tree(
                entry.path().to_str().unwrap(),
                &format!("{}{}", prefix, new_prefix),
                depth + 1,
                max_depth,
            )?;
        }
    }

    Ok(())
}