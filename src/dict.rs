use std::collections::HashMap;

use crate::variation::Variation;

/// Stores a list of words in toki pona, including variants for
/// other orthographic systems such as tipunsin and CJK characters.
#[derive(Debug)]
pub struct Dictionary<'a> {
    default: DefaultDictionary<'a>,
    variations: HashMap<Variation, VariationDictionary<'a>>,
}

lazy_static::lazy_static! {
    pub static ref PU: Dictionary<'static> = Dictionary::from_csv(include_str!("dicts/pu.csv"));
}

/// Represents the dictionary of words for the default orthographic system
/// for toki pona. This is the version toki pona that is most commonly seen
/// online and in pu/ku.
#[derive(Debug)]
struct DefaultDictionary<'a> {
    /// A list of words, written out in toki pona.
    words: Vec<&'a str>,
}

/// Represents the dictionary of words for a specific variation of toki pona.
#[derive(Debug)]
struct VariationDictionary<'a> {
    /// If a word does not exist in this variation, None is returned.
    words: Vec<Option<&'a str>>,
}

impl<'a> Dictionary<'a> {
    /// Parses a CSV file containing a dictionary of toki pona words.
    /// The first row states the variation, and subsequent lines are words in each
    /// variation. The default variation may not have empty entries.
    /// No quote marks or extra commas are expected.
    pub fn from_csv(text: &'a str) -> Self {
        let mut lines = text.lines();
        let mut headers = lines
            .next()
            .expect("expected headers listing variations")
            .split(",");
        assert_eq!(headers.next(), Some("tp"), "expected default variation");

        let mut default = DefaultDictionary { words: Vec::new() };
        let mut variations = Vec::new();
        let mut variation_names = Vec::new();
        for variation in headers {
            variations.push(VariationDictionary { words: Vec::new() });
            variation_names.push(variation.try_into().expect("not a known variation"));
        }

        for record in lines {
            let mut words = record.split(",");
            default.words.push(words.next().unwrap());
            for (item, variation) in words.zip(&mut variations) {
                variation
                    .words
                    .push(if item.is_empty() { None } else { Some(item) });
            }
        }

        Dictionary {
            default,
            variations: variation_names.into_iter().zip(variations).collect(),
        }
    }
}
