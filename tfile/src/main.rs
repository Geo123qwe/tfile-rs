use std::{
    fs,
    io,
    path::{Path, PathBuf},
};
use ansi_term::Colour;
use atty::Stream;
use clap::Parser;

#[derive(Parser)]
#[clap(version, author)]
struct Args {
    #[clap(default_value = ".")]
    path: String,

    #[clap(short, long, default_value = "5")]
    depth: usize,

    #[clap(long)]
    no_color: bool,
}

fn main() {
    let args = Args::parse();
    let use_color = !args.no_color && atty::is(Stream::Stdout);

    println!("Структура папки: {}\n", Path::new(&args.path).display());
    if let Err(e) = print_tree(&args.path, "", 0, args.depth, use_color) {
        eprintln!("Ошибка: {}", e);
    }
}

fn print_tree(
    path: &str,
    prefix: &str,
    current_depth: usize,
    max_depth: usize,
    use_color: bool,
) -> io::Result<()> {
    if current_depth >= max_depth {
        return Ok(());
    }

    let dir = fs::read_dir(path)?;
    let mut entries: Vec<PathBuf> = dir
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let line = if is_last { "└── " } else { "├── " };
        let new_prefix = if is_last { "    " } else { "│   " };

        let name = entry.file_name().unwrap_or_default().to_string_lossy();

        if entry.is_dir() {
            print_entry(&format!("{}{}{}", prefix, line, name), use_color, Colour::Blue);
        } else {
            print_entry(&format!("{}{}{}", prefix, line, name), use_color, Colour::Yellow);
        }

        if entry.is_dir() {
            print_tree(
                entry.to_str().unwrap(),
                &format!("{}{}", prefix, new_prefix),
                current_depth + 1,
                max_depth,
                use_color,
            )?;
        }
    }

    Ok(())
}

fn print_entry(text: &str, use_color: bool, color: Colour) {
    if use_color {
        println!("{}", color.paint(text));
    } else {
        println!("{}", text);
    }
}