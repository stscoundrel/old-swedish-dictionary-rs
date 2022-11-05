use std::fs::read_to_string;
use std::path::Path;

fn is_json_file(filename: &str) -> bool {
    Path::new(&filename).extension().unwrap().eq("json")
}

pub fn read_json_file(filename: &str) -> Result<String, &'static str> {
    if is_json_file(filename) {
        return match read_to_string(filename) {
            Ok(text) => Ok(text),
            Err(_e) => Err("Could not read the given JSON file"),
        }
    }

    Err("Given file was not a JSON file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_json_files() {
        let filename1 = "foo.json";
        let filename2 = "foo.gson";
        let filename3 = "virus.exe";

        let result1 = is_json_file(&filename1);
        let result2 = is_json_file(&filename2);
        let result3 = is_json_file(&filename3);

        assert_eq!(result1, true);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }

    #[test]
    fn errors_on_non_json_files() {
        let filename = "undefined.txt";

        let result = read_json_file(filename);

        assert_eq!(result, Err("Given file was not a JSON file"));
    }

    #[test]
    fn errors_on_invalid_json_files() {
        let filename = "undefined.json";

        let result = read_json_file(filename);

        assert_eq!(result, Err("Could not read the given JSON file"));
    }

    #[test]
    fn reads_json_file() {
        let filename = "src/reader/fixtures/test.json";

        let result = read_json_file(filename);
        let expected = String::from("{\n    \"headword\": \"abo mynt\",\n    \"partOfSpeech\": \"nn\",\n    \"grammaticalAspect\": \"\",\n    \"definitions\": [\n        \"Åbo-mynt, i Åbo slaget mynt. hundradha mark ok niyotigi mark peninga abo mynt FH3: 122 (  1448) . \"\n    ],\n    \"alternativeForms\": [\n        \"abo päningar , \"\n    ]\n}");

        assert_eq!(result, Ok(expected));
    }
}
