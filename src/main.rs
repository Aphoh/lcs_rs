use suffix::SuffixTable;
use std::*;
use std::string::String;
use std::collections::HashSet;

fn main() {
    let mut s1 = String::from("a huge big ol meme").into_bytes();
    let mut s2 = String::from("a big hume ol meme").into_bytes();
    let mut s3 = String::from(  "a big  oeoe meme").into_bytes();
    let non_ess_pos: HashSet<_> = vec![s1.len(), s2.len(), s3.len()].iter().cloned().collect();
    s1.push(0);
    s2.push(0);
    s3.push(0);
    s1.extend(s2);
    s1.extend(s3);

    let combined = as_hex(s1);
    print!("{}\n", combined);

    let st: SuffixTable = SuffixTable::new(combined);
    let lcp: Vec<u32> = st.lcp_lens();
   
    print!("{:?}\n", st);
    
}

fn as_hex(bytes: Vec<u8>) -> String {
    let mut res = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        res.push_str(format!("{:0>2X}", b).as_str());
    }
    res
}

