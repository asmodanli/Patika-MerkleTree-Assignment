// Import Crates
use std::fs;

use sha2::{Sha256, Digest};

fn merkle_root(filename: String) -> String {

    // Read Input Data from txt file
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    println!("{contents}");

    
    // got words
    let splitted_words = contents.split("\n");
    
    // got vector of words
    let mut words = splitted_words.collect::<Vec<&str>>();

    // remove number
    let number  = words.remove(0);

    // Create vector of strings for leaves
    let mut hashed_words : Vec<String> = Vec::new();

    // push hashed inputs to vector
    for word in words{
        println!("{word}");
        hashed_words.push(hash_single_input(word));
    }

    // take first two member of vector
    // concat and hash members then push to vector
    // continues until there is only one element int the vector
    while hashed_words.len() != 1 {
        let mut first_member = hashed_words.remove(0).to_string();
        let  second_member = hashed_words.remove(0).to_string();
        println!("{second_member}");
        first_member.push_str(&second_member);
        hashed_words.push(hash_single_input(&first_member));
    }

    let final_hash = hashed_words.remove(0);
    println!("{final_hash}");
    return final_hash;

}


fn hash_single_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    let input = a;
    hasher.update(input);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash);

    return hex.to_string();
}

fn main() { 
merkle_root("input0.txt".to_string());
merkle_root("input1.txt".to_string());
merkle_root("input2.txt".to_string());
merkle_root("input3.txt".to_string());
merkle_root("input4.txt".to_string());

}



// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
