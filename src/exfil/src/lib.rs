use hex;

pub fn hash(x: String) {
    #[derive(Debug)]
    struct Data {
        ascii   : String,
        md2     : String,
    };

    let md2 = md2(x.clone());
    
    let mut stag = Data {
        ascii: String::from("test"),
        md2: String::from("A1"),
    };

    stag.ascii = String::from(x.clone());
    stag.md2 = String::from(md2);

    println!("{:?}", stag);
}


fn md2(s: String) -> String {
    use md2::{Md2, Digest};
    let mut hasher = Md2::new();
    hasher.update(s);
    let result = hasher.finalize();
    let out = hex::encode(result);
    return out;
}
