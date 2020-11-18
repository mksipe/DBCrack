#[macro_use]
use serde::ser::{Serialize, SerializeStruct, Serializer};
use hex;

pub fn hash(x: String) {
    #[derive(Debug)]
    struct Data {
        ascii   : String,
        md2     : String,
    };

    let md2 = md2(x.clone());
    
    let mut stag = Data {
        ascii: String::from("test"), // this initialized the variables
        md2: String::from("A1"),
    };

    stag.ascii = String::from(x.clone());  // this sets the variables
    stag.md2 = String::from(md2);

    // Serialize the data to be exported
    
    impl Serialize for Data {
        fn serialize<S>(&self, serilizer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut s = serilizer.serialize_struct("Data", 3)?;
            s.serialize_field("ascii", &self.ascii)?;
            s.serialize_field("md2", &self.md2)?;
            s.end()
        }
    }
    
    //println!("{:?}", serilized);
}


fn md2(s: String) -> String {
    use md2::{Md2, Digest};
    let mut hasher = Md2::new();
    hasher.update(s);
    let result = hasher.finalize();
    let out = hex::encode(result);
    return out;
}

use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::{Read, BufReader};
 
fn entropy<I: IntoIterator<Item = u8>>(iter: I) -> f32 {
    let mut histogram = [0u64; 256];
    let mut len = 0u64;
 
    for b in iter {
        histogram[b as usize] += 1;
        len += 1;
    }
 
    histogram
        .iter()
        .cloned()
        .filter(|&h| h > 0)
        .map(|h| h as f32 / len as f32)
        .map(|ratio| -ratio * ratio.log2())
        .sum()
}
//nice

use std::process;
 
pub fn calcent(name: String) {
    //let name = std::env::args().nth(0).expect("Could not get program name.");
    let file = BufReader::new(File::open(name).expect("Could not read file."));
    for line in file.lines() {
        let mut l = line.unwrap();
        let entr: f32 = entropy(l.bytes());
        //println!("{:?}", l); //debug function
        //println!("{}", entr); //debug function
        if entr > 3.5 {
            if entr < 5.0{
                println!("Transisional:{}:{}",l,entr);
                process::exit(0);
            }
        println!("Sequential:{}:{}",l,entr);
        }
        if entr > 5.0 {
            println!("Encrypted:{}:{}", l,entr);
        }
        else{
            println!("{}:{}:{}", "Error",entr,l);
        }

    }

}