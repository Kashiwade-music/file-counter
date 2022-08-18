use ansi_term::{Colour, Style};
use fs_extra::dir::get_size;
use humansize::{file_size_opts as options, FileSize};
use std::{env, path::Path};
use walkdir::WalkDir;

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!(
        "{:>21} : {}",
        "current directory",
        Colour::Cyan.paint(current_dir.to_str().unwrap())
    );
    println!(
        "{:>21} : {}",
        "number of total files",
        Colour::Green.paint(format!(
            "{}",
            WalkDir::new(&current_dir).into_iter().count() - 1
        ))
    );
    println!(
        "{:>21} : {}",
        "size of total files",
        Colour::Green.paint(format!(
            "{}",
            get_size(&current_dir)
                .unwrap()
                .file_size(options::CONVENTIONAL)
                .unwrap()
        ))
    );

    println!(
        "\n{}   {}   {}",
        Style::new().bold().paint(format!("{:^5}", "num")),
        Style::new().bold().paint(format!("{:^10}", "size")),
        Style::new().bold().paint(format!("{}", "path"))
    );
    println!("------------------------------------------------------------------");

    for path_1 in WalkDir::new(current_dir)
        .max_depth(1)
        .min_depth(1)
        .sort_by_key(|a| a.path().is_file())
    {
        let path: &Path = path_1.as_ref().unwrap().path();
        if path.is_dir() {
            println!(
                "{:>5}   {:>10}   {}",
                WalkDir::new(path).into_iter().count() - 1,
                get_size(path)
                    .unwrap()
                    .file_size(options::CONVENTIONAL)
                    .unwrap(),
                Colour::Cyan.paint(path.to_str().unwrap())
            );
        } else {
            println!(
                "{:>5}   {:>10}   {}",
                "---",
                get_size(path)
                    .unwrap()
                    .file_size(options::CONVENTIONAL)
                    .unwrap(),
                path.to_str().unwrap()
            );
        }
    }
}
