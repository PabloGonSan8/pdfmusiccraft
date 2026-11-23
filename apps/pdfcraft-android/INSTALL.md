# Installing PdfMusicCraft

**English** · [Español](INSTALL.es.md)

PdfMusicCraft is an app for reading and marking up PDF scores on Android tablets and phones.
It is not on Google Play: you install it from the `.apk` file published on the project's
**Releases** page:

**https://github.com/PabloGonSan8/pdfmusiccraft/releases**

## What you need

- Android 8.0 (API 26) or later.
- A 64-bit ARM device (`arm64-v8a`), which is almost every current phone and tablet. There is no
  build for x86 or 32-bit ARM devices.
- About 200 MB of free space (the download itself is about 32 MB).

| The download | |
|---|---|
| File | `PdfMusicCraft-<version>.apk`, one per release, under **Assets** |
| Version | the one in the file name; the newest is always at [releases/latest](https://github.com/PabloGonSan8/pdfmusiccraft/releases/latest) |
| Signed | every release with the same key, so a new one installs over the old one |
| Checksum | the SHA-256 shown next to the file on the release page ([how to check it](#check-the-download-sha-256)) |
| Permissions | file access only, no internet: see [Privacy](#permissions-and-privacy) |

## Install from the phone or tablet

1. Open the [Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases) page in the device's
   browser.
2. Under the latest version (at the top), expand **Assets** and tap the
   `PdfMusicCraft-….apk` file to download it.
3. Open the downloaded file (from the notification, or from the **Files** app ▸ Downloads).
4. The first time, Android asks to let the browser install apps. Tap **Settings**, turn on
   **Allow from this source**, and go back.
5. Tap **Install**. If Play Protect warns that the app isn't verified, tap **More details** ▸
   **Install anyway**. It warns about any app that doesn't come from Google Play.
6. Open **PdfMusicCraft**.

### First start: file access

To list your scores in Download, Documents and other folders, the app needs the **All files
access** permission:

**Settings** ▸ **Apps** ▸ **PdfMusicCraft** ▸ **Permissions** ▸ **Files** ▸ **Allow management
of all files**.

Copy your PDFs into **Download** or **Documents** and they appear on the **Scores** screen. Each
folder holding several PDFs shows as a **concert**.

## Check the download (SHA-256)

Optional, but it proves the file you got is the one that was published. On the release page,
GitHub shows a `sha256:` value next to each file. Compute the same value for your download and
compare:

| System | Command |
|---|---|
| Windows (PowerShell) | `Get-FileHash .\PdfMusicCraft-1.1.1.apk -Algorithm SHA256` |
| macOS | `shasum -a 256 PdfMusicCraft-1.1.1.apk` |
| Linux | `sha256sum PdfMusicCraft-1.1.1.apk` |

On the device itself, file manager apps such as Material Files show a file's SHA-256 in its
properties. If the values differ, don't install it: download it again from the release page.

Android adds its own check on updates: an APK signed with a different key won't install over the
one you have.

## Permissions and privacy

The app asks for **file access** only, to list your PDFs and save your marks into them. It does
not ask for internet access, so it sends nothing anywhere; there is no account, no ads and no
analytics. The details: [PRIVACY.md](../../PRIVACY.md).

## Install from a computer

### Option A: copy the file

1. Download the `.apk` from [Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases) on the
   computer.
2. Connect the device by USB and choose **File transfer** in the device's notification.
3. Copy the `.apk` into the device's **Download** folder.
4. On the device, open **Files** ▸ Downloads, tap the `.apk`, and follow steps 4 to 6 above.

### Option B: with adb (if you already use it)

With [USB debugging](https://developer.android.com/studio/debug/dev-options) turned on:

```
adb install -r PdfMusicCraft-….apk
```

`-r` installs over an earlier version without losing anything.

## Updating

Each new version is published on [Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases).
To update, download the new `.apk` and install it **over** the one you have, as the first time,
**without uninstalling**. Your scores, marks and settings stay.

> Marks are saved inside the PDFs themselves when you tap **Save**: even if you uninstalled the
> app, your marked-up scores would stay as they are.

### Automatic updates with Obtainium (recommended)

[Obtainium](https://github.com/ImranR98/Obtainium) is a free, open-source app that watches
GitHub projects and tells you about (and installs) each new version.

1. Install Obtainium from its [Releases](https://github.com/ImranR98/Obtainium/releases) page
   or from [F-Droid](https://f-droid.org/packages/dev.imranr.obtainium.fdroid/).
2. Tap **Add app** and paste this address:
   `https://github.com/PabloGonSan8/pdfmusiccraft`
3. Tap **Add**. From then on Obtainium tells you about each new version.

Obtainium installs the APK straight from GitHub Releases, so it is the same file as a manual
download, signed with the same key: you can switch between the two at any time.

### Other stores

PdfMusicCraft is **not on Google Play**, by choice, and **not on F-Droid** today. What would be
needed, for anyone who wants to help:

- **F-Droid** builds every app from source on its own servers. The recipe would need Rust,
  the Android NDK, `cargo-apk` and the [craft-i18n](https://github.com/storytold/craft-i18n)
  checkout, and the build is large (it compiles the whole PdfCraft engine). All dependencies are
  open source, which F-Droid requires; *All files access* is allowed there but has to be
  justified in the listing.
- **IzzyOnDroid** (an F-Droid-compatible repository) takes the APKs from GitHub Releases as they
  are, so it needs no new build; it has its own inclusion rules, including APK size limits, to
  check first.

## Reporting a bug or asking for something

Open an issue in the project's **[Issues](https://github.com/PabloGonSan8/pdfmusiccraft/issues)** tab
(you need a free GitHub account). So it can be fixed quickly, say:

- which version you have (the number is in the name of the `.apk` you installed);
- your device and Android version;
- what you did, what you expected, and what happened;
- a screenshot if you can.

Without a GitHub account, tell whoever gave you the app.

## Uninstalling

**Settings** ▸ **Apps** ▸ **PdfMusicCraft** ▸ **Uninstall**. The app's settings go; your PDFs,
and the marks saved in them, stay.

## Licence

PdfMusicCraft is an adaptation of [PdfCraft](https://github.com/storytold/pdfcraft), free
software under the MIT or Apache-2.0 licence. The licence notices are in `LICENSE-MIT`,
`LICENSE-APACHE`, `NOTICE` and `ATTRIBUTION.md`, at the root of the repository.
