use suffix::SuffixTable;
use std::*;
use std::string::String;
use std::collections::HashSet;

const K: u32 = 2;

fn main() {
    let data = vec!["093AB", "0AB435AB", "0K093KABB"];
    let n_strings = data.len(); 
    let total_length : usize = data.iter().map(|s| s.len()).sum(); 
    let mut combined = String::with_capacity(total_length);
    let mut sentinel_pos_set : HashSet<usize> = HashSet::new();
    let mut sentinel_pos : Vec<usize> = Vec::new();
    for s in data {
        combined.push_str(&s);
        combined.push('$');
        let sent_ind = combined.len() - 1; 
        sentinel_pos.push(sent_ind);
        sentinel_pos_set.insert(sent_ind);
    }

    print!("{}\n", combined);


    let suffix_table: SuffixTable = SuffixTable::new(&combined);
    let suffix_array = suffix_table.table();
    let lcp_array: Vec<u32> = suffix_table.lcp_lens();
   
    let l0 = get_l0(&suffix_table, &sentinel_pos_set, &sentinel_pos);

    print!("{:?}\n", suffix_table);
    print!("L0: {}\n", &l0);
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
    print!("{:0>2?}\n", delta_ls);
    print!("{:0>2?}\n", delta_rs);
   
    let mut maxi = 0usize;
    let mut maxv = 0u32;
    let result = delta_ls.iter().zip(&delta_rs).enumerate()
        .map(|(i, (&l, &r))| (i, lcp_array[(l + 1)..r].iter().min().unwrap_or(&0)));

    println!("(delta, min_lcp) pairs:");
    for (i, lcp_min) in result {
        println!("({}:{}, {})", &delta_ls[i], &delta_rs[i], &lcp_min);
        if lcp_min > &maxv {
            maxv = *lcp_min;
            maxi = i;
        }
    }

    println!("Got max delta {} with min_lcp {}", maxi, maxv);

    let lcs_start = suffix_array[delta_ls[maxi]] as usize;
    let lcs_end = lcs_start + (maxv as usize);

    let lcs = &combined[lcs_start..lcs_end];
    println!("Result: {:?}", lcs);
        
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

fn get_l0(suffix_table: &SuffixTable, 
          sentinel_pos_set: &HashSet<usize>, 
          sentinel_pos: &Vec<usize>) -> usize {
    
    let mut present_strs = HashSet::<usize>::new(); 
    let total_len = suffix_table.text().len();
    let suffix_array = suffix_table.table();

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

/*fn as_hex(bytes: Vec<u8>) -> String {
    let mut res = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        res.push_str(format!("{:0>2X}", b).as_str());
    }
    res
}*/

