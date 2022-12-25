use old_swedish_dictionary::{get_dictionary, DictionaryEntry};
use insta::assert_json_snapshot;

#[test]
fn gets_default_dictionary() {    
    let result = get_dictionary().unwrap();

    assert_json_snapshot!(result)
}
#[test]
fn exports_dictionary_entry() {    
    let entry = DictionaryEntry {
        headword: String::from("Lorem ipsum"),
        part_of_speech: vec![
            String::from("vb.")
        ],
        grammatical_aspect: String::from("v"),
        information: String::from("Läs mer fra internet"),
        definitions: vec!(
            String::from("Dolor sit amet"),
            String::from("Dolor sit igitur"),
        ),
        alternative_forms: vec!(
            String::from("Lorem ipsum dolor sit amet"),
        )
    };

    assert_eq!(entry.headword, "Lorem ipsum");
    assert_eq!(entry.part_of_speech[0], "vb.");
    assert_eq!(entry.grammatical_aspect, "v");
    assert_eq!(entry.definitions[0], "Dolor sit amet");
    assert_eq!(entry.definitions[1], "Dolor sit igitur");
    assert_eq!(entry.alternative_forms[0], "Lorem ipsum dolor sit amet");
}