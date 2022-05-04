#[cfg(feature = "wasm")] 
use {
    best_macros::public_struct,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Locale<'a> {
    pub ru: &'a str,
    pub en: &'a str,
}

#[public_struct]
#[cfg(feature = "wasm")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LocaleExport {
    ru: String,
    en: String
}

#[cfg(feature = "wasm")]
impl<'a> Locale<'a> {
    pub fn export(&self) -> LocaleExport {
        LocaleExport { 
            ru: self.ru.to_string(),
            en: self.en.to_string() 
        }
    }
}