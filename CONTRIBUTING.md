# Contributing to PdfMusicCraft

**English** · [Español](#español)

Thanks for helping. You don't need to write code to contribute: a clear bug report, a missing
musical mark, a translation fix or a test on your tablet all help.

## Reporting bugs and asking for features

- **Bugs**: [open a bug report](https://github.com/PabloGonSan8/pdfmusiccraft/issues/new?template=bug_report.yml).
  Say which app version, device and Android version you have, what you did, what you expected
  and what happened. A screenshot or a small PDF that shows it helps a lot (no personal
  documents, please).
- **Ideas**: [suggest a feature](https://github.com/PabloGonSan8/pdfmusiccraft/issues/new?template=feature_request.yml).
  Tell us your instrument and how you would use it on the stand.
- **Security problems**: follow [SECURITY.md](SECURITY.md) instead of opening a public issue.

Search the [existing issues](https://github.com/PabloGonSan8/pdfmusiccraft/issues?q=is%3Aissue)
first; if yours is there, add what you know to it. Planned work is in the
[roadmap](apps/pdfcraft-android/ROADMAP.md).

## Where the music code lives

Most of this repository is [PdfCraft](https://github.com/storytold/pdfcraft)'s engine and editor.
The PdfMusicCraft parts are:

| Path | What |
|---|---|
| [`apps/pdfcraft-android/`](apps/pdfcraft-android/) | The Android app: entry point, manifest settings, launcher icon, install guides, changelog, roadmap |
| [`crates/ui-egui/src/touch.rs`](crates/ui-egui/src/touch.rs) | The tablet interface: Scores screen, concerts, music view, gestures |
| [`crates/ui-egui/src/music_ui.rs`](crates/ui-egui/src/music_ui.rs) | The marks panel and its tools |
| [`crates/annot/src/appearance/music.rs`](crates/annot/src/appearance/music.rs) | How each musical mark is drawn into the PDF |

Changes to the rest of the engine are often better proposed to PdfCraft itself.

## Building

The APK: see [Building and installing](apps/pdfcraft-android/README.md#building-and-installing)
(Rust, the Android SDK and NDK 26.1, JDK 17 and `cargo-apk`).

The same code also builds and runs on the desktop, which is quicker for trying a change:

```sh
cargo run -p pdfcraft -- path/to/score.pdf
```

The music marks are under **Stamps ▸ Music** there.

## Pull requests

1. Open or comment on an issue first, so two people don't do the same work.
2. Branch from `main`; `main` only changes through pull requests.
3. Keep the change focused on one thing, and add a test that fails without it when you fix a bug.
4. Before opening the pull request, run on the crates you changed:

   ```sh
   cargo fmt --check
   cargo clippy -p <crate> --all-targets -- -D warnings
   cargo test -p <crate>
   ```

   and `cargo xtask assets` if you added or changed an image, icon or other asset.
5. Say in the description what you tested, and on which device if it is an Android change.

The project's full rules are in [AGENTS.md](AGENTS.md). The ones that matter most:

- **Never crash.** No `unwrap()`, `expect()` or `panic!` outside tests; PDFs from anywhere are
  treated as hostile input.
- **Assets** (icons, images, fonts) must be under an open licence and recorded in
  `ATTRIBUTION.toml`. Nothing from Adobe products, ever.
- **Don't change CI** in a feature or fix pull request.

By contributing you agree that your work is licensed under the project's licence, MIT OR
Apache-2.0.

---

## Español

Gracias por ayudar. No hace falta programar: un buen informe de fallo, una marca musical que
falta, una corrección de traducción o una prueba en tu tableta también ayudan.

### Fallos y peticiones

- **Fallos**: [abre un informe](https://github.com/PabloGonSan8/pdfmusiccraft/issues/new?template=bug_report.yml)
  con la versión de la app, el dispositivo y la versión de Android, qué hiciste, qué esperabas y
  qué pasó. Una captura o un PDF pequeño que lo muestre ayuda mucho (sin documentos personales).
- **Ideas**: [propón una mejora](https://github.com/PabloGonSan8/pdfmusiccraft/issues/new?template=feature_request.yml).
  Cuenta qué instrumento tocas y cómo lo usarías en el atril.
- **Problemas de seguridad**: sigue [SECURITY.md](SECURITY.md) en vez de abrir una incidencia
  pública.

Busca antes en las [incidencias abiertas](https://github.com/PabloGonSan8/pdfmusiccraft/issues?q=is%3Aissue);
si la tuya ya está, añade lo que sepas. Lo previsto está en la
[hoja de ruta](apps/pdfcraft-android/ROADMAP.md).

### Código

La tabla de arriba indica dónde está el código musical; el resto del repositorio es el motor de
PdfCraft. Para compilar el APK, mira
[Building and installing](apps/pdfcraft-android/README.md#building-and-installing); para probar
un cambio más rápido, `cargo run -p pdfcraft -- partitura.pdf` lo abre en el escritorio (las
marcas están en **Stamps ▸ Music**).

Para un pull request: comenta antes en una incidencia, parte de `main`, haz un cambio concreto con
su test, pasa `cargo fmt --check`, clippy y los tests de los crates que toques, y di qué probaste.
Las reglas completas están en [AGENTS.md](AGENTS.md).

Al contribuir aceptas que tu trabajo se publique con la licencia del proyecto, MIT OR Apache-2.0.
