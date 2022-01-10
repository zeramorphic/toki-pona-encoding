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
    pub fn get_identifier_variation(
        &self,
        word: &str,
        variation: Variation,
    ) -> Option<WordIdentifier> {
        if variation == Variation::Default {
            return self.get_identifier(word);
        }

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
        self.get_identifier(word)
    }

    /// Looks up a toki pona word, written in the default orthography.
    pub fn get_identifier(&self, word: &str) -> Option<WordIdentifier> {
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

    /// Looks up a word identifier and returns the toki pona word in the given orthography.
    pub fn get_word_variation(&self, identifier: WordIdentifier, variation: Variation) -> &'a str {
        let dict = self.base_dictionaries[identifier.dict];
        if variation == Variation::Default {
            dict.default.words[identifier.word]
        } else if let Some(word) = dict.variations[&variation].words[identifier.word] {
            word
        } else {
            dict.default.words[identifier.word]
        }
    }

    /// Returns a list of bytes representing this word.
    pub fn word_to_bytes(&self, word: WordIdentifier) -> Vec<u8> {
        if word.dict < self.base_dictionaries.len() {
            let index = self
                .base_dictionaries
                .iter()
                .take(word.dict)
                .map(|dict| dict.default.words.len())
                .sum::<usize>()
                + word.word;
            vec![0x22 + u8::try_from(index).expect("index too large")]
        } else {
            todo!()
        }
    }

    /// Returns the word identifier representing this word.
    pub fn word_from_bytes(&self, bytes: &[u8]) -> WordIdentifier {
        if bytes.len() == 1 {
            // This is a single-byte word, which must be in the base dictionaries.
            let mut index = bytes[0] as usize;
            let mut dict_index = 0;
            while index >= self.base_dictionaries[dict_index].default.words.len() {
                index -= self.base_dictionaries[dict_index].default.words.len();
                dict_index += 1;
            }
            WordIdentifier {
                dict: dict_index,
                word: index,
            }
        } else {
            todo!()
        }
    }
}
