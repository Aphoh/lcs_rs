use bio::data_structures::suffix_array::{LCPArray, RawSuffixArray};
use clap::{Arg, App};
use std::io::{Error, ErrorKind};
use std::*;
use std::{fs, io};
mod lcs;

const K_DEFAULT: u32 = 2;
const DEBUG: bool = false;

fn main() -> Result<(), Error> {
    let matches = App::new("lcs_rs")
        .version("0.1.0")
        .author("William Arnold <willarnold@berkeley.edu>")
        .about("Finds the longest common byte subsequence in an arbitrary number of files")
        .arg(Arg::with_name("min-files")
             .short("k")
             .long("min-files")
             .help("The minimum number of files the subsequence must be present in")
             .default_value("2"))
        .arg(Arg::with_name("files")
            .help("The files to search through")
            .required(true).min_values(1))
        .get_matches();

    let k = matches.value_of("min-files").map(|k| k.parse::<u32>()).unwrap_or(Ok(K_DEFAULT)).unwrap();
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    println!("{:?}", files);

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

    if data.len() < k as usize {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("You must pass {} files as arguments", k),
        ));
    }

    let n_strings = data.len();
    let total_length = data.iter().map(|s| s.len()).sum();
    let mut combined: Vec<u16> = Vec::with_capacity(total_length);
    let mut sentinel_pos: Vec<usize> = Vec::new();
    let mut file_starts: Vec<usize> = Vec::new();

    for s in data {
        file_starts.push(combined.len());
        combined.extend(s);
        let sent_ind = combined.len() - 1;
        sentinel_pos.push(sent_ind);
    }

    let suffix_array: RawSuffixArray = lcs::suffix_array_u16(&combined, &(n_strings as u16));
    let lcp_array: LCPArray = lcs::lcp_unique_sentinels(&combined, &suffix_array);

    let l0 = lcs::get_l0(&combined, &suffix_array, &k, &sentinel_pos);

    let (delta_ls, delta_rs) =
        lcs::compute_deltas(&n_strings, &l0, &k, &suffix_array, &sentinel_pos);

    let (maxi, maxv) = lcs::max_min_lcp(&delta_ls, &delta_rs, &lcp_array);

    let lcs_start = suffix_array[delta_ls[maxi]] as usize;
    let lcs_end = lcs_start + (maxv as usize);

    if DEBUG {
        let lcs = &combined[lcs_start..lcs_end];
        print!("Result: ");
        for b in post_process(lcs) {
            print!("{:0>2x} ", b);
        }
        print!("\n");
    }
    let file_offsets = file_offsets_in_delta(&files, 
                                             &file_starts, 
                                             &sentinel_pos,
                                             &suffix_array, 
                                             &delta_ls[maxi], 
                                             &delta_rs[maxi]);

    println!("LCS found with length {}", maxv);
    for (f, off) in file_offsets {
        println!("-> in {} at {}", f, off);
    }
    
    Ok(())
}

fn file_offsets_in_delta<'a>(files: &[&'a str], 
                             file_starts: &Vec<usize>, 
                             sentinel_pos: &Vec<usize>,
                             suffix_array: &RawSuffixArray, 
                             delta_l: &usize, 
                             delta_r: &usize) -> Vec<(&'a str, usize)> {

    let mut file_counts = vec![0usize; files.len()];
    let mut offsets     = vec![0usize; files.len()];

    for i in *delta_l..*delta_r {
        let suff_ind = suffix_array[i];
        if let Some(si) = lcs::get_string_index(suff_ind, sentinel_pos) {
            file_counts[si] += 1;
            offsets[si] = suff_ind - file_starts[si];
        }
    }

    let filenames: Vec<(&str, usize)> = file_counts.iter().enumerate()
        .filter(|(_, c)| *c > &0)
        .map(|(i, _)| (files[i], offsets[i]))
        .collect();
   
    filenames
}

/* Reads the given file and returns a vector of all bytes incremented by one
 * with the 0 sentinel at the end.
 */
fn read_file_and_preprocess(filename: &str) -> Result<Vec<u16>, io::Error> {
    let f = fs::read(filename)?;
    let mut shifted_bytes = vec![0u16; f.len() + 1]; //Increment all bytes by one for sentinels
    for i in 0..f.len() {
        if DEBUG {
            print!("{:0>2x} ", f[i]);
        }
        shifted_bytes[i] = (f[i] as u16) + 1;
    }
    if DEBUG {
        print!("\n");
    }
    Ok(shifted_bytes)
}

fn post_process(text: &[u16]) -> Vec<u8> {
    text.iter().map(|s| (s - 1) as u8).collect()
}
