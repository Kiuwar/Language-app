mod langdetect;
mod lemmatize;
mod lookup;
#[cfg(test)]
mod test_support;

pub use langdetect::detect_language;
pub use lemmatize::lemmatize;
pub use lookup::{is_known, lookup_entry, search_entries, DictionaryEntry};
