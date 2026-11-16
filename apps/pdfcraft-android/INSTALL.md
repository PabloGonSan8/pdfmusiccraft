# Installing PdfMusicCraft

**English** · [Español](INSTALL.es.md)

PdfMusicCraft is an app for reading and marking up PDF scores on Android tablets and phones.
It is not on Google Play: you install it from the `.apk` file published on the project's
**Releases** page:

**https://github.com/PabloGonSan8/pdfmusiccraft/releases**

## What you need

- Android 8.0 or later.
- A 64-bit ARM device, which is almost every current phone and tablet.
- About 200 MB of free space.

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
