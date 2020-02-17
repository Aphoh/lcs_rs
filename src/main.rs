use bio::data_structures::suffix_array::{LCPArray, RawSuffixArray};
use std::io::{Error, ErrorKind};
use std::string::String;
use std::*;
use std::{env, fs, io};
mod lcs;

const K: u32 = 2;
const DEBUG: bool = false;

fn main() -> Result<(), Error> {
    let files: Vec<String> = env::args().skip(1).collect();
    let mut data: Vec<Vec<u16>> = Vec::with_capacity(files.len());
    for f in files {
        match read_file_and_preprocess(&f) {
            Ok(bstr) => {
                data.push(bstr);
            }
            Err(why) => {
                eprintln!("Error reading file {}: {}", f, why);
            }
        }
    }

    if data.len() < K as usize {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("You must pass {} files as arguments", K),
        ));
    }

    let n_strings = data.len();
    let total_length = data.iter().map(|s| s.len()).sum();
    let mut combined: Vec<u16> = Vec::with_capacity(total_length);
    let mut sentinel_pos: Vec<usize> = Vec::new();

    for s in data {
        combined.extend(s);
        let sent_ind = combined.len() - 1;
        sentinel_pos.push(sent_ind);
    }

    let suffix_array: RawSuffixArray = lcs::suffix_array_u16(&combined, &(n_strings as u16));
    let lcp_array: LCPArray = lcs::lcp_unique_sentinels(&combined, &suffix_array);

    let l0 = lcs::get_l0(&combined, &suffix_array, &K, &sentinel_pos);

    let (delta_ls, delta_rs) =
        lcs::compute_deltas(&n_strings, &l0, &K, &suffix_array, &sentinel_pos);

    let (maxi, maxv) = lcs::max_min_lcp(&delta_ls, &delta_rs, &lcp_array);

    let lcs_start = suffix_array[delta_ls[maxi]] as usize;
    let lcs_end = lcs_start + (maxv as usize);

    let lcs = &combined[lcs_start..lcs_end];
    if DEBUG {
        print!("Result: ");
        for b in post_process(lcs) {
            print!("{:0>2x} ", b);
        }
        print!("\n");
    }
    Ok(())
}

/* Reads the given file and returns a vector of all bytes incremented by one
 * with the 0 sentinel at the end.
 */
fn read_file_and_preprocess(filename: &String) -> Result<Vec<u16>, io::Error> {
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
