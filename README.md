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

Fase 0 concluída: banco SQLite (`lexidesk.db`, migrações via `rusqlite_migration`) + navegação principal (Hoje, Revisar, Palavras, Mapa, Configurações) + comando `db_health` validando o pipeline React → Rust → SQLite. Testado de ponta a ponta (build + execução real da janela). Próximas fases: interface completa, dicionário flutuante, áudio, revisão (SM-2), OCR, exportações, configurações, bandeja/autostart, auto-update — conforme o plano combinado.

## Licenças de dados

Dados de dicionário usados pelo app e suas licenças serão listados em `data/dict/LICENSES.md` quando a fonte for definida.
