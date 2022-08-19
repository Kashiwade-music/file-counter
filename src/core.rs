use indicatif::{MultiProgress, ProgressIterator, ProgressStyle};
use std::{fs, path::Path};
use walkdir::WalkDir;

pub struct Data {
    pub name: String,
    pub size: u64,
    pub num: u64,
}

fn get_size_and_count(path: &Path, bars: &MultiProgress) -> Data {
    //! 与えられたパスがディレクトリの場合、内部を再帰的にwalkして、
    //! 内部にある総ファイル数・サイズを返します。

    let mut size: u64 = 0;
    let mut num: u64 = 0;

    let main_pb = WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .progress_count(
            WalkDir::new(path)
                .min_depth(1)
                .into_iter()
                .count()
                .try_into()
                .unwrap(),
        )
        .with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );

    bars.add(main_pb.progress.clone());

    main_pb
        .progress
        .set_message(path.to_str().unwrap().to_owned());

    if path.is_dir() {
        for entry in main_pb {
            match entry {
                Ok(dir_entry) => {
                    if dir_entry.path().is_file() {
                        match dir_entry.path().metadata() {
                            Ok(meta) => {
                                size += meta.len();
                                num += 1;
                            }
                            Err(_e) => bars
                                .println(format!(
                                    "Can't access to {}",
                                    dir_entry.path().to_str().unwrap()
                                ))
                                .unwrap(),
                        }
                    }
                }
                Err(e) => bars.println(format!("error {}", e)).unwrap(),
            }
        }
    } else {
        size = path.metadata().unwrap().len();
        num = 1;
    }

    let returns = Data {
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        size: size,
        num: num,
    };

    return returns;
}

pub struct OutputData {
    pub root_abs_dirpath: String,
    pub root_total_file_num: u64,
    pub root_total_file_size: u64,
    pub dirs: Vec<Data>,
    pub files: Vec<Data>,
}

pub fn get_rootdir_info(
    path: &Path,
    should_sort_by_filenum: bool,
    should_sort_by_filesize: bool,
) -> OutputData {
    //! 与えられたパスルートディレクトリとして、配下にあるファイルやディレクトリの情報を返します。

    let mut dir_returns = Vec::new();
    let mut file_returns = Vec::new();
    let mut total_file_size = 0;
    let mut total_file_num = 0;

    let bars = MultiProgress::new();

    let main_pb = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .progress_count(
            WalkDir::new(path)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .count()
                .try_into()
                .unwrap(),
        )
        .with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );

    bars.add(main_pb.progress.clone());

    for entry in main_pb {
        match entry {
            Ok(dir_entry) => {
                let res = get_size_and_count(dir_entry.path(), &bars);
                total_file_size += res.size;
                total_file_num += res.num;
                if dir_entry.path().is_file() {
                    file_returns.push(res);
                } else {
                    dir_returns.push(res);
                }
            }
            Err(e) => bars.println(format!("error {}", e)).unwrap(),
        }
    }

    if should_sort_by_filenum {
        dir_returns.sort_by(|a, b| b.num.cmp(&a.num));
        file_returns.sort_by(|a, b| b.num.cmp(&a.num));
    }
    if should_sort_by_filesize {
        dir_returns.sort_by(|a, b| b.size.cmp(&a.size));
        file_returns.sort_by(|a, b| b.size.cmp(&a.size));
    }

    let returns = OutputData {
        root_abs_dirpath: fs::canonicalize(path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        root_total_file_num: total_file_num,
        root_total_file_size: total_file_size,
        dirs: dir_returns,
        files: file_returns,
    };

    return returns;
}
