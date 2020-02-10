use suffix::SuffixTable;
use std::*;
use std::string::String;
use std::collections::HashSet;

const K: u32 = 2;

fn main() {
    let data = vec!["ABC", "BCD", "ABB"];
    let n_strings = data.len(); 
    let lengths : Vec<usize> = data.iter().map(|s| s.len()).collect();
    
    let mut combined = String::with_capacity(lengths.iter().sum::<usize>() + data.len());
    let mut non_essential_pos : HashSet<usize> = HashSet::new();
    for s in data {
        combined.push_str(&s);
        combined.push('$');
        non_essential_pos.insert(combined.len() - 1);
    }

    print!("{}\n", combined);


    let suffix_table: SuffixTable = SuffixTable::new(&combined);
    let suffix_array = suffix_table.table();
    let lcp_array: Vec<u32> = suffix_table.lcp_lens();
   
    let l0 = get_l0(&suffix_table, &non_essential_pos, &lengths);

    print!("{:?}\n", suffix_table);
    print!("L0: {}\n", &l0);
    print!("LCP Array:\n{:0>2?}\n", lcp_array);
    
    let start_ind = n_strings;
    let end_ind = l0 - 1;
    let mut delta_ls = vec![0usize; end_ind + 1 - start_ind];
    let mut delta_rs = vec![0usize; end_ind + 1 - start_ind];
    let mut type_counters = vec![0u32; n_strings];

    delta_ls[0] = start_ind;
    delta_rs[0] = start_ind + 1;
    if !non_essential_pos.contains(&start_ind) {
        let s_ind = get_string_index(suffix_array[start_ind] as usize, &lengths);
        type_counters[s_ind] += 1;
    }
    //Advance until K-good
    while count_nonzero(&type_counters) < K {
        delta_rs[0] += 1;
        let i = delta_rs[0] - 1; //The new character that is in delta_0
        let s_ind = get_string_index(suffix_array[i] as usize, &lengths);
        type_counters[s_ind] += 1;
    }

    //delta_n+1 is now k-good, can advance

    for (j, i) in (start_ind..(end_ind + 1)).enumerate().skip(1){
        delta_ls[j] = i;
        delta_rs[j] = delta_rs[j - 1];
        //Remove one from the type counter corresponding to the i-1-th char
        if !non_essential_pos.contains(&(i-1)) { 
            let ls_ind = get_string_index(suffix_array[i-1] as usize, &lengths);
            type_counters[ls_ind] -= 1;
        }
        while count_nonzero(&type_counters) < K {
            let new_char_ind = delta_rs[j];
            delta_rs[j] += 1;
            if !non_essential_pos.contains(&new_char_ind) { 
                let s_ind = get_string_index(suffix_array[new_char_ind] as usize, &lengths);
                type_counters[s_ind] += 1;
            }
        }
    }
    print!("{:0>2?}\n", delta_ls);
    print!("{:0>2?}\n", delta_rs);
   
    let mut maxi = 0usize;
    let mut maxv = 0u32;
    for i in 0..delta_ls.len(){
        let mut min_lcp = u32::MAX;
        for j in delta_ls[i]..delta_rs[i] {
            if min_lcp > lcp_array[j - 1]{
                min_lcp = lcp_array[j - 1];
            }
        }
        if min_lcp >= maxv {
            maxv = min_lcp;
            maxi = i;
        }
    }

    print!("Got max delta {} with min_lcp {}\n", maxi, maxv);

    let lcs_start = suffix_array[delta_ls[maxi]] as usize;
    let lcs_end = lcs_start + (maxv as usize) + 1;

    let lcs = &combined[lcs_start..lcs_end];
    print!("Result: {:?}\n", lcs);
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
          invalid_pos: &HashSet<usize>, 
          string_lengths: &Vec<usize>) -> usize {
    
    let mut present_strs = HashSet::<u32>::new(); 
    let total_len = suffix_table.text().len();
    let suffix_array = suffix_table.table();

    //Find L_0
    let mut n_strings = vec![K + 1; total_len];
    for i in (0..total_len).rev(){
        n_strings[i] = if !invalid_pos.contains(&i) {
            let si = get_string_index(suffix_array[i] as usize, &string_lengths);
            present_strs.insert(si as u32);
            present_strs.len() as u32
        } else {
            K + 1 
        }
    }
    print!("{:?}\n", n_strings);
    let mut l0 = total_len;
    for (i, k) in n_strings.iter().enumerate() {
        if k < &K {
            l0 = i - 1; //As [i, L] has at least K distinct essenital types
            break;
        }
    }
    l0 + 1
}

fn get_string_index(pos: usize, string_lengths: &Vec<usize>) -> usize {
   let mut curr = 0; 
   let mut acc_len = 0;
   while pos > acc_len + string_lengths[curr] {
       acc_len += string_lengths[curr] + 1;
       curr+=1;
   }
   curr
}

/*fn as_hex(bytes: Vec<u8>) -> String {
    let mut res = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        res.push_str(format!("{:0>2X}", b).as_str());
    }
    res
}*/

