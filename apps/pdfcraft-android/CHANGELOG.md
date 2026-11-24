# Changelog

PdfMusicCraft's versions, newest first. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/). Downloads are on the
[Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases) page.

## [1.1.2] - 2026-11-24

**Uninstall the previous version first.** 1.1.2 is signed with a new key (the old one can no
longer be used), so Android won't install it over 1.1.1 or earlier. Uninstalling loses what the
app keeps inside it (concert lists, settings, recent files); your PDFs and the marks saved in
them stay. Obtainium will report a signature error: uninstall, then install again from
Obtainium. Later versions update normally.

### Changed
- The marks panel shows the signs themselves instead of their names: accent, staccato, tenuto,
  marcato, fermata, breath mark, caesura, up and down bow, pedal release, glasses, hairpins,
  slurs, segno and coda look on their buttons as they do on the score, in any language. With a mouse, hovering
  over one still names it. Dynamics, tempo words, fingering and letters stay as
  written, as they are in a score
  ([#5](https://github.com/PabloGonSan8/pdfmusiccraft/issues/5)).

## [1.1.1] - 2026-11-23

The app works as in 1.1.0; this version brings its number in line and adds the project's
documentation.

### Changed
- The app's version is 1.1.1, the same as its release. The release first tagged `1.1.1` held
  the 1.1.0 APK, so the website and Obtainium showed one number and the app another.

### Added
- Documentation: a PdfMusicCraft roadmap, a privacy page (permissions and what is stored), contributing
  and security guides, issue templates, and download details (architecture, size, SHA-256
  check, Obtainium) in the install guides.

## [1.1.0] - 2026-11-16

### Added
- The pencil has colours and widths: pencil grey, blue, red, green and **white**, which covers
  what is printed (a wrong note, a fingering to replace); the widest stroke is for covering.
- Note heads alone, filled and open, among the notes of the marks panel: to add a note to a
  chord or to write the right pitch over a covered one.

### Changed
- Move and Size are two tools. With one, a mark smaller than a fingertip was all resize handles,
  and a drag meant to move it resized it. With Move a drag only moves the mark; Size shows the
  handles, and Larger and Smaller.
- Fit fits the music of the page you are on across the screen, as when a score opens; a second
  tap shows that page whole. It used to fit the largest page of the PDF, which left the others
  short of the screen's edges.

### Fixed
- With the tablet held upright (or on a phone) the top bar drew its tiles over each other: Undo
  on top of a concert's Previous and Next. The bar, and the marks panel's row of tools, now take
  as many rows as they need.
- A score fitted to the screen stayed at that zoom when the tablet was turned; it is fitted again
  to the new width (unless you have zoomed since).
- A tap beside a mark with Move chosen washed the whole scanned page in blue.
- With a score open, "Open from…" on the Scores screen showed that score behind the file browser,
  and cancelling left you in it. The browser now opens over the Scores screen, which stays until
  you choose a PDF.

## [1.0.1] - 2026-11-11

### Fixed
- A new install listed no scores and opened no folders until "All files access" was found and
  turned on by hand in Android's settings. The app now explains it on first start and opens that
  setting with one tap; the scores appear as soon as you come back.

## [1.0.0] - 2026-11-11

The first public version.

### Scores
- The start screen lists the PDFs in Download, Documents and the app's folder, and the recent
  files, with search. Folders of PDFs show as concerts, gone through with Previous and Next.
- Choose several scores (Choose, or a long press) and take them off the list; Show hidden brings
  them back. The files stay on the device.
- Close all closes every open score; scores with unsaved marks ask first.

### Music view
- The score full screen, fitted to its music (a scan's specks don't count as music); Fit shows
  the whole sheet.
- Swipe sideways to turn the page; fitted, one finger moves up and down or sideways; zoomed in,
  freely. Two fingers always zoom and move, whatever tool is chosen.
- The pages scroll on past the last one, so its end can be marked above the marks panel.

### Marks
- Dynamics, hairpins, slurs, articulations, bowing and pedal, fingering, rehearsal letters,
  numbers and jumps (segno, coda, D.S., D.C., lettered jumps that a tap follows).
- Tempo headings (Grave to Presto), tempo changes (rit., rall., accel., a tempo, meno mosso,
  più mosso) and metronome marks for every note value, with the tempo chosen in the panel.
- Notes written into the score: whole to sixteenth, dotted, stem up or turned stem down.
- Move / resize: drag a mark, pull its corners, or use Larger and Smaller; a tap elsewhere lets
  it go.
- Pencil, highlighter (six colours, three widths) and eraser (three sizes).

### Full editor
- Every PdfCraft tool. The Music view button can be dragged out of the way, and a long press on
  a tab offers Close all.
