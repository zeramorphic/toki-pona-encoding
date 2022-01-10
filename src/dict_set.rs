use crate::{dict::*, variation::Variation};

/// Indexes all words and variants in toki pona.
/// This uses multiple dictionaries to categorise each word.
pub struct DictionarySet<'a> {
    /// A list of dictionaries whose words will be directly encoded as a single byte.
    base_dictionaries: Vec<&'a Dictionary<'a>>,
}

/// Uniquely identifies a word, and possibly a variant.
/// Linked to a single DictionarySet.
#[derive(Debug, Clone, Copy)]
pub struct WordIdentifier {
    /// Which dictionary is this word stored in?
    pub dict: usize,
    /// What index is this word at in the dictionary above?
    pub word: usize,
}

impl<'a> Default for DictionarySet<'a> {
    fn default() -> Self {
        Self {
            base_dictionaries: vec![&*PU],
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DICT_SET: DictionarySet<'static> = DictionarySet::default();
}

impl<'a> DictionarySet<'a> {
    /// Looks up a toki pona word, written in the given variation.
    /// If this lookup fails, the lookup will be retried in the default orthography.
    pub fn lookup_variation(&self, word: &str, variation: Variation) -> Option<WordIdentifier> {
        for (dict_idx, dict) in self.base_dictionaries.iter().enumerate() {
            if let Some(variation_dict) = dict.variations.get(&variation) {
                if let Some(result) = variation_dict.lookup.get(word) {
                    return Some(WordIdentifier {
                        dict: dict_idx,
                        word: *result,
                    });
                }
            }
        }
        self.lookup(word)
    }

    /// Looks up a toki pona word, written in the default orthography.
    pub fn lookup(&self, word: &str) -> Option<WordIdentifier> {
        for (dict_idx, dict) in self.base_dictionaries.iter().enumerate() {
            if let Some(result) = dict.default.lookup.get(word) {
                return Some(WordIdentifier {
                    dict: dict_idx,
                    word: *result,
                });
            }
        }
        None
    }
}
