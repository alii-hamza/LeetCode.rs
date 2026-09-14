/*
You can run this code on:    play.rust-lang.org
*/

struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut out = String::new();
        for s in &strs {
            out.push_str(&format!("{}#{}", s.len(), s));
        }
        out
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let hash = s[i..].find('#').unwrap();
            let j = i + hash;
            let len: usize = s[i..j].parse().unwrap();
            let start = j + 1;
            let end = start + len;
            res.push(s[start..end].to_string());
            i = end;
        }
        res
    }
}

fn main() {
    let codec = Codec::new();

    let tests = vec![
        vec!["Hello".to_string(), "World".to_string()],
        vec!["".to_string()],
        vec!["#".to_string(), "a#b#c".to_string()],
    ];

    for input in tests {
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded.clone());
        println!("Input: {:?} -> Encoded: {:?} -> Decoded: {:?}", input, encoded, decoded);
        assert_eq!(input, decoded);
    }
    println!("All passed!");
}




/*
There is also a trick...
*/

struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut out = String::new();
        for s in strs{
            out.push_str(&format!("{}#{}", s.len(),s));
        }
        out
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        while i < s.len(){
            // let hash = s[i..].find("#").unwrap();
            let j = i + 1;
            let len: usize = s[i..j].parse().unwrap();
            let start = j + 1;
            let end = start + len;
            res.push(s[start..end].to_string());
            i = end;
        }
        res
    }
}

fn main() {
    let codec = Codec::new();

    let tests = vec![
        vec!["Hello".to_string(), "World".to_string()],
        vec!["".to_string()],
        vec!["#".to_string(), "a#b#c".to_string()],
        vec!["#1#".to_string(), "3a##2#c".to_string()],
    ];

    for input in tests {
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded.clone());
        println!("Input: {:?} -> Encoded: {:?} -> Decoded: {:?}", input, encoded, decoded);
        assert_eq!(input, decoded);
    }
    println!("All passed!");
}
