# LexiDesk

App desktop (Windows) para consulta rápida de vocabulário e revisão de idiomas, construído com Tauri + React.

## Baixar e instalar (link fixo, mais simples)

1. Abra a aba **Releases** do repositório no GitHub.
2. Na release **LexiDesk (build mais recente)** (ou numa versão `vX.Y.Z`), baixe o `.exe` (NSIS) ou `.msi`.
3. Execute o instalador — ele cria o atalho no Menu Iniciar e já oferece para abrir o app ao terminar.

Esse link não expira (diferente dos artefatos de Actions, que somem depois de um tempo) e é atualizado a cada novo build disparado.

## Gerar um novo build (dispara a Release acima)

1. No GitHub, abra a aba **Actions** → workflow **Build Windows installer**.
2. Clique em **Run workflow** (branch `main`) e confirme.
3. Quando o job `build-windows` terminar, a Release "LexiDesk (build mais recente)" é criada/atualizada automaticamente com o novo instalador.

Para uma versão "oficial" com link próprio (ex. `v0.1.0`), crie a tag e dê `git push --tags` — isso gera uma Release separada, não a de build mais recente.

## Rodar em desenvolvimento (PowerShell)

```powershell
npm install
npm run tauri dev
```

## Gerar o instalador localmente (Windows, PowerShell)

```powershell
npm install
npm run tauri build
```

O instalador fica em `src-tauri\target\release\bundle\msi\` e `...\bundle\nsis\`.

## Status

- **Fase 0**: banco SQLite (`lexidesk.db`) + navegação principal.
- **Fase 1**: paleta de comandos (Ctrl+K) + tema persistente (Sistema/Claro/Escuro).
- **Fase 2**: núcleo do dicionário — atalho global (`Ctrl+Alt+D`, Windows), janela flutuante pré-aquecida perto do cursor, lematização, busca (FTS5), card com tradução/IPA/exemplos. Dicionário atual é uma **lista semente** (~30 palavras, ver `data/dict/LICENSES.md`) — ingestão completa do Wiktionary é um próximo passo separado.

Próximas fases: áudio (Piper), salvar palavras, revisão (SM-2), mapa, OCR, exportações, configurações completas, bandeja/autostart, auto-update.

### O que é testável fora do Windows

A captura de texto selecionado em qualquer app (`Ctrl+C` simulado) só funciona no Windows de verdade — nesta sandbox Linux ela sempre retorna "sem seleção", o que ainda permite testar o resto do fluxo (atalho global, posicionamento da janela, busca manual, lematização, card). O código Windows-specific (`src-tauri/src/capture/selection.rs`) foi validado com `cargo check --target x86_64-pc-windows-gnu` (compila limpo), mas só roda de verdade no CI (`windows-latest`) ou numa máquina Windows.

## Licenças de dados

Dados de dicionário usados pelo app e suas licenças serão listados em `data/dict/LICENSES.md` quando a fonte for definida.
