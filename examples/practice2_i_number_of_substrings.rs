use ac_library_rs::{lcp_array, suffix_array};
use std::io::Read;
use std::iter;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim();
    let suffix_array = suffix_array(s);
    let ans: u64 = iter::once(0)
        .chain(lcp_array(s, &suffix_array))
        .zip(suffix_array)
        .map(|(c, i)| (s.len() - i - c) as u64)
        .sum();
    println!("{}", ans);
}
