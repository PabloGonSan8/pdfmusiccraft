# Privacy

**English** · [Español](#español)

This page describes what the PdfMusicCraft Android app (`io.github.pablogonsan8.pdfmusiccraft`)
does with your data. It is checked against the permissions in
[`apps/pdfcraft-android/Cargo.toml`](apps/pdfcraft-android/Cargo.toml), which become the APK's
manifest. If you find something here that doesn't match the app, please
[open an issue](https://github.com/PabloGonSan8/pdfmusiccraft/issues).

## In short

- No account, no ads, no analytics and no tracking code.
- The app **does not request Android's `INTERNET` permission**, so Android does not let it open
  network connections: your scores and marks are not sent anywhere by the app.
- Everything stays on your device: your PDFs where you keep them, and the app's settings in its
  private storage.

## Permissions

| Permission | Android versions | Why |
|---|---|---|
| `MANAGE_EXTERNAL_STORAGE` ("All files access") | 11 and later | To list the PDFs you already have in Download, Documents and other folders, open them, and save your marks back into them, without importing copies. You grant it yourself in Settings, and you can take it back there at any time. |
| `READ_EXTERNAL_STORAGE` | up to 12 (API 32) | The same, on older Android versions. |
| `WRITE_EXTERNAL_STORAGE` | up to 10 (API 29) | Saving marks into your PDFs on older Android versions. |

No other permission is requested: no internet, camera, microphone, location, contacts or
notifications.

## What is stored, and where

- **Your PDFs** stay where they are. When you tap **Save**, your marks are written into that PDF
  file. Nothing is saved into a PDF until you do.
- **The app's settings** (preferences, the recent-files list, the scores you hid from the list)
  are kept in one file in the app's private storage. Android deletes it when you uninstall the
  app.
- **The app's own folder** (`Android/data/io.github.pablogonsan8.pdfmusiccraft/files`) is one of
  the places the Scores screen looks for PDFs. Android deletes it when you uninstall the app.
- **Diagnostic messages** go to Android's system log on the device (logcat), as for any app. They
  are not sent anywhere by PdfMusicCraft.

## Downloads and updates

You download the APK from GitHub (directly or through the [website](https://pdfmusiccraft.netlify.app/),
whose download button points to GitHub Releases). Those visits are covered by
[GitHub's](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement)
and [Netlify's](https://www.netlify.com/privacy/) privacy policies, not by the app. If you use
Obtainium, it contacts GitHub on its own to check for new versions.

---

## Español

Esta página explica qué hace la app de Android PdfMusicCraft
(`io.github.pablogonsan8.pdfmusiccraft`) con tus datos. Está comprobada con los permisos de
[`apps/pdfcraft-android/Cargo.toml`](apps/pdfcraft-android/Cargo.toml), que forman el manifiesto
del APK. Si algo de aquí no coincide con la app,
[abre una incidencia](https://github.com/PabloGonSan8/pdfmusiccraft/issues).

### En resumen

- Sin cuenta, sin anuncios, sin analíticas y sin código de seguimiento.
- La app **no pide el permiso `INTERNET` de Android**, así que Android no le deja abrir
  conexiones de red: la app no envía tus partituras ni tus marcas a ningún sitio.
- Todo se queda en tu dispositivo: tus PDF donde los tengas, y los ajustes de la app en su
  almacenamiento privado.

### Permisos

| Permiso | Versiones de Android | Para qué |
|---|---|---|
| `MANAGE_EXTERNAL_STORAGE` ("Acceso a todos los archivos") | 11 y posteriores | Para listar los PDF que ya tienes en Descargas, Documentos y otras carpetas, abrirlos y guardar en ellos tus marcas, sin importar copias. Lo concedes tú en Ajustes y puedes quitarlo cuando quieras. |
| `READ_EXTERNAL_STORAGE` | hasta 12 (API 32) | Lo mismo, en versiones antiguas de Android. |
| `WRITE_EXTERNAL_STORAGE` | hasta 10 (API 29) | Guardar las marcas en tus PDF en versiones antiguas de Android. |

No pide ningún otro permiso: ni internet, ni cámara, micrófono, ubicación, contactos o
notificaciones.

### Qué se guarda y dónde

- **Tus PDF** se quedan donde están. Al tocar **Guardar**, las marcas se escriben en ese PDF.
  Hasta entonces no se modifica.
- **Los ajustes de la app** (preferencias, la lista de recientes, las partituras que ocultaste)
  se guardan en un archivo del almacenamiento privado de la app. Android lo borra al
  desinstalarla.
- **La carpeta propia de la app** (`Android/data/io.github.pablogonsan8.pdfmusiccraft/files`) es
  uno de los sitios donde la pantalla Partituras busca PDF. Android la borra al desinstalar la
  app.
- **Los mensajes de diagnóstico** van al registro del sistema de Android en el dispositivo
  (logcat), como en cualquier app. PdfMusicCraft no los envía a ningún sitio.

### Descargas y actualizaciones

El APK se descarga de GitHub (directamente o desde la [web](https://pdfmusiccraft.netlify.app/),
cuyo botón apunta a GitHub Releases). Esas visitas se rigen por las políticas de privacidad de
[GitHub](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement) y
[Netlify](https://www.netlify.com/privacy/), no por la app. Si usas Obtainium, es Obtainium quien
consulta GitHub para buscar versiones nuevas.
