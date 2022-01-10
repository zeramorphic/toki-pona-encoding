/// Each variation is a specific orthographic system for toki pona.
/// These orthographic systems may each contain their own options for
/// customising the text that is encoded.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Variation {
    Default,
    Tipunsin,
    Hanzi,
}

/// Converts from variation codes such as "tp_ZH".
impl<'a> TryFrom<&'a str> for Variation {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "tp" => Self::Default,
            "tp_S" => Self::Tipunsin,
            "tp_ZH" => Self::Hanzi,
            _ => return Err(()),
        })
    }
}
