mod dictionary_entry;
pub use dictionary_entry::DictionaryEntry;
use crate::reader;

/// Get full list of dictionary entries.
///
/// 
/// # Examples
/// 
/// ```
/// use old_swedish_dictionary::{get_dictionary, DictionaryEntry};
/// 
/// let dictionary: Vec<DictionaryEntry> = get_dictionary().unwrap();
/// 
/// println!("First word is {}, first definition for it being {}", &dictionary[0].headword, &dictionary[0].definitions[0])
/// ```
pub fn get_dictionary() -> Result<Vec<DictionaryEntry>, &'static str> {
    let contents = reader::read_dictionary_json();

    match serde_json::from_str(contents){
        Ok(entries) => Ok(entries),
        Err(_e) => Err("Failed to serialize dictionary to DictionaryEntries"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_has_correct_amount_of_entries() {
        let result = get_dictionary();
        let dictionary = result.unwrap();

        assert_eq!(dictionary.len(), 41744);
    }

    #[test]
    fn dictionary_has_expected_entry_content() {
        let result = get_dictionary();
        let dictionary = result.unwrap();
        let entry: &DictionaryEntry = &dictionary[1989];

        assert_eq!(entry.headword, "barklös");
        assert_eq!(entry.part_of_speech[0], "av");
        assert_eq!(entry.part_of_speech.len(), 1);
        assert_eq!(entry.grammatical_aspect, "adj.");
        assert_eq!(entry.information, "");
        assert_eq!(entry.definitions[0], "saknande bark.  Lg 91 .");
        assert_eq!(entry.definitions.len(), 1);
        assert_eq!(entry.alternative_forms.len(), 0);
    }    
}