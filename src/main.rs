use toki_pona_encoding::encoding::{Decoder, Encoder};

fn main() {
    let mut encoded = Vec::new();
    let mut encoder = Encoder::new(&mut encoded);
    encoder.write_text("alasa akesi a lukin oko");
    drop(encoder);

    println!("encoded: {:x?}", encoded);

    let mut decoded = String::new();
    let mut decoder = Decoder::new(&mut decoded);
    decoder.read_bytes(&encoded);
    println!("decoded: {}", decoded);
}
