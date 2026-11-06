# Licenças e origem dos dados do dicionário

## Status atual: dicionário semente (placeholder)

O `dictionary.db` atual foi gerado por `tools/build-dictionary/build.py` a
partir de uma lista curada à mão (`tools/build-dictionary/words.json`,
~30 palavras comuns em inglês com tradução, classe gramatical, IPA e
exemplos escritos para este projeto). Não é um extrato do Wiktionary —
serve só para validar o pipeline de ponta a ponta (captura → lematização
→ busca → card → áudio).

## Próximo passo (ainda não feito)

A fonte definitiva combinada é o **Wiktionary** (inglês), processado
offline (ex. via `wiktextract`/kaikki.org) para gerar um `dictionary.db`
com cobertura real. Quando isso for feito, esta seção será atualizada
com:
- Atribuição exigida pela licença **CC BY-SA 3.0** do Wiktionary.
- Link para a versão/dump usado e para a licença completa.

## Piper (TTS)

Quando o áudio de pronúncia (Fase 3) for implementado, o motor **Piper**
(licença MIT) e o modelo de voz usado serão listados aqui com o aviso de
copyright exigido.

## Pacote de inglês jurídico

Ainda não incluído. Quando for adicionado, a fonte e a licença específicas
do glossário jurídico entram aqui antes de ficar disponível no app.
