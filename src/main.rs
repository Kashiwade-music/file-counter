use ansi_term::{Colour, Style};
use clap::Parser;
use humansize::{file_size_opts as options, FileSize};
use std::path::PathBuf;

mod core;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// directory path which you want to see
    #[clap(default_value_t = String::from("./"), value_parser)]
    path: String,

    /// sort by file num
    #[clap(short, long, action)]
    num: bool,

    /// sort by file size
    #[clap(short, long, action)]
    size: bool,
}

fn main() {
    let cli = Cli::parse();

    let result = core::get_rootdir_info(&PathBuf::from(cli.path), cli.num, cli.size);

    println!(
        "\n{:>21} : {}",
        "current directory",
        Colour::Cyan.paint(result.root_abs_dirpath)
    );
    println!(
        "{:>21} : {}",
        "number of total files",
        Colour::Green.paint(format!("{}", result.root_total_file_num))
    );
    println!(
        "{:>21} : {}",
        "size of total files",
        Colour::Green.paint(format!(
            "{}",
            result
                .root_total_file_size
                .file_size(options::CONVENTIONAL)
                .unwrap()
        ))
    );

    println!(
        "\n{}   {}   {}",
        Style::new().bold().paint(format!("{:^7}", "num")),
        Style::new().bold().paint(format!("{:^12}", "size")),
        Style::new().bold().paint(format!("{}", "path"))
    );
    println!("-----------------------------------------------");

    for data in result.dirs.iter() {
        println!(
            "{:>7}   {:>12}   {}",
            data.num,
            data.size.file_size(options::CONVENTIONAL).unwrap(),
            Colour::Cyan.paint(&data.name)
        );
    }
    for data in result.files.iter() {
        println!(
            "{:>7}   {:>12}   {}",
            "---",
            data.size.file_size(options::CONVENTIONAL).unwrap(),
            data.name
        );
    }
}
