# build-dictionary

Gera `data/dict/dictionary.db`, o recurso de dicionário empacotado com o
app (somente leitura, anexado como `dict` à `lexidesk.db` em runtime).

## Uso (dicionário semente atual)

```
python3 tools/build-dictionary/build.py
```

Lê `words.json` (lista curada à mão) e escreve `data/dict/dictionary.db`.
Ver `data/dict/LICENSES.md` para o status da fonte dos dados.

## Próximo passo: ingestão real do Wiktionary

Não implementado ainda. Plano: usar `wiktextract` (ou os extratos já
processados em kaikki.org) para o Wiktionary em inglês, filtrar entradas
com tradução para português, e gerar as mesmas tabelas
(`dictionary_entries`, `word_forms`, `dictionary_fts`) que este script
produz — o schema já é compatível, só muda a fonte dos dados.
