//! PdfCraft on Android: the same app and engine as the desktop, in a native activity.
//!
//! The start screen lists the PDFs in the app's own folder (`Android/data/io.github.pablogonsan8.pdfmusiccraft/files/`),
//! Download and Documents, and the recent files. Open and Save use the app's own file browser (`pdfcraft_ui_egui::touch`), which
//! needs "All files access" (Settings ▸ Apps ▸ PdfCraft ▸ Permissions ▸ Files) to see Download
//! and Documents. ponytail: no Storage Access Framework (that needs JNI and a Java activity);
//! "Open with" from other apps isn't handled yet.

#![cfg_attr(not(target_os = "android"), allow(unused))]

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    // Android sets no LANG: the interface follows the device's language ("es" + "ES").
    let config = app.config();
    if let Some(lang) = config.language() {
        let tag = config.country().map_or_else(|| lang.clone(), |c| format!("{lang}-{c}"));
        pdfcraft_ui_egui::i18n::set_platform_language(&tag);
    }
    // Full screen: no status bar, navigation bar or taskbar.
    if let Err(e) = immersive(&app) {
        log::warn!("could not hide the system bars: {e}");
    }
    let files = app.external_data_path();
    let options = eframe::NativeOptions {
        // Settings live in the app's private storage.
        persistence_path: app.internal_data_path().map(|d| d.join("pdfcraft.ron")),
        android_app: Some(app),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let result = eframe::run_native(
        "PdfMusicCraft",
        options,
        Box::new(move |cc| {
            let mut app = pdfcraft_ui_egui::PdfCraftApp::new();
            if let Some(json) = cc.storage.and_then(|s| s.get_string("pdfcraft")) {
                app.restore(&json);
            }
            // A tablet: the start screen lists the scores; they open in the music view, moved by
            // the finger.
            app.touch.enabled = true;
            app.touch.focus = true;
            app.quick_tool = pdfcraft_ui_egui::QuickTool::Hand;
            let storage = std::path::Path::new("/storage/emulated/0");
            app.touch.library = files.into_iter().chain([storage.join("Download"), storage.join("Documents")]).collect();
            Ok(Box::new(app))
        }),
    );
    if let Err(e) = result {
        log::error!("PdfCraft could not start: {e}");
    }
}

/// Hide the system bars for good: load `Immersive` (compiled from `java/` by `build.rs`) from
/// memory and run it on the UI thread; it hides them again whenever the system shows them.
#[cfg(target_os = "android")]
fn immersive(app: &winit::platform::android::activity::AndroidApp) -> Result<(), jni::errors::Error> {
    use jni::objects::{JClass, JObject};
    use jni::{jni_sig, jni_str};
    static DEX: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
    if DEX.is_empty() {
        return Ok(());
    }
    let activity_ptr = app.activity_as_ptr();
    jni::JavaVM::singleton()?.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
        // SAFETY: android-activity's global reference to the running Activity, valid while the app
        // runs; it isn't deleted here (a `JObject` doesn't free its reference when dropped).
        let activity = unsafe { JObject::from_raw(env, activity_ptr.cast()) };
        let bytes = env.byte_array_from_slice(DEX)?;
        let buffer = env
            .call_static_method(jni_str!("java/nio/ByteBuffer"), jni_str!("wrap"), jni_sig!("([B)Ljava/nio/ByteBuffer;"), &[(&bytes).into()])?
            .l()?;
        let parent = env.call_method(&activity, jni_str!("getClassLoader"), jni_sig!("()Ljava/lang/ClassLoader;"), &[])?.l()?;
        let loader = env.new_object(
            jni_str!("dalvik/system/InMemoryDexClassLoader"),
            jni_sig!("(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V"),
            &[(&buffer).into(), (&parent).into()],
        )?;
        let name = env.new_string("org.pdfcraft.music.Immersive")?;
        let class = env.call_method(&loader, jni_str!("loadClass"), jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"), &[(&name).into()])?.l()?;
        let class: JClass = env.cast_local::<JClass>(class)?;
        let runnable = env.new_object(&class, jni_sig!("(Landroid/app/Activity;)V"), &[(&activity).into()])?;
        env.call_method(&activity, jni_str!("runOnUiThread"), jni_sig!("(Ljava/lang/Runnable;)V"), &[(&runnable).into()])?;
        Ok(())
    })
}
