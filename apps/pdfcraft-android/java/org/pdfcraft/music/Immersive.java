package org.pdfcraft.music;

import android.Manifest;
import android.app.Activity;
import android.app.AlertDialog;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.net.Uri;
import android.os.Build;
import android.os.Environment;
import android.provider.Settings;
import java.util.Locale;
import android.view.View;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;

/**
 * Full screen without the system bars (status bar, navigation bar, a tablet's taskbar): a swipe
 * from the edge shows them for a moment. Runs on the UI thread (posted with runOnUiThread from
 * Rust) and hides the bars again whenever the system brings them back.
 *
 * On its first run it also asks for access to the scores: without "All files access" (Android 11
 * and later) or the storage permission (earlier), the app lists no PDFs and opens no folders.
 */
public final class Immersive implements Runnable, View.OnSystemUiVisibilityChangeListener {
    private static final int FLAGS = View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
            | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
            | View.SYSTEM_UI_FLAG_FULLSCREEN
            | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
            | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
            | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN;

    private final Activity activity;
    /** Storage access is asked about once per launch, not each time the bars are hidden again. */
    private boolean askedStorage;

    public Immersive(Activity activity) {
        this.activity = activity;
    }

    @Override
    public void run() {
        Window window = activity.getWindow();
        if (window == null) {
            return;
        }
        View decor = window.getDecorView();
        decor.setSystemUiVisibility(FLAGS);
        decor.setOnSystemUiVisibilityChangeListener(this);
        if (Build.VERSION.SDK_INT >= 30) {
            WindowInsetsController c = window.getInsetsController();
            if (c != null) {
                c.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                c.hide(WindowInsets.Type.systemBars());
            }
        }
        if (!askedStorage) {
            askedStorage = true;
            askForStorage();
        }
    }

    /** Ask for access to shared storage when the app doesn't have it yet. */
    private void askForStorage() {
        if (Build.VERSION.SDK_INT >= 30) {
            if (Environment.isExternalStorageManager()) {
                return;
            }
            boolean es = "es".equals(Locale.getDefault().getLanguage());
            new AlertDialog.Builder(activity)
                    .setTitle(es ? "Acceso a tus partituras" : "Access to your scores")
                    .setMessage(es
                            ? "Para ver y abrir los PDF de Descargas, Documentos y otras carpetas, PdfMusicCraft necesita el permiso \"Acceso a todos los archivos\". En la pantalla siguiente, activa \"Permitir la gestión de todos los archivos\" y vuelve a la app."
                            : "To list and open the PDFs in Download, Documents and other folders, PdfMusicCraft needs \"All files access\". On the next screen, turn on \"Allow management of all files\" and come back to the app.")
                    .setPositiveButton(es ? "Dar permiso" : "Allow", (d, w) -> openAllFilesSettings())
                    .setNegativeButton(es ? "Ahora no" : "Not now", null)
                    .show();
        } else if (activity.checkSelfPermission(Manifest.permission.READ_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED) {
            activity.requestPermissions(new String[] {Manifest.permission.READ_EXTERNAL_STORAGE, Manifest.permission.WRITE_EXTERNAL_STORAGE}, 1);
        }
    }

    /** The system's "All files access" switch for this app (or the list of apps, if that fails). */
    private void openAllFilesSettings() {
        try {
            Intent intent = new Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION, Uri.parse("package:" + activity.getPackageName()));
            activity.startActivity(intent);
        } catch (RuntimeException e) {
            try {
                activity.startActivity(new Intent(Settings.ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION));
            } catch (RuntimeException ignored) {
                // No settings screen to send the player to: the app's notice says where it is.
            }
        }
    }

    @Override
    public void onSystemUiVisibilityChange(int visibility) {
        if ((visibility & View.SYSTEM_UI_FLAG_HIDE_NAVIGATION) == 0) {
            activity.getWindow().getDecorView().postDelayed(this, 1500);
        }
    }
}
