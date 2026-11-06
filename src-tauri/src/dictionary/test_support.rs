#![cfg(test)]

use rusqlite::Connection;

/// Conexão de teste com o dictionary.db semente anexado (o mesmo usado em
/// dev/produção) e uma tabela `user_words` mínima, só com as colunas que
/// `is_known` lê.
pub fn test_connection() -> Connection {
    let conn = Connection::open_in_memory().expect("conexão em memória");
    let dict_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/dict/dictionary.db");
    conn.execute("ATTACH DATABASE ?1 AS dict", [dict_path])
        .expect("anexar dictionary.db de teste");
    conn.execute_batch(
        "CREATE TABLE user_words (lemma TEXT, language_code TEXT, state TEXT);",
    )
    .expect("criar user_words de teste");
    conn
}
