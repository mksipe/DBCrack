#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use hex;
use std::collections::HashMap;

pub fn hash(x: String) {
    let mut calculated = HashMap::new();
    let md2 = md2(x.clone());
    calculated.insert(String::from("ASCII:"), x);
    calculated.insert(String::from("MD2:"), md2);
    for (a, b) in &calculated {
        println!("{}: {}", a,b);
    };
}


fn md2(s: String) -> String {
    use md2::{Md2, Digest};
    let mut hasher = Md2::new();
    hasher.update(s);
    let result = hasher.finalize();
    let out = hex::encode(result);
    return out;
}
