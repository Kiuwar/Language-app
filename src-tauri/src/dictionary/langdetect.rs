use whatlang::{detect, Lang};

/// Detecta o idioma de um trecho de texto. Retorna `None` quando a detecção
/// não é confiável (texto curto demais, por exemplo) — nesse caso quem
/// chama deve assumir um idioma padrão (inglês) em vez de travar a busca.
pub fn detect_language(text: &str) -> Option<&'static str> {
    let info = detect(text)?;
    if !info.is_reliable() {
        return None;
    }
    match info.lang() {
        Lang::Eng => Some("en"),
        Lang::Por => Some("pt"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_english_sentence() {
        assert_eq!(
            detect_language(
                "I need to study for my law exam this weekend because the test is important."
            ),
            Some("en")
        );
    }

    #[test]
    fn detects_portuguese_sentence() {
        assert_eq!(
            detect_language("Esta é uma frase razoavelmente longa em português."),
            Some("pt")
        );
    }
}
