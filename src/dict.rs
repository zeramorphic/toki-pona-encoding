use std::collections::HashMap;

use crate::variation::Variation;

/// Stores a list of words in toki pona, including variants for
/// other orthographic systems such as tipunsin and CJK characters.
#[derive(Debug)]
pub struct Dictionary<'a> {
    pub default: DefaultDictionary<'a>,
    pub variations: HashMap<Variation, VariationDictionary<'a>>,
}

lazy_static::lazy_static! {
    pub static ref PU: Dictionary<'static> = Dictionary::from_csv(include_str!("dicts/pu.csv"));
}

/// Represents the dictionary of words for the default orthographic system
/// for toki pona. This is the version toki pona that is most commonly seen
/// online and in pu/ku.
#[derive(Debug)]
pub struct DefaultDictionary<'a> {
    /// A list of words, written out in toki pona.
    pub words: Vec<&'a str>,
    /// Maps a word to its index in the dictionary.
    pub lookup: HashMap<&'a str, usize>,
}

/// Represents the dictionary of words for a specific variation of toki pona.
#[derive(Debug)]
pub struct VariationDictionary<'a> {
    /// If a word does not exist in this variation, None is returned.
    pub words: Vec<Option<&'a str>>,
    /// Maps a word to its index in the dictionary.
    pub lookup: HashMap<&'a str, usize>,
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
            .split(',');
        assert_eq!(headers.next(), Some("tp"), "expected default variation");

        let mut default = DefaultDictionary {
            words: Vec::new(),
            lookup: HashMap::new(),
        };
        let mut variations = Vec::new();
        let mut variation_names = Vec::new();
        for variation in headers {
            variations.push(VariationDictionary {
                words: Vec::new(),
                lookup: HashMap::new(),
            });
            variation_names.push(variation.try_into().expect("not a known variation"));
        }

        for (i, record) in lines.enumerate() {
            let mut words = record.split(',');
            let word = words.next().unwrap();
            default.lookup.insert(word, i);
            default.words.push(word);

            for (word, variation) in words.zip(&mut variations) {
                if !word.is_empty() {
                    variation.lookup.insert(word, i);
                    variation.words.push(Some(word));
                }
            }
        }

        Dictionary {
            default,
            variations: variation_names.into_iter().zip(variations).collect(),
        }
    }
}
