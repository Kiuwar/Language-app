# LexiDesk

App desktop (Windows) para consulta rápida de vocabulário e revisão de idiomas, construído com Tauri + React.

## Baixar o instalador Windows (sem compilar nada)

1. No GitHub, abra a aba **Actions** do repositório.
2. Clique no workflow **Build Windows installer**.
3. Clique em **Run workflow** (botão à direita, branch `main`) e confirme.
4. Espere o job `build-windows` terminar (gera o `.msi`/`.exe`).
5. Abra a execução concluída e baixe o artefato **LexiDesk-Windows-installer** em "Artifacts".
6. Descompacte e execute o instalador no Windows.

Alternativa: criar uma tag `v0.1.0` e dar `git push --tags` também dispara o build automaticamente.

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

Esqueleto inicial do app (janela em branco). As funcionalidades (dicionário, revisão, mapa de vocabulário, OCR, exportações) entram em etapas, conforme o plano combinado.

## Licenças de dados

Dados de dicionário usados pelo app e suas licenças serão listados em `data/dict/LICENSES.md` quando a fonte for definida.
