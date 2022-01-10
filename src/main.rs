use toki_pona_encoding::{dict_set::DICT_SET, variation::Variation};

fn main() {
    println!(
        "{:#?}",
        "ti pun sin oko"
            .split(' ')
            .map(|word| DICT_SET.lookup_variation(word, Variation::Tipunsin))
            .collect::<Vec<_>>()
    );
}
