use std::fmt;
use std::io;

use crate::{
    dict_set::{WordIdentifier, DICT_SET},
    variation::Variation,
};

/// A passage of text is comprised of a list of instructions.
/// Each instruction may write some Unicode output, or alter some internal state.
#[derive(Debug)]
pub enum Instruction {
    /// Print a word in toki pona from the main dictionary set
    /// using the active variation.
    TokiPonaWord { word: WordIdentifier },
    /// Instead of prepending a space before the subsequent word, attach the words together.
    AttachToPrevious,
}

impl Instruction {
    fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::TokiPonaWord { word } => DICT_SET.word_to_bytes(*word),
            Instruction::AttachToPrevious => vec![0x21],
        }
    }
}

/// The internal state of an encoder or decoder.
#[derive(Debug, Default)]
struct EncodingState {
    /// What orthography variation is active?
    variation: Variation,
    /// If this is true, before printing the next TokiPonaWord, a space will be prepended.
    prepend_space: bool,
}

/// Encodes text into the toki pona encoding.
#[derive(Debug)]
pub struct Encoder<T: io::Write> {
    writer: T,
    state: EncodingState,
    /// We may have some text that is not encoded yet, but that might
    /// be more efficiently encoded later. This unencoded text is stored here.
    /// Stored as a Vec<char> for convenience; UTF-8 is likely unnecessary.
    unencoded: Vec<char>,
}

impl<T: io::Write> Encoder<T> {
    /// Encodes input text and writes it to the given writer.
    pub fn new(writer: T) -> Self {
        Self {
            writer,
            state: EncodingState::default(),
            unencoded: Vec::new(),
        }
    }

    /// Process the given text and output the generated instructions to the writer.
    pub fn write_text(&mut self, text: &str) {
        for c in text.chars() {
            self.write_character(c);
        }
    }

    pub fn write_character(&mut self, c: char) {
        if c == ' ' || self.unencoded.len() >= 16 {
            self.encode();
        }
        self.unencoded.push(c);
    }

    /// Encode what remains in the unencoded text.
    /// After this method, the unencoded text will be empty.
    fn encode(&mut self) {
        // Check if the buffer represents a word.
        let manually_attach_to_previous;
        // If the buffer represents a toki pona word, this string is that word.
        // Space-handling is different for non-toki-pona words.
        let toki_pona_word = {
            let mut chars = self.unencoded.iter().peekable();
            if self.state.prepend_space {
                manually_attach_to_previous = !matches!(chars.peek(), Some(' '));
                if !manually_attach_to_previous {
                    // This space character is implicit, so ignore it.
                    chars.next().unwrap();
                }
            } else {
                manually_attach_to_previous = false;
            }
            chars.collect::<String>()
        };

        if let Some(word) = DICT_SET.get_identifier_variation(&toki_pona_word, self.state.variation)
        {
            // This was a toki pona word.

            // The previous word was a toki pona word, but there was no space between this word and the previous.
            if manually_attach_to_previous {
                // We expected a space character, but one was not given.
                // We must emit an instruction to attach this word to the previous
                // word when decoding.
                self.write(Instruction::AttachToPrevious);
            }

            // io::Write the instruction to the writer.
            self.write(Instruction::TokiPonaWord { word });
            // If the next word is a toki pona word, we will expect to put a space before it.
            self.state.prepend_space = true;
        } else {
            panic!(
                "encoding failed for character sequence [{}]",
                toki_pona_word
            );
        }

        self.unencoded.clear();
    }

    fn write(&mut self, instruction: Instruction) {
        self.writer
            .write_all(&instruction.encode())
            .expect("writing failed");
    }
}

impl<T: io::Write> Drop for Encoder<T> {
    fn drop(&mut self) {
        // Process the remaining unencoded text.
        self.encode();
    }
}

/// Decodes text from the toki pona encoding into a io::Writer.
#[derive(Debug)]
pub struct Decoder<T> {
    writer: T,
    state: EncodingState,
}

impl<T: fmt::Write> Decoder<T> {
    /// Decodes input bytes and writes the resultant text to the given writer.
    pub fn new(writer: T) -> Self {
        Self {
            writer,
            state: EncodingState::default(),
        }
    }

    /// Process the given text and output the encoded text to the writer.
    pub fn read_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.read_byte(*byte);
        }
    }

    pub fn read_byte(&mut self, byte: u8) {
        match byte {
            0x21 => self.execute(Instruction::AttachToPrevious),
            _ if 0x22 <= byte => {
                // This is a single-byte toki pona word.
                self.execute(Instruction::TokiPonaWord {
                    word: DICT_SET.word_from_bytes(&[byte - 0x22]),
                });
            }
            _ => panic!("unexpected byte {:#x?}", byte),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TokiPonaWord { word } => {
                if self.state.prepend_space {
                    self.write(" ");
                }
                self.write(DICT_SET.get_word_variation(word, self.state.variation));
                self.state.prepend_space = true;
            }
            Instruction::AttachToPrevious => {
                self.state.prepend_space = false;
            }
        }
    }

    fn write(&mut self, string: &str) {
        self.writer.write_str(string).expect("writing failed")
    }
}
