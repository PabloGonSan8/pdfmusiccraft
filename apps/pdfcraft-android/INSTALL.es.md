# Instalar PdfMusicCraft

[English](INSTALL.md) · **Español**

PdfMusicCraft es una app para leer y anotar partituras en PDF en tabletas y móviles Android.
No está en Google Play: se instala con el archivo `.apk` que se publica en la página de
**Releases** del proyecto:

**https://github.com/PabloGonSan8/pdfmusiccraft/releases**

## Qué necesitas

- Android 8.0 (API 26) o posterior.
- Un dispositivo ARM de 64 bits (`arm64-v8a`), que son casi todos los móviles y tabletas
  actuales. No hay versión para x86 ni para ARM de 32 bits.
- Unos 200 MB libres (la descarga ocupa unos 32 MB).

| La descarga | |
|---|---|
| Archivo | `PdfMusicCraft-<versión>.apk`, uno por versión, en **Assets** |
| Versión | la del nombre del archivo; la más reciente está siempre en [releases/latest](https://github.com/PabloGonSan8/pdfmusiccraft/releases/latest) |
| Firma | todas las versiones con la misma clave, para que la nueva se instale encima de la anterior |
| Suma de comprobación | el SHA-256 que aparece junto al archivo en la página de la versión ([cómo comprobarlo](#comprobar-la-descarga-sha-256)) |
| Permisos | solo acceso a archivos, sin internet: mira [Privacidad](#permisos-y-privacidad) |

## Instalar desde el móvil o la tableta

1. Abre la página de [Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases) en el
   navegador del dispositivo.
2. En la versión más reciente (arriba del todo), despliega **Assets** y toca el archivo
   `PdfMusicCraft-….apk` para descargarlo.
3. Abre el archivo descargado (desde la notificación o desde la app **Archivos** ▸ Descargas).
4. La primera vez, Android te pide permiso para instalar apps desde el navegador. Toca
   **Ajustes**, activa **Permitir de esta fuente** y vuelve atrás.
5. Toca **Instalar**. Si Play Protect avisa de que la app no está verificada, toca **Más
   detalles** ▸ **Instalar de todas formas**. Ese aviso sale con cualquier app que no venga de
   Google Play.
6. Abre **PdfMusicCraft**.

### Primer arranque: permiso de archivos

Para ver tus partituras en Descargas, Documentos y otras carpetas, la app necesita el permiso
**Acceso a todos los archivos**:

**Ajustes** ▸ **Aplicaciones** ▸ **PdfMusicCraft** ▸ **Permisos** ▸ **Archivos** ▸ **Permitir
la gestión de todos los archivos**.

Copia tus PDF a **Descargas** o a **Documentos** y aparecerán en la pantalla **Partituras**.
Cada carpeta con varios PDF aparece como un **concierto**.

## Comprobar la descarga (SHA-256)

Es opcional, pero demuestra que el archivo que tienes es el publicado. En la página de la
versión, GitHub muestra un valor `sha256:` junto a cada archivo. Calcula el mismo valor de tu
descarga y compáralos:

| Sistema | Orden |
|---|---|
| Windows (PowerShell) | `Get-FileHash .\PdfMusicCraft-1.1.0.apk -Algorithm SHA256` |
| macOS | `shasum -a 256 PdfMusicCraft-1.1.0.apk` |
| Linux | `sha256sum PdfMusicCraft-1.1.0.apk` |

En el propio dispositivo, gestores de archivos como Material Files muestran el SHA-256 en las
propiedades del archivo. Si los valores no coinciden, no lo instales: descárgalo otra vez desde
la página de la versión.

Android añade su propia comprobación al actualizar: un APK firmado con otra clave no se instala
encima del que tienes.

## Permisos y privacidad

La app solo pide **acceso a los archivos**, para listar tus PDF y guardar en ellos tus marcas. No
pide acceso a internet, así que no envía nada a ningún sitio; no hay cuenta, ni anuncios, ni
analíticas. Los detalles: [PRIVACY.md](../../PRIVACY.md#español).

## Instalar desde el ordenador

### Opción A: copiar el archivo

1. Descarga el `.apk` desde [Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases) en el
   ordenador.
2. Conecta el dispositivo por USB y elige **Transferencia de archivos** en la notificación del
   dispositivo.
3. Copia el `.apk` a la carpeta **Download** del dispositivo.
4. En el dispositivo, abre **Archivos** ▸ Descargas, toca el `.apk` y sigue los pasos 4 a 6 de
   arriba.

### Opción B: con adb (para quien ya lo use)

Con la [depuración USB](https://developer.android.com/studio/debug/dev-options) activada:

```
adb install -r PdfMusicCraft-….apk
```

`-r` instala encima de una versión anterior sin perder nada.

## Actualizar

Cada versión nueva se publica en [Releases](https://github.com/PabloGonSan8/pdfmusiccraft/releases).
Para actualizar, descarga el `.apk` nuevo e instálalo **encima** del que tienes, igual que la
primera vez, **sin desinstalar**. Tus partituras, marcas y ajustes se conservan.

> Las marcas se guardan dentro de los propios PDF cuando tocas **Guardar**: aunque
> desinstalaras la app, tus partituras anotadas seguirían intactas.

### Actualizaciones automáticas con Obtainium (recomendado)

[Obtainium](https://github.com/ImranR98/Obtainium) es una app gratuita y libre que vigila
proyectos de GitHub y te avisa (e instala) cuando sale una versión nueva.

1. Instala Obtainium desde su página de
   [Releases](https://github.com/ImranR98/Obtainium/releases) o desde
   [F-Droid](https://f-droid.org/packages/dev.imranr.obtainium.fdroid/).
2. Toca **Añadir app** y pega esta dirección:
   `https://github.com/PabloGonSan8/pdfmusiccraft`
3. Toca **Añadir**. A partir de ahí Obtainium te avisará de cada versión nueva.

Obtainium instala el APK directamente desde GitHub Releases: es el mismo archivo que la descarga
manual, firmado con la misma clave, así que puedes pasar de un método a otro cuando quieras.

### Otras tiendas

PdfMusicCraft **no está en Google Play**, por decisión propia, y **hoy no está en F-Droid**. Lo
que haría falta, por si alguien quiere ayudar:

- **F-Droid** compila cada app desde el código en sus servidores. La receta necesitaría Rust, el
  NDK de Android, `cargo-apk` y [craft-i18n](https://github.com/storytold/craft-i18n), y la
  compilación es grande (todo el motor de PdfCraft). Todas las dependencias son libres, como
  exige F-Droid; el *Acceso a todos los archivos* se permite, pero hay que justificarlo en la
  ficha.
- **IzzyOnDroid** (un repositorio compatible con F-Droid) toma los APK de GitHub Releases tal
  cual, sin compilar de nuevo; tiene sus propias reglas de inclusión, entre ellas límites de
  tamaño del APK, que habría que revisar antes.

## Informar de un fallo o pedir algo

Abre una incidencia en la pestaña
**[Issues](https://github.com/PabloGonSan8/pdfmusiccraft/issues)** del proyecto (necesitas una cuenta
gratuita de GitHub). Para que se pueda arreglar rápido, cuenta:

- qué versión tienes (el número aparece en el nombre del `.apk` que instalaste);
- tu dispositivo y versión de Android;
- qué hiciste, qué esperabas y qué pasó;
- una captura de pantalla si puedes.

Si no tienes cuenta de GitHub, cuéntaselo directamente a quien te pasó la app.

## Desinstalar

**Ajustes** ▸ **Aplicaciones** ▸ **PdfMusicCraft** ▸ **Desinstalar**. Se borran los ajustes de
la app; tus PDF y las marcas guardadas en ellos se quedan.

## Licencia

PdfMusicCraft es una adaptación de [PdfCraft](https://github.com/storytold/pdfcraft), software
libre bajo licencia MIT o Apache-2.0. Los avisos de licencia están en `LICENSE-MIT`,
`LICENSE-APACHE`, `NOTICE` y `ATTRIBUTION.md`, en la raíz del repositorio.
