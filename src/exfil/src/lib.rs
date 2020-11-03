#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use hex;

pub fn hash(x: String) {
    md2(x);
}



fn md2(s: String) {
    use md2::{Md2, Digest};
    let mut hasher = Md2::new();
    hasher.update(s);
    let result = hasher.finalize();
    let out = hex::encode(result);
    println!("{}", out);
}