
extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::path::Path;

// ANSI colors
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

// File categories (customize freely)
const DATA_EXT: &[&str] = &["json", "csv", "dat", "xml", "yaml", "yml", "toml"];
const CODE_EXT: &[&str] = &["rs", "py", "r", "js", "ts", "cpp", "c", "h", "hpp", "java", "go"];
const IMG_EXT: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "svg"];

fn main() {
    let matches = App::new("rtree")
        .version("1.0")
        .author("Jayden Jimenez")
        .about("Pretty file tree viewer with colored file types")
        .arg(
            Arg::with_name("depth")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("How deep to search"),
        )
        .arg(
            Arg::with_name("files-only")
                .long("files-only")
                .help("Only show files"),
        )
        .arg(
            Arg::with_name("dirs-only")
                .long("dirs-only")
                .help("Only show directories"),
        )
        .arg(
            Arg::with_name("show-hidden")
                .long("show-hidden")
                .help("Show hidden files"),
        )
        .get_matches();

    let max_depth: usize = matches.value_of("depth").unwrap().parse().unwrap();
    let files_only = matches.is_present("files-only");
    let dirs_only = matches.is_present("dirs-only");
    let show_hidden = matches.is_present("show-hidden");

    list_dir(Path::new("."), 0, max_depth, "", files_only, dirs_only, show_hidden);
}

fn list_dir(
    path: &Path,
    depth: usize,
    max_depth: usize,
    prefix: &str,
    files_only: bool,
    dirs_only: bool,
    show_hidden: bool,
) {
    if depth > max_depth {
        return;
    }

    let entries: Vec<_> = match fs::read_dir(path) {
        Ok(e) => e
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if !show_hidden && name.starts_with('.') {
                    return false;
                }
                true
            })
            .collect(),
        Err(_) => return,
    };

    for (i, entry) in entries.iter().enumerate() {
        let p = entry.path();
        let name = p.file_name().unwrap().to_string_lossy();
        let is_dir = p.is_dir();
        let is_last = i == entries.len() - 1;

        // Filtering flags
        if files_only && is_dir {
            continue;
        }
        if dirs_only && !is_dir {
            continue;
        }

        let branch = if is_last { "└── " } else { "├── " };

        // Directory = blue
        if is_dir {
            println!("{}{}{}{}{}", prefix, branch, BLUE, name, RESET);
        } else {
            // File = choose color by extension
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let color = if DATA_EXT.contains(&ext.as_str()) {
                YELLOW
            } else if CODE_EXT.contains(&ext.as_str()) {
                GREEN
            } else if IMG_EXT.contains(&ext.as_str()) {
                MAGENTA
            } else {
                RESET
            };

            println!("{}{}{}{}{}", prefix, branch, color, name, RESET);
        }

        // Recurse into directory
        if is_dir {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            list_dir(
                &p,
                depth + 1,
                max_depth,
                &new_prefix,
                files_only,
                dirs_only,
                show_hidden,
            );
        }
    }
}

