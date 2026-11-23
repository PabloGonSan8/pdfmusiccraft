# PdfMusicCraft roadmap

**English** · [Español](#español)

Where PdfMusicCraft is and where it may go. It covers the Android app only; the root
[ROADMAP.md](../../ROADMAP.md) is PdfCraft's (the engine this app is built on).

- ✅ **Shipped**: in a published release ([CHANGELOG.md](CHANGELOG.md)).
- 🔧 **Planned**: a known gap, to be fixed.
- 💡 **Proposed**: an idea, not a promise. Comments and votes on the
  [issues](https://github.com/PabloGonSan8/pdfmusiccraft/issues) decide what comes first.

Nothing is in active development right now beyond what is on `main`. When work starts on an
item, it gets an issue, linked here.

## ✅ Shipped (1.1.1)

**Scores**
- Scores screen: PDFs in Download, Documents and the app's folder, and recent files, with search.
- Concerts: every folder of PDFs, gone through with Previous and Next or by swiping past the last
  page.
- Hide scores from the list (the files stay); Show hidden; Close all.

**Reading**
- Music view: full screen without system bars, fitted to the music of the page; Fit shows the
  whole page.
- Swipe to turn the page, drag (with a glide), pinch to zoom; fits again when the tablet turns.
- Portrait and landscape; the bars wrap onto more rows on narrow screens.

**Marks** (drawn into the PDF, visible in any reader)
- Dynamics, hairpins, slurs, tempo headings and changes, metronome marks, articulations, bowing,
  pedal, fingering, rehearsal letters, numbers.
- Jumps (segno, coda, D.S., D.C., lettered jumps) that a tap follows.
- Written notes (whole to sixteenth, dotted, stem up or down) and loose note heads.
- Move and Size tools, with corner handles.
- Pencil (five colours, white to cover print, several widths), highlighter, eraser.

**Everything else**
- The full PdfCraft editor one tap away (pages: reorder, rotate, delete).
- English and Spanish interface, following the device language.
- Offline, no account, no internet permission ([PRIVACY.md](../../PRIVACY.md)).

## 🔧 Planned: known gaps

- **Printing** on Android: the dialog shows, but it isn't connected to Android's print service.
- **Open with**: opening a PDF from another app (Files, email, a browser download).
- **Storage Access Framework**: choosing files and folders with Android's own picker, so the
  app could work without *All files access*. See the study in [README.md](README.md#storage-access-framework).
- **Other processors**: only 64-bit ARM is built today; an x86_64 build would cover emulators and
  some Chromebooks.

## 💡 Proposed

### Music tools
- More marks: trills and ornaments (mordent, turn), glissando, tremolo, 8va/8vb, string numbers,
  breath and phrasing marks for singers and winds, chord symbols.
- Text marks with a musical font for free text (cues, lyrics corrections).
- Mark presets per instrument (a string player's palette, a pianist's palette).

### Tablets and styluses
- Stylus support: today a stylus acts as a finger. Ideas: draw with the pen while the finger
  moves the page, pressure for pencil width, palm rejection, the pen's button as an eraser.
- Bluetooth page-turn pedals (which act as a keyboard): test them in the music view and make
  sure they turn the page.
- Two pages side by side in landscape on large tablets.
- Half-page turns, so the next system is visible before the turn.

### Managing scores
- Setlists: an ordered list of scores from any folder, not only a folder's own order.
- Sort and filter the Scores screen (by name, date, composer folder).
- Bookmarks inside long scores (movements, sections).
- Rename, move and duplicate scores from the app.

### Experience
- A short first-run tour of the music view and the marks panel.
- Larger touch targets and a high-contrast option for the stage.
- Night mode for the page (inverted or sepia), for dark pits.
- More interface languages (through [craft-i18n](https://github.com/storytold/craft-i18n)).

### Distribution
- Keep GitHub Releases and the website as the main channels, and Obtainium for updates.
- Study F-Droid or IzzyOnDroid (see [INSTALL.md](INSTALL.md#other-stores)). Not on Google Play,
  by choice.

---

## Español

- ✅ **Hecho**: está en una versión publicada ([CHANGELOG.md](CHANGELOG.md)).
- 🔧 **Previsto**: un hueco conocido, por arreglar.
- 💡 **Propuesta**: una idea, no un compromiso. Lo que se comente y vote en las
  [incidencias](https://github.com/PabloGonSan8/pdfmusiccraft/issues) decide el orden.

Ahora mismo no hay nada en desarrollo aparte de lo que está en `main`. Cuando se empiece algo,
tendrá su incidencia, enlazada aquí.

### ✅ Hecho (1.1.1)

- **Partituras**: pantalla Partituras con buscador y recientes; conciertos por carpetas; ocultar
  partituras de la lista; Cerrar todas.
- **Lectura**: vista de música a pantalla completa, ajustada a la música; pasar página
  deslizando, arrastrar, pellizcar; vertical y horizontal.
- **Marcas** dentro del PDF: dinámicas, reguladores, ligaduras, tempo, metrónomo,
  articulaciones, arco, pedal, digitación, letras de ensayo, números, saltos que se siguen,
  notas escritas y cabezas sueltas; herramientas Mover y Tamaño; lápiz (cinco colores, blanco
  para tapar), subrayador y goma.
- **Lo demás**: el editor completo de PdfCraft; interfaz en inglés y español; sin conexión, sin
  cuenta, sin permiso de internet ([PRIVACY.md](../../PRIVACY.md#español)).

### 🔧 Previsto

- **Imprimir** en Android (el diálogo sale, pero no está conectado al servicio de impresión).
- **Abrir con**: abrir un PDF desde otra app.
- **Storage Access Framework**: el selector de archivos de Android, para no necesitar *Acceso a
  todos los archivos* ([estudio](README.md#storage-access-framework)).
- **Otros procesadores**: hoy solo se compila para ARM de 64 bits.

### 💡 Propuestas

- **Herramientas musicales**: trinos y ornamentos, glissando, trémolo, 8va/8vb, números de
  cuerda, cifrado americano, texto libre con fuente musical, paletas por instrumento.
- **Tabletas y lápices**: hoy un lápiz digital actúa como un dedo. Ideas: dibujar con el lápiz y
  mover con el dedo, presión, rechazo de la palma, botón del lápiz como goma; probar y asegurar
  los pedales Bluetooth para pasar página; dos páginas en horizontal; medias páginas.
- **Gestión de partituras**: listas de concierto de cualquier carpeta, ordenar y filtrar,
  marcadores dentro de una obra, renombrar y mover desde la app.
- **Experiencia**: un recorrido inicial, botones más grandes y alto contraste, modo noche para la
  página, más idiomas.
- **Distribución**: GitHub Releases y la web como canales principales, Obtainium para
  actualizar; estudiar F-Droid o IzzyOnDroid. No en Google Play, por decisión propia.
