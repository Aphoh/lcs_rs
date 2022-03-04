use std::io::Error;
use clap::{App, Arg};
use lcs_rs::{read_file_and_preprocess, compute};

const K_DEFAULT: u32 = 2;

fn main() -> Result<(), Error> {
    let matches = App::new("lcs_rs")
        .version("0.1.0")
        .author("William Arnold <willarnold@berkeley.edu>")
        .about("Finds the longest common byte subsequence in an arbitrary number of files")
        .arg(
            Arg::with_name("min-files")
                .short("k")
                .long("min-files")
                .help("The minimum number of files the subsequence must be present in")
                .default_value("2"),
        )
        .arg(
            Arg::with_name("files")
                .help("The files to search through")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let k = matches
        .value_of("min-files")
        .map(|k| k.parse::<u32>())
        .unwrap_or(Ok(K_DEFAULT))
        .unwrap();
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    //println!("{:?}", files);

    let mut data: Vec<Vec<u16>> = Vec::with_capacity(files.len());
    for f in &files {
        match read_file_and_preprocess(f) {
            Ok(bstr) => {
                data.push(bstr);
            }
            Err(why) => {
                eprintln!("Error reading file {}: {}", f, why);
            }
        }
    }

    let res = compute(&files, &data, k)?;

    println!("LCS found with length {}", res.length);
    for (f, off) in res.offsets {
        println!("-> in {} at {}", f, off);
    }

    Ok(())
}
