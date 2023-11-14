// Receives a string and returns the Vigenere cipher of that string
fn main() {
    let plaintext = "ATTACKATDAWN";
    let key = "LEMON";
    let ciphertext = vigenere(plaintext, key);
    println!("{}", ciphertext);
}

fn vigenere(p0: &str, p1: &str) -> String {
    let mut ciphertext = String::new();
    let mut i = 0;
    for c in p0.chars() {
        let k = p1.chars().nth(i).unwrap();
        let c = shift(c, k);
        ciphertext.push(c);
        i = (i + 1) % p1.len();
    }
    ciphertext
}

fn shift(p0: char, p1: char) -> char {
    let a = 'A' as u8;
    let p0 = p0 as u8;
    let p1 = p1 as u8;
    let c = ((p0 - a) + (p1 - a)) % 26 + a;
    c as char
}

