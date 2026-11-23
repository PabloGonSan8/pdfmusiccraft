# PdfMusicCraft

A personal adaptation of [PdfCraft](https://github.com/storytold/pdfcraft) for musicians, on
Android tablets: read scores full screen, write the usual markings into them, and play through a
concert from one folder. It is the same Rust engine and egui interface as the desktop app, run
in a native Android activity; nothing here is sent upstream.

- **Install it**: [INSTALL.md](INSTALL.md) · [INSTALL.es.md](INSTALL.es.md) (español)
- **What changed**: [CHANGELOG.md](CHANGELOG.md)
- **What comes next**: [ROADMAP.md](ROADMAP.md)
- **Privacy and permissions**: [PRIVACY.md](../../PRIVACY.md)
- **Publish a version**: [RELEASING.md](RELEASING.md)

Everything is on `main`; changes arrive through pull requests.

## What it adds

### Music marks (desktop and tablet)

Stamps drawn as PDF content, so any PDF reader shows them:

| Section | Marks |
|---|---|
| Dynamics | ppp, pp, p, mp, mf, f, ff, fff, sfz, fp |
| Hairpins and slurs | crescendo, diminuendo, slur above, slur below (drag to set their length) |
| Tempo | cresc., dim., rit., accel., a tempo |
| Articulation | accent, staccato, tenuto, marcato, fermata, breath mark, caesura |
| Bowing and pedal | up bow, down bow, Ped., pedal release, "watch" (glasses) |
| Fingering | 1–5 |
| Rehearsal letters | A–H, boxed |
| Numbers | any number, counting up by one if you like (bars, beats, strings) |
| Jumps | "→ A" … "A" (A–H), segno, D.S., coda, To Coda, D.C., Fine |

A tap on a jump mark (with **Move**) goes to the other end: "→ A" to "A" and back, D.S. to the
segno, To Coda to the coda, D.C. to the first page.

On the desktop they are in **Stamps ▸ Music**; headless, `comment_add` with `type: "stamp"` and
the mark's name (`mf`, `crescendo`, `rehearsal f`, `number` with `text`…).

### The tablet interface

- **Scores** (start screen): the PDFs in Download, Documents and the app's own folder, and the
  recent files, as cards with a search field. Folders that hold PDFs show as **concerts**.
- **Music view**: the score alone, fitted to the screen, with no system bars. A tap shows the
  bar (scores, undo, redo, save, marks, fit page, full editor, hide); in a concert, Previous and
  Next go through its scores, and a swipe past the last page goes on to the next one.
- **Moving around**: drag, and a flick glides on; a quick sideways swipe turns the page; two
  fingers zoom about the point between them.
- **Marks panel**: Move, Pencil (handwriting), Highlighter (six colours, three widths), Erase
  (three sizes; rubs out drawings and marks), Select (tap a mark, then Larger, Smaller, Delete,
  or drag it and its corner handles), Undo. Folds down to one row.
- **Full editor**: every PdfCraft tool; "♪ Music view" goes back. In the Pages panel a quick
  drag scrolls, resting on a page first moves it, and Delete / Rotate buttons act on the
  selected pages and save at once.
- **Files**: the app's own browser stands in for the system's open and save panels (it needs
  *All files access*: Settings ▸ Apps ▸ PdfMusicCraft ▸ Permissions ▸ Files).

## Building and installing

Needs: Rust, the Android SDK and NDK (26.1), a JDK 17, `cargo-apk`, and the
[craft-i18n](https://github.com/storytold/craft-i18n) checkout for the Spanish interface.

```powershell
# NDK clang as the C compiler for aarch64-linux-android, ANDROID_HOME, JAVA_HOME…
$env:CRAFT_I18N_DIR = "path\to\craft-i18n"
$env:CARGO_BUILD_JOBS = "2"                # 16 GB of RAM: more jobs run out of memory
$env:CARGO_PROFILE_DEV_OPT_LEVEL = "3"     # full speed (smooth scrolling)
$env:CARGO_PROFILE_DEV_DEBUG = "false"
cargo apk build -p pdfcraft-android --lib
adb install -r target\debug\apk\PdfMusicCraft.apk
```

`build.rs` compiles `java/Immersive.java` (full screen without system bars) with `javac` and the
SDK's `d8`; the app loads it from memory through JNI. `make_icon.py` draws the launcher icon.

## Known gaps

- Printing reaches no printer on Android (the dialog shows, the system print service isn't
  wired up).
- No "Open with" from other apps, and no Storage Access Framework picker.
- Only a 64-bit ARM (`arm64-v8a`) build: `build_targets` lists `aarch64-linux-android` alone.

## Storage Access Framework

A study, not a plan yet: could the app drop *All files access* (`MANAGE_EXTERNAL_STORAGE`) and
use Android's Storage Access Framework (SAF) instead?

**What SAF would give.** No broad permission: the user picks a folder once
(`ACTION_OPEN_DOCUMENT_TREE`) and the app keeps access to that folder only
(`takePersistableUriPermission`). Stores such as F-Droid and Google Play also look harder at apps
that ask for all files access.

**Why it isn't a drop-in change.** Everything here works on file paths:

- The Scores screen walks Download, Documents and the app's folder with `std::fs`, and concerts
  are real folders. Under SAF these become `content://` tree URIs, listed through
  `DocumentsContract` queries over JNI, not directories.
- The engine opens and saves PDFs by path, and saves write a new file and rename it over the old
  one (atomic saves). A SAF document is reached through a file descriptor
  (`ContentResolver.openFileDescriptor`), and "rename over" isn't available the same way.
- The app has no Java activity of its own (it runs in `NativeActivity`); the only Java is
  `Immersive.java`. Starting a picker and receiving its result needs an activity result callback,
  so more Java glue.

**A path that keeps today's features working.**

1. Add "Open with" first (an `ACTION_VIEW` intent filter for `application/pdf`): it needs no
   permission, and it is the same `content://` plumbing (read through a descriptor, save back
   through one) on a smaller scale.
2. Then offer "Add a folder" through `ACTION_OPEN_DOCUMENT_TREE`, listing that tree beside the
   path-based folders, with saves through the descriptor.
3. Only when SAF folders do everything the path-based ones do (concerts, hide, save in place)
   make *All files access* optional, and keep it for those who prefer it.

Until then the permission stays: removing it now would empty the Scores screen and break saving
marks into existing scores.
