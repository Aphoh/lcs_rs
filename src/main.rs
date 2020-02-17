use std::*;
use std::string::String;
use std::collections::HashSet;
use bio::data_structures::suffix_array::{suffix_array, RawSuffixArray, LCPArray};
use bio::data_structures::smallints::SmallInts;
use std::ops::Deref;

const K: u32 = 2;

fn main() {
    let data = vec![b"093AB".to_vec(), b"0AB435AB".to_vec(), b"0C093CABB".to_vec()];
    let n_strings = data.len(); 
    let total_length : usize = data.iter().map(|s| s.len()).sum(); 
    let mut combined : Vec<u8> = Vec::with_capacity(total_length + n_strings); 
    let mut sentinel_pos : Vec<usize> = Vec::new();
    for s in data {
        combined.extend(s);
        combined.push(b'$');
        let sent_ind = combined.len() - 1; 
        sentinel_pos.push(sent_ind);
    }


    let suffix_array: RawSuffixArray = suffix_array(&combined); 
    let lcp_array: LCPArray = lcp(&combined, &suffix_array);
   
    let l0 = get_l0(&combined, &suffix_array, &sentinel_pos);

    unsafe {
        print_suffix_array(&suffix_array, &lcp_array, &combined);
    }

    for (i, l) in lcp_array.iter().enumerate() {
        println!("lcp_array[{}]: {}", i, l);
    }
    
    let start_ind = n_strings;
    let end_ind = l0 - 1;
    let mut delta_ls = vec![0usize; end_ind + 1 - start_ind];
    let mut delta_rs = vec![0usize; end_ind + 1 - start_ind];
    let mut type_counters = vec![0u32; n_strings];

    delta_ls[0] = start_ind;
    delta_rs[0] = start_ind + 1;
    if let Some(s_ind) = get_string_index(suffix_array[start_ind] as usize, &sentinel_pos) {
        type_counters[s_ind] += 1;
    }

    //Advance until K-good
    while count_nonzero(&type_counters) < K {
        delta_rs[0] += 1;
        let i = delta_rs[0] - 1; //The new character that is in delta_0
        if let Some(s_ind) = get_string_index(suffix_array[i] as usize, &sentinel_pos) {
            type_counters[s_ind] += 1;
        }
    }

    //delta_n+1 is now K-good, can advance
    for (j, i) in (start_ind..(end_ind + 1)).enumerate().skip(1){
        delta_ls[j] = i;
        delta_rs[j] = delta_rs[j - 1];
        //Remove one from the type counter corresponding to the i-1-th char
        if let Some(ls_ind) = get_string_index(suffix_array[i-1] as usize, &sentinel_pos) {
            type_counters[ls_ind] -= 1;
        }
        while count_nonzero(&type_counters) < K {
            let new_char_ind = delta_rs[j];
            delta_rs[j] += 1;
            if let Some(s_ind) = get_string_index(suffix_array[new_char_ind] as usize, &sentinel_pos) {
                type_counters[s_ind] += 1;
            }
        }
    }
   
    let mut maxi = 0usize;
    let mut maxv = 0isize;
    let result = delta_ls.iter().zip(&delta_rs).enumerate()
        .map(|(i, (&l, &r))| {
            let mut min = isize::MAX;
            for k in (l+1)..r {
                if let Some(v) = lcp_array.get(k) {
                    min = if v < min { v } else { min }
                }
            }
            (i, min)

        });

    println!("(delta, min_lcp) pairs:");
    for (i, lcp_min) in result {
        println!("({}:{}, {})", &delta_ls[i], &delta_rs[i], &lcp_min);
        if lcp_min > maxv {
            maxv = lcp_min;
            maxi = i;
        }
    }

    println!("Got max delta {} with min_lcp {}", maxi, maxv);

    let lcs_start = suffix_array[delta_ls[maxi]] as usize;
    let lcs_end = lcs_start + (maxv as usize);

    let lcs = &combined[lcs_start..lcs_end];
    unsafe {
        println!("Result: {:?}", str::from_utf8_unchecked(lcs));
    }
}

fn count_nonzero(vec: &Vec<u32>) -> u32 {
    let mut c = 0;
    for i in vec{
        if i > &0 {
            c+=1
        }
    }
    c
}

fn get_l0(text: &Vec<u8>,
          suffix_array: &RawSuffixArray, 
          sentinel_pos: &Vec<usize>) -> usize {
    
    let mut present_strs = HashSet::<usize>::new(); 
    let total_len = text.len(); 

    //Find L_0
    let mut n_strings: Vec<usize> = vec![(K as usize) + 1; total_len];
    for i in (0..total_len).rev(){
        if let Some(si) = get_string_index(suffix_array[i] as usize, &sentinel_pos) {
            present_strs.insert(si);
            n_strings[i] = present_strs.len()
        }
    }
    let mut l0 = total_len;
    for (i, k) in n_strings.iter().enumerate() {
        if k < &(K as usize) {
            l0 = i - 1; //As [i, L] has at least K distinct essenital types
            break;
        }
    }
    l0 + 1
}

fn get_string_index(pos: usize, sentinel_pos: &Vec<usize>) -> Option<usize> {
    let mut i = 0;
    while pos > sentinel_pos[i] {
        i+=1;
    }

    if sentinel_pos[i] == pos { None } else { Some(i) }
}

fn print_suffix_array(pos: &RawSuffixArray, lcp_array: &LCPArray, text: &Vec<u8>) {
    unsafe {
        let text_str = str::from_utf8_unchecked(text);
        println!("Suffix Array for {:?}", text_str);
        for i in 0..text_str.len() {
            println!("sa[{:0>2}] : {:0>2} | lcp {:0>2} | {:?}", 
                     &i, 
                     &pos[i], 
                     lcp_array.get(i).unwrap_or(-999), 
                     &text_str[pos[i]..text_str.len()]);
        }
    }
} 

pub fn lcp<SA: Deref<Target = RawSuffixArray>>(text: &[u8], pos: SA) -> LCPArray {
    assert_eq!(text.len(), pos.len());
    let n = text.len();
    let sentinel = text[n - 1];

    // provide the lexicographical rank for each suffix
    let mut rank: Vec<usize> = iter::repeat(0).take(n).collect();
    for (r, p) in pos.iter().enumerate() {
        rank[*p] = r;
    }

    let mut lcp = SmallInts::from_elem(-1, n + 1);
    let mut l = 0usize;
    for (p, &r) in rank.iter().enumerate().take(n - 1) {
        // since the sentinel has rank 0 and is excluded above,
        // we will never have a negative index below
        let pred = pos[r - 1];
        while pred + l < n && p + l < n && text[p + l] == text[pred + l] && text[p + 1] != sentinel && text[pred + l] != sentinel {
            l += 1;
        }
        lcp.set(r, l as isize);
        l = if l > 0 { l - 1 } else { 0 };
    }

    lcp
}

/*fn as_hex(bytes: Vec<u8>) -> String {
    let mut res = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        res.push_str(format!("{:0>2X}", b).as_str());
    }
    res
}*/

