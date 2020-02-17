use bio::data_structures::smallints::SmallInts;
use bio::data_structures::suffix_array::{LCPArray, RawSuffixArray};
use std::collections::HashSet;
use std::ops::Deref;
use std::*;

mod sais;

/* Get the L_0 corresponding to the upper bound upon which to search for K-good strings
 *
 */
pub fn get_l0(
    text: &Vec<u16>,
    suffix_array: &RawSuffixArray,
    k: &u32,
    sentinel_pos: &Vec<usize>,
) -> usize {
    let mut present_strs = HashSet::<usize>::new();
    let total_len = text.len();

    //Find L_0
    let mut n_strings: Vec<usize> = vec![(*k as usize) + 1; total_len];
    for i in (0..total_len).rev() {
        if let Some(si) = get_string_index(suffix_array[i] as usize, &sentinel_pos) {
            present_strs.insert(si);
            n_strings[i] = present_strs.len()
        }
    }
    let mut l0 = total_len;
    for (i, l) in n_strings.iter().enumerate() {
        if l < &(*k as usize) {
            l0 = i - 1; //As [i, L] has at least K distinct essenital types
            break;
        }
    }
    l0 + 1
}
/* Computes ranges (called deltas) upon which the elt's of SA[delta_l..delta_r]
 * correspond to K different input strings, meaning every delta range is K-good.
 */
pub fn compute_deltas(
    n_strings: &usize,
    l0: &usize,
    k: &u32,
    suffix_array: &RawSuffixArray,
    sentinel_pos: &Vec<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let start_ind = *n_strings;
    let end_ind = *l0 - 1;
    let mut delta_ls = vec![0usize; end_ind + 1 - start_ind];
    let mut delta_rs = vec![0usize; end_ind + 1 - start_ind];
    let mut type_counters = vec![0u32; *n_strings];

    delta_ls[0] = start_ind;
    delta_rs[0] = start_ind + 1;
    if let Some(s_ind) = get_string_index(suffix_array[start_ind] as usize, &sentinel_pos) {
        type_counters[s_ind] += 1;
    }

    //Advance until K-good
    while count_nonzero(&type_counters) < *k {
        delta_rs[0] += 1;
        let i = delta_rs[0] - 1; //The new character that is in delta_0
        if let Some(s_ind) = get_string_index(suffix_array[i] as usize, &sentinel_pos) {
            type_counters[s_ind] += 1;
        }
    }

    //delta_n+1 is now K-good, can advance
    for (j, i) in (start_ind..(end_ind + 1)).enumerate().skip(1) {
        delta_ls[j] = i;
        delta_rs[j] = delta_rs[j - 1];
        //Remove one from the type counter corresponding to the i-1-th char
        if let Some(ls_ind) = get_string_index(suffix_array[i - 1] as usize, &sentinel_pos) {
            type_counters[ls_ind] -= 1;
        }
        while count_nonzero(&type_counters) < *k {
            let new_char_ind = delta_rs[j];
            delta_rs[j] += 1;
            if let Some(s_ind) =
                get_string_index(suffix_array[new_char_ind] as usize, &sentinel_pos)
            {
                type_counters[s_ind] += 1;
            }
        }
    }

    (delta_ls, delta_rs)
}

/* Given delta ranges, find the delta with the largest min_lcp value where the min_lcp
 * is the minimum of the lcp array values present within a given delta.
 * This corresponds to the length of the longest substring
 * Returns a tuple of the form (maxi, maxv) where maxi is the index of min-max delta and
 * maxv is the minimum lcp value in delta_maxi
 */
pub fn max_min_lcp(
    delta_ls: &Vec<usize>,
    delta_rs: &Vec<usize>,
    lcp_array: &LCPArray,
) -> (usize, isize) {
    let result = delta_ls
        .iter()
        .zip(delta_rs)
        .enumerate()
        .map(|(i, (&l, &r))| {
            let mut min = isize::MAX;
            for k in (l + 1)..r {
                if let Some(v) = lcp_array.get(k) {
                    min = if v < min { v } else { min }
                }
            }
            (i, min)
        });

    let mut maxi = 0usize;
    let mut maxv = 0isize;
    for (i, lcp_min) in result {
        if lcp_min > maxv {
            maxv = lcp_min;
            maxi = i;
        }
    }

    (maxi, maxv)
}

/* Computes the lcp array values in O(n) time, considering all sentinels as unique characters.
 */
pub fn lcp_unique_sentinels<SA: Deref<Target = RawSuffixArray>>(text: &[u16], pos: SA) -> LCPArray {
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
        while pred + l < n
            && p + l < n
            && text[p + l] == text[pred + l]
            && text[p + 1] != sentinel
            && text[pred + l] != sentinel
        {
            l += 1;
        }
        lcp.set(r, l as isize);
        l = if l > 0 { l - 1 } else { 0 };
    }

    lcp
}

/* Construct the SuffixArray using u16 instead of u8 to allow for a unique sentinel,
 * assuming the sentinel is 0u16
 */
pub fn suffix_array_u16(text: &[u16], n_sentinels: &u16) -> RawSuffixArray {
    let n = text.len();
    let alphabet = sais::Alphabet::new(text);
    let mut sais = sais::SAIS::new(n);
    let sentinel_count = *n_sentinels as usize;

    match alphabet.len() + (*n_sentinels as usize) {
        a if a <= std::u8::MAX as usize => sais.construct(&sais::transform_text::<u8>(
            text,
            &0,
            &alphabet,
            sentinel_count,
        )),
        a if a <= std::u16::MAX as usize => sais.construct(&sais::transform_text::<u16>(
            text,
            &0,
            &alphabet,
            sentinel_count,
        )),
        a if a <= std::u32::MAX as usize => sais.construct(&sais::transform_text::<u32>(
            text,
            &0,
            &alphabet,
            sentinel_count,
        )),
        _ => sais.construct(&sais::transform_text::<u64>(
            text,
            &0,
            &alphabet,
            sentinel_count,
        )),
    }

    sais.pos
}

fn count_nonzero(vec: &Vec<u32>) -> u32 {
    let mut c = 0;
    for i in vec {
        if i > &0 {
            c += 1
        }
    }
    c
}

pub fn get_string_index(pos: usize, sentinel_pos: &Vec<usize>) -> Option<usize> {
    let mut i = 0;
    while pos > sentinel_pos[i] {
        i += 1;
    }

    if sentinel_pos[i] == pos {
        None
    } else {
        Some(i)
    }
}
