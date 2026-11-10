package org.pdfcraft.music;

import android.app.Activity;
import android.os.Build;
import android.view.View;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;

/**
 * Full screen without the system bars (status bar, navigation bar, a tablet's taskbar): a swipe
 * from the edge shows them for a moment. Runs on the UI thread (posted with runOnUiThread from
 * Rust) and hides the bars again whenever the system brings them back.
 */
public final class Immersive implements Runnable, View.OnSystemUiVisibilityChangeListener {
    private static final int FLAGS = View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
            | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
            | View.SYSTEM_UI_FLAG_FULLSCREEN
            | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
            | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
            | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN;

    private final Activity activity;

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
    }

    @Override
    public void onSystemUiVisibilityChange(int visibility) {
        if ((visibility & View.SYSTEM_UI_FLAG_HIDE_NAVIGATION) == 0) {
            activity.getWindow().getDecorView().postDelayed(this, 1500);
        }
    }
}
