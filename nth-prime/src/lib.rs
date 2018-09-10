extern crate primal;

pub fn nth(n: u32) -> u32 {
    // the tests provided assume 0 based index for n, while primal's nth_prime function assumes a 1 based indexed for n
    // therefore we need to add 1 to use the primal libraries
    let p = primal::StreamingSieve::nth_prime((n + 1) as usize);
    p as u32
}
