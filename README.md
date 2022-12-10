# Old Swedish Dictionary

Old Swedish Dictionary for Rust. The dictionary consists of 40 000+ Old Swedish words with Swedish translations.

Based on K.F. Söderwall's Medieval Swedish Dictionary

### Install

`cargo add old_swedish_dictionary`

Or add this to your `Cargo.toml`:

```toml
[dependencies]
old_swedish_dictionary = "0.2.0"
```

### Usage


```rust
// Ships getter for dictionary, plus DictionaryEntry.
use old_swedish_dictionary::{get_dictionary, DictionaryEntry};

let dictionary = get_dictionary();

// Both methods return Result, which should always be safe to unwrap.
// Up to you if you wish to just unwrap, or use other error handling method.
let dictionary_content: Vec<DictionaryEntry> = dictionary.unwrap();

println!("A word from dictionary: {}. First definition for it is: {}", &dictionary_content[0].headword, &dictionary_content[0].definitions[0])
```

### About "Dictionary of Old Swedish"

_"Ordbok Öfver svenska medeltids-språket"_ dictionary was published in late 1884—1918 by K.F. Söderwall. Additional supplement to it was published in 1953—1973.

Old Swedish developed from Old East Norse, the eastern dialect of Old Norse, at the end of the Viking Age. Early Old Swedish was spoken from about 1225 until about 1375, and Late Old Swedish was spoken from about 1375 until about 1526.

The original material is licenced under [Creative Commons International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/), made available by University of Gothenburg. The source code for this library is under MIT licence.

- https://spraakbanken.gu.se/en/resources/soederwall
- https://spraakbanken.gu.se/en/resources/soederwall-supp

