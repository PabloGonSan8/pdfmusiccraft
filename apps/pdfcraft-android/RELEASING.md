# Releasing PdfMusicCraft

How a new version reaches people: a signed APK attached to a GitHub release, which
[INSTALL.md](INSTALL.md) points them to.

## Once: the signing key

Every release must be signed with the same key, or it won't install over the one before.
Keep the keystore and its password **outside the repository**, backed up somewhere safe: a lost
key means everyone has to uninstall to get the next version.

```powershell
keytool -genkeypair -v -keystore $HOME\pdfmusiccraft-release.jks -alias pdfmusiccraft `
  -keyalg RSA -keysize 4096 -validity 10000
```

`keytool` asks for a password and your name; it comes with the JDK
(`C:\Program Files\Eclipse Adoptium\jdk-17…\bin`).

## Each release

1. **Version.** Raise `version` in `apps/pdfcraft-android/Cargo.toml` (semver: `1.0.1` for fixes,
   `1.1.0` for new features, `2.0.0` for changes that break something). cargo-apk derives the
   Android version from it; it must go up for the APK to install over the last.
2. **Changelog.** Move what changed into a new section of [CHANGELOG.md](CHANGELOG.md), with the
   date.
3. **Build** with the Android environment of [README.md](README.md#building-and-installing), the
   release profile and the key (the password only in the environment, never in a file):

   ```powershell
   $env:CARGO_APK_RELEASE_KEYSTORE = "$HOME\pdfmusiccraft-release.jks"
   $env:CARGO_APK_RELEASE_KEYSTORE_PASSWORD = Read-Host "Keystore password"
   cargo apk build -p pdfcraft-android --lib --release
   Copy-Item target-android\release\apk\PdfMusicCraft.apk PdfMusicCraft-1.0.0.apk
   ```

4. **Try it** on a device: `adb install -r PdfMusicCraft-1.0.0.apk`.
5. **Commit and tag**: commit the version and changelog, then `git tag v1.0.0` and
   `git push origin <branch> v1.0.0`.
6. **Publish** on GitHub: the repository's **Releases** ▸ **Draft a new release** ▸ choose the tag
   `v1.0.0`, title `PdfMusicCraft 1.0.0`, paste that version's changelog section as the notes,
   attach `PdfMusicCraft-1.0.0.apk`, and **Publish release**.

Obtainium users get it on their own; others download it as INSTALL.md says.
