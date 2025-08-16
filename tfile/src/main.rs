use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use clap::Parser;

/// Аргументы командной строки
#[derive(Parser)]
#[clap(version, author)]
struct Args {
    /// Путь к папке (по умолчанию — текущая)
    path: Option<String>,

    /// Глубина рекурсии
    #[clap(short, long, default_value = "5")]
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

/// Вывод цветного текста
fn colored_print(text: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", text)?;
    stdout.reset()?;
    Ok(())
}

/// Рекурсивный вывод дерева папок
fn print_tree(
    path: &str,
    prefix: &str,
    current_depth: usize,
    max_depth: usize,
) -> io::Result<()> {
    if current_depth >= max_depth {
        return Ok(());
    }

    let dir = fs::read_dir(path)?;
    let mut entries: Vec<PathBuf> = dir
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    // Сортируем для единообразного вывода
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let line = if is_last { "└── " } else { "├── " };
        let new_prefix = if is_last { "    " } else { "│   " };

        let display_name = entry.file_name()
            .unwrap_or_default()
            .to_string_lossy();

        // Цвета в зависимости от типа
        if entry.is_dir() {
            colored_print(
                &format!("{}{}{}", prefix, line, display_name),
                Color::Blue,
            )?;
        } else {
            colored_print(
                &format!("{}{}{}", prefix, line, display_name),
                Color::Yellow,
            )?;
        }

        // Рекурсия для папок
        if entry.is_dir() {
            print_tree(
                entry.to_str().unwrap(),
                &format!("{}{}", prefix, new_prefix),
                current_depth + 1,
                max_depth,
            )?;
        }
    }

    Ok(())
}