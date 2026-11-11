//! The touch interface (tablets and phones; the Android app turns it on).
//!
//! - Start screen: the PDFs in the library folders and the recent files, one tap to open.
//! - Music view: the open document fitted to the screen with nothing around it. A tap on the page
//!   shows or hides a bar (start screen, open, save, undo, the full editor, music); "Music" opens a
//!   sheet of finger-sized marks, a pen and an eraser, and each tap on the page places the chosen
//!   mark. The sheet folds down to one row and the bar hides, so the score keeps the screen.
//! - A tap on a jump mark (segno, coda, D.S., D.C.) goes to the other one (`music_ui`).
//! - In the full editor a floating "Music view" button goes back.
//! - On Android, the app's own file browser stands in for the native open/save panels.

use std::path::PathBuf;

use egui::{Align2, Color32, RichText, vec2};

use crate::music_ui::es;
use crate::{PdfCraftApp, QuickTool};

/// The touch interface is on: page thumbnails scroll under a quick drag (see [`drag_scrolls`]).
static TOUCH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// How long (seconds) a finger rests on a page thumbnail before a drag moves the page rather
/// than scrolling the list.
const LONG_PRESS: f64 = 0.35;

/// Whether the touch interface is on.
pub(crate) fn on() -> bool {
    TOUCH.load(std::sync::atomic::Ordering::Relaxed)
}

/// Note whether the touch interface is on, for widgets that don't see the app.
fn note_touch(app: &PdfCraftApp, ctx: &egui::Context) {
    TOUCH.store(app.touch.enabled, std::sync::atomic::Ordering::Relaxed);
    crate::comments::set_finger_handles(app.touch.enabled);
    if app.touch.enabled && ctx.options(|o| o.input_options.max_click_dist) < FINGER_CLICK_DIST {
        // A fingertip wobbles: a tap that moved a little is still a tap (it deselects, places a
        // mark), not the start of a drag.
        ctx.options_mut(|o| o.input_options.max_click_dist = FINGER_CLICK_DIST);
    }
}

/// How far (points) a finger may move during a tap.
/// (A drag starts only past it, and then catches up all at once: more makes each drag begin
/// with a visible jump.)
const FINGER_CLICK_DIST: f32 = 10.0;

/// How much room (points, as f32 bits) the music sheet covers at the bottom of the screen: the
/// pages scroll that much further, so the end of the last page can come up above it.
static BOTTOM_ROOM: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The room the pages leave below the last one for the music sheet (0 without it).
pub(crate) fn bottom_room() -> f32 {
    f32::from_bits(BOTTOM_ROOM.load(std::sync::atomic::Ordering::Relaxed))
}

fn set_bottom_room(points: f32) {
    BOTTOM_ROOM.store(points.max(0.0).to_bits(), std::sync::atomic::Ordering::Relaxed);
}

/// Two fingers always move and zoom the page, whatever tool is chosen: while they're down the
/// tool is put aside for the Hand, and comes back once every finger has lifted.
fn two_fingers_pan(app: &mut PdfCraftApp, ctx: &egui::Context) {
    let (fingers, down) = ctx.input(|i| (i.multi_touch().is_some(), i.pointer.any_down()));
    if fingers && app.quick_tool != QuickTool::Hand && app.touch.before_fingers.is_none() {
        app.touch.before_fingers = Some(app.quick_tool);
        app.quick_tool = QuickTool::Hand;
        // Whatever the first finger began (a selection, a stroke) ends here.
        if let Some(view) = app.active.and_then(|i| app.views.get_mut(i)) {
            view.comments.gesture = None;
        }
    } else if !fingers && !down && let Some(tool) = app.touch.before_fingers.take() {
        app.quick_tool = tool;
    }
}

/// With the touch interface, whether the drag on page thumbnail `resp` scrolls the list (a quick
/// drag) rather than moving pages (a drag after resting the finger on the page for a moment); a
/// scrolling drag scrolls here. Always false without the touch interface.
pub(crate) fn drag_scrolls(ui: &egui::Ui, resp: &egui::Response) -> bool {
    if !TOUCH.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    let id = resp.id.with("touch-drag-scrolls");
    if resp.drag_started() {
        let held = ui.input(|i| i.pointer.press_start_time().map_or(0.0, |t| i.time - t));
        ui.data_mut(|d| d.insert_temp(id, held < LONG_PRESS));
    }
    let scrolling = ui.data(|d| d.get_temp::<bool>(id)).unwrap_or(false);
    if scrolling && resp.dragged() {
        ui.scroll_with_delta(resp.drag_delta());
    }
    if resp.drag_stopped() {
        ui.data_mut(|d| d.remove::<bool>(id));
    }
    scrolling
}

/// The most PDFs the start screen lists.
const MAX_SHELF: usize = 300;

/// The touch interface's state.
#[derive(Default)]
pub struct Touch {
    /// The touch interface is on (set by the app at start).
    pub enabled: bool,
    /// Show an open document in the music view rather than the full editor.
    pub focus: bool,
    /// Show the start screen even with documents open.
    pub home: bool,
    /// The folders whose PDFs the start screen lists (the Android app sets them).
    pub library: Vec<PathBuf>,
    /// The PDFs the start screen lists, read again each time it opens.
    shelf: Option<Vec<PathBuf>>,
    /// The start screen's search text.
    search: String,
    /// The folders of scores the start screen offers as concerts, with how many PDFs each has.
    concerts: Vec<(PathBuf, usize)>,
    /// The concert open now: its scores in order, gone through with Previous and Next.
    pub concert: Vec<PathBuf>,
    /// The music view's bar is showing.
    bar: bool,
    /// The music sheet is showing.
    music: bool,
    /// The music sheet is folded down to one row.
    folded: bool,
    /// The tool chosen in the music sheet (a mark, the pen, the eraser): a tool may put itself
    /// down after use, and the music sheet picks it up again, so a player can go on.
    tool: Option<QuickTool>,
    /// Where a one-finger press on the score began, for a sideways swipe that turns the page.
    swipe_from: Option<egui::Pos2>,
    /// The document last fitted to the screen.
    fitted: Option<pdfcraft_engine::DocId>,
    /// The document being fitted (its pages laid out at the page's width first).
    fitting: Option<pdfcraft_engine::DocId>,
    /// The zoom that fits the music to the screen's width: at it or below, a sideways swipe turns
    /// the page (zoomed in further, it pans).
    fit_zoom: Option<f32>,
    /// The folder the file browser showed last.
    pub last_dir: Option<PathBuf>,
    /// The tool put aside while two fingers move the page ([`two_fingers_pan`]).
    before_fingers: Option<QuickTool>,
    /// The scores taken off the start screen (they stay on disk); saved with the settings.
    pub hidden: Vec<PathBuf>,
    /// The start screen is choosing scores (to take them off it): the ones chosen.
    choosing: Option<std::collections::BTreeSet<PathBuf>>,
    /// Where the editor's "Music view" button was dragged to, from its usual place.
    music_button: egui::Vec2,
    /// The file browser, while it's open (Android).
    #[cfg(target_os = "android")]
    pub(crate) browser: Option<Browser>,
}

impl std::fmt::Debug for Touch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Touch").field("enabled", &self.enabled).field("focus", &self.focus).finish_non_exhaustive()
    }
}

/// Finger-sized buttons (the Android file browser's).
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
fn big(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(egui::Button::new(RichText::new(text).size(20.0)).min_size(vec2(64.0, 52.0)))
}

/// A tool tile: its icon over its name, washed in the accent colour while it's on. An icon that
/// isn't a Lucide name (such as "♪") is drawn as text.
fn tile(ui: &mut egui::Ui, icon: &str, text: &str, on: bool) -> egui::Response {
    let t = crate::theme::Tokens::get(ui.ctx());
    let font = egui::FontId::proportional(13.0);
    let w = ui.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), font.clone(), t.text).size().x).max(40.0) + 20.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 62.0), egui::Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, ui.is_enabled(), text));
    let fill = if on {
        t.accent_soft
    } else if resp.is_pointer_button_down_on() {
        t.pressed
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 14.0, fill);
    let ink = if on { t.accent_text } else { t.icon };
    let at = rect.center_top() + vec2(0.0, 22.0);
    if crate::icons::exists(icon) {
        crate::icons::paint(ui, egui::Rect::from_center_size(at, vec2(24.0, 24.0)), icon, 24.0, ink);
    } else {
        ui.painter().text(at, Align2::CENTER_CENTER, icon, egui::FontId::proportional(26.0), ink);
    }
    ui.painter().text(rect.center_bottom() - vec2(0.0, 11.0), Align2::CENTER_CENTER, text, font, if on { t.accent_text } else { t.text });
    resp
}

/// A tile on its own card, for the start screen's buttons; in a wrapping row it starts a new
/// row when it doesn't fit (egui places a frame before knowing its size, so it can't wrap one,
/// and an overflowing row widened the whole start screen past the screen). Whether it was tapped.
fn card_tile(ui: &mut egui::Ui, icon: &str, text: &str) -> bool {
    let w = ui.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), egui::FontId::proportional(13.0), Color32::WHITE).size().x).max(40.0) + 20.0;
    // The tile, the card's margins and stroke, and the gap after it.
    let need = w + 2.0 * 10.0 + 2.0 + ui.spacing().item_spacing.x;
    if ui.available_size_before_wrap().x < need && ui.cursor().min.x > ui.max_rect().min.x + 1.0 {
        ui.end_row();
    }
    card(ui).show(ui, |ui| tile(ui, icon, text, false)).inner.clicked()
}

/// The floating card the bar and the music sheet sit on.
fn card(ui: &egui::Ui) -> egui::Frame {
    let t = crate::theme::Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.card)
        .stroke(egui::Stroke::new(1.0, t.border))
        .corner_radius(18.0)
        .inner_margin(egui::Margin::symmetric(10, 6))
        .shadow(ui.visuals().popup_shadow)
}

/// Whether the touch interface draws the whole window this frame (the start screen or the music
/// view) instead of the editor.
pub(crate) fn owns_window(app: &PdfCraftApp) -> bool {
    app.touch.enabled && (app.touch.home || app.active.is_none() || app.touch.focus)
}

/// The start screen or the music view, whichever shows.
pub(crate) fn frame(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    note_touch(app, ui.ctx());
    if app.touch.home || app.active.is_none() {
        set_bottom_room(0.0);
        home(app, ui);
    } else {
        app.touch.shelf = None;
        focus_frame(app, ui);
    }
}

/// The start screen: a grid of the scores in the library folders and the recent files, a search
/// field, and a way to open one from anywhere.
fn home(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    if app.touch.shelf.is_none() {
        app.touch.shelf = Some(shelf(app));
        app.touch.concerts = concerts(&app.touch.library);
    }
    let t = crate::theme::Tokens::get(ui.ctx());
    let mut concert: Option<PathBuf> = None;
    let mut open: Option<PathBuf> = None;
    let mut toggle: Option<PathBuf> = None;
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(t.pasteboard).inner_margin(egui::Margin::symmetric(28, 24))).show(ui, |ui| {
        // The title, then the buttons in as many rows as they need: in one row with the title
        // they grew wider than the screen once a score was open, pushing the list off its left.
        ui.horizontal(|ui| {
            ui.label(RichText::new(es("Scores", "Partituras")).size(32.0).strong().color(t.text));
            let n = app.touch.shelf.as_ref().map_or(0, Vec::len);
            ui.label(RichText::new(format!("{n} PDF")).size(15.0).color(t.text_muted));
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            if let Some(chosen) = &app.touch.choosing {
                // Choosing scores: take them off the start screen, choose them all, or stop.
                let n = chosen.len();
                if card_tile(ui, "list-checks", es("All", "Todas")) {
                    let all: std::collections::BTreeSet<PathBuf> = app.touch.shelf.iter().flatten().cloned().collect();
                    app.touch.choosing = Some(all);
                }
                let remove = format!("{} ({n})", es("Remove from list", "Quitar de la lista"));
                if n > 0 && card_tile(ui, "eye-off", &remove) {
                    hide_chosen(app);
                }
                if card_tile(ui, "x", es("Cancel", "Cancelar")) {
                    app.touch.choosing = None;
                }
                return;
            }
            if app.active.is_some() && card_tile(ui, "book-open", es("Back", "Volver")) {
                app.touch.home = false;
                app.touch.focus = true;
            }
            if !app.views.is_empty() && card_tile(ui, "circle-x", es("Close all", "Cerrar todas")) {
                close_all(app);
            }
            if card_tile(ui, "folder-open", es("Open from…", "Abrir desde…")) {
                app.touch.home = false;
                app.run_command("file.open");
            }
            if card_tile(ui, "folder", es("Open folder…", "Abrir carpeta…")) {
                app.ask(crate::pickers::Ask::Folder(rfd::AsyncFileDialog::new()), None, |app, paths| {
                    if let Some(dir) = paths.first() {
                        open_concert(app, dir);
                    }
                });
            }
            if card_tile(ui, "rotate-cw", es("Refresh", "Actualizar")) {
                app.touch.shelf = None;
            }
            if app.touch.shelf.as_ref().is_some_and(|s| !s.is_empty())
                && card_tile(ui, "list-checks", es("Choose", "Elegir"))
            {
                app.touch.choosing = Some(std::collections::BTreeSet::new());
            }
            if !app.touch.hidden.is_empty() && card_tile(ui, "eye", es("Show hidden", "Mostrar ocultas")) {
                app.touch.hidden.clear();
                app.touch.shelf = None;
            }
        });
        ui.add_space(14.0);
        // Search by name.
        card(ui).show(ui, |ui| {
            // A row as tall as the field, so the magnifier sits level with its text (a plain
            // `horizontal` row starts short and centres the icon on that).
            ui.allocate_ui_with_layout(vec2(ui.available_width(), 44.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(4.0);
                ui.add(crate::icons::image("search", 20.0, t.icon));
                ui.add_space(8.0);
                ui.add(
                    egui::TextEdit::singleline(&mut app.touch.search)
                        .hint_text(es("Search scores", "Buscar partituras"))
                        .font(egui::FontId::proportional(19.0))
                        .frame(egui::Frame::NONE)
                        .desired_width(f32::INFINITY)
                        .margin(egui::Margin::symmetric(0, 10)),
                );
            });
        });
        ui.add_space(16.0);
        let want = app.touch.search.trim().to_lowercase();
        let shelf: Vec<&PathBuf> = app
            .touch
            .shelf
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|p| want.is_empty() || p.file_name().is_some_and(|n| n.to_string_lossy().to_lowercase().contains(&want)))
            .collect();
        if shelf.is_empty() && app.touch.concerts.is_empty() {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                crate::icons::paint(
                    ui,
                    egui::Rect::from_center_size(ui.cursor().center_top() + vec2(0.0, 24.0), vec2(48.0, 48.0)),
                    "files",
                    48.0,
                    t.text_faint,
                );
                ui.add_space(64.0);
                let text = if want.is_empty() {
                    es(
                        "No PDFs yet. Put scores in Download or Documents, or tap \"Open from…\".",
                        "Aún no hay PDFs. Copia partituras en Descargas o Documentos, o toca \"Abrir desde…\".",
                    )
                } else {
                    es("No score has that name.", "Ninguna partitura se llama así.")
                };
                ui.label(RichText::new(text).size(17.0).color(t.text_muted));
            });
            return;
        }
        // Cards in as many columns as fit.
        const CARD_W: f32 = 250.0;
        const GAP: f32 = 14.0;
        // Less the scroll bar's room, so a long list's bar never pushes the last column out.
        let width = ui.available_width() - ui.spacing().scroll.allocated_width() - 2.0;
        let cols = ((width + GAP) / (CARD_W + GAP)).floor().max(1.0);
        let card_w = (width - GAP * (cols - 1.0)) / cols;
        let concerts: Vec<&(PathBuf, usize)> = app
            .touch
            .concerts
            .iter()
            .filter(|(p, _)| want.is_empty() || p.file_name().is_some_and(|n| n.to_string_lossy().to_lowercase().contains(&want)))
            .collect();
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            if !concerts.is_empty() {
                ui.label(RichText::new(es("Concerts (folders)", "Conciertos (carpetas)")).size(19.0).strong().color(t.text));
                ui.add_space(8.0);
                for row in concerts.chunks(cols as usize) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = GAP;
                        for (dir, n) in row {
                            let title = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                            let sub = format!("{n} {}", es("scores", "partituras"));
                            if score_card(ui, "folder-open", &title, &sub, card_w, None).clicked() && app.touch.choosing.is_none() {
                                concert = Some(dir.clone());
                            }
                        }
                    });
                    ui.add_space(GAP);
                }
                ui.add_space(8.0);
                ui.label(RichText::new(es("Scores", "Partituras")).size(19.0).strong().color(t.text));
                ui.add_space(8.0);
            }
            for row in shelf.chunks(cols as usize) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = GAP;
                    for path in row {
                        let title = path.file_stem().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                        let folder = path.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                        let chosen = app.touch.choosing.as_ref().map(|c| c.contains(*path));
                        let resp = score_card(ui, "file-text", &title, &folder, card_w, chosen);
                        // A long press starts choosing with this score; while choosing a tap
                        // chooses or unchooses.
                        if resp.long_touched() || (chosen.is_some() && resp.clicked()) {
                            toggle = Some((*path).clone());
                        } else if resp.clicked() {
                            open = Some((*path).clone());
                        }
                    }
                });
                ui.add_space(GAP);
            }
        });
    });
    if let Some(path) = toggle {
        let chosen = app.touch.choosing.get_or_insert_with(Default::default);
        if !chosen.remove(&path) {
            chosen.insert(path);
        }
    }
    if let Some(path) = open {
        app.touch.home = false;
        app.touch.focus = true;
        app.touch.concert.clear();
        #[cfg(not(target_arch = "wasm32"))]
        app.open_path(&path.to_string_lossy());
    }
    if let Some(dir) = concert {
        open_concert(app, &dir);
    }
}

/// The PDFs directly in `dir`, by name, numbers in order ("2 …" before "10 …").
fn pdfs_in(dir: &std::path::Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut found: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .take(MAX_SHELF * 4)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf")) && p.is_file())
        .collect();
    found.sort_by_key(|p| natural_key(&p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default()));
    found.truncate(MAX_SHELF);
    found
}

/// `name` with each run of digits padded to ten, so names sort with their numbers in order.
fn natural_key(name: &str) -> String {
    let mut out = String::new();
    let mut digits = String::new();
    for c in name.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            continue;
        }
        if !digits.is_empty() {
            out.push_str(&format!("{digits:0>10}"));
            digits.clear();
        }
        out.push(c);
    }
    if !digits.is_empty() {
        out.push_str(&format!("{digits:0>10}"));
    }
    out
}

/// The library folders' subfolders that hold PDFs: each one a concert.
fn concerts(library: &[PathBuf]) -> Vec<(PathBuf, usize)> {
    let mut out = Vec::new();
    for dir in library {
        let Ok(entries) = std::fs::read_dir(dir) else { continue };
        let mut subs: Vec<PathBuf> = entries.filter_map(Result::ok).take(MAX_SHELF).map(|e| e.path()).filter(|p| p.is_dir()).collect();
        subs.sort_by_key(|p| natural_key(&p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default()));
        for sub in subs {
            let n = pdfs_in(&sub).len();
            if n > 0 {
                out.push((sub, n));
            }
        }
    }
    out
}

/// Open every PDF in `dir` as a concert: the first one shows, the others wait in their tabs, and
/// Previous and Next (or a swipe past the last page) go through them in order.
pub(crate) fn open_concert(app: &mut PdfCraftApp, dir: &std::path::Path) {
    let program = pdfs_in(dir);
    if program.is_empty() {
        return;
    }
    app.touch.home = false;
    app.touch.focus = true;
    // The last one opened shows: open them from the end, so the first is the one on screen.
    #[cfg(not(target_arch = "wasm32"))]
    for p in program.iter().rev() {
        app.open_path(&p.to_string_lossy());
    }
    app.touch.concert = program;
}

/// Where the score showing is in the concert: its position and how many there are.
fn concert_place(app: &PdfCraftApp) -> Option<(usize, usize)> {
    let path = app.active.and_then(|i| app.views.get(i)).and_then(|v| app.session.get(v.id)).and_then(|d| d.path.clone())?;
    let at = app.touch.concert.iter().position(|p| std::path::Path::new(&path) == p.as_path())?;
    Some((at, app.touch.concert.len()))
}

/// Show the concert's score at `at` (open already: its tab comes forward).
fn go_to_piece(app: &mut PdfCraftApp, at: usize) {
    if let Some(p) = app.touch.concert.get(at).cloned() {
        #[cfg(not(target_arch = "wasm32"))]
        app.open_path(&p.to_string_lossy());
    }
}

/// A card on the start screen: an icon, a title (two lines at most) and a line under it.
/// `chosen` is whether it's chosen, while the start screen is choosing scores.
fn score_card(ui: &mut egui::Ui, icon_name: &str, name: &str, under: &str, width: f32, chosen: Option<bool>) -> egui::Response {
    let t = crate::theme::Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 96.0), egui::Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, name));
    let fill = if resp.is_pointer_button_down_on() { t.pressed } else { t.card };
    ui.painter().rect(rect, 16.0, fill, egui::Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    // The icon on a soft accent square.
    let icon = egui::Rect::from_min_size(rect.min + vec2(14.0, 18.0), vec2(48.0, 60.0));
    ui.painter().rect_filled(icon, 10.0, t.accent_soft);
    crate::icons::paint(ui, egui::Rect::from_center_size(icon.center(), vec2(28.0, 28.0)), icon_name, 28.0, t.accent_text);
    let left = icon.right() + 14.0;
    let text_w = (rect.right() - left - 12.0).max(20.0);
    let galley = ui.painter().layout(name.to_owned(), egui::FontId::proportional(17.0), t.text, text_w);
    let rows = galley.rows.len();
    let galley = if rows > 2 {
        // Too long for two lines: one line, cut with an ellipsis.
        let mut job = egui::text::LayoutJob::simple_singleline(name.to_owned(), egui::FontId::proportional(17.0), t.text);
        job.wrap = egui::text::TextWrapping::truncate_at_width(text_w);
        ui.painter().layout_job(job)
    } else {
        galley
    };
    ui.painter().galley(egui::pos2(left, rect.top() + 20.0), galley, t.text);
    // One line, cut with an ellipsis: a long folder name wrapped out of the card.
    let mut job = egui::text::LayoutJob::simple_singleline(under.to_owned(), egui::FontId::proportional(13.5), t.text_muted);
    job.wrap = egui::text::TextWrapping::truncate_at_width(text_w);
    let folder_text = ui.painter().layout_job(job);
    ui.painter().galley(egui::pos2(left, rect.bottom() - 30.0), folder_text, t.text_muted);
    if let Some(on) = chosen {
        // A tick in a circle at the top right: filled when chosen.
        let at = rect.right_top() + vec2(-20.0, 20.0);
        if on {
            ui.painter().rect_stroke(rect, 16.0, egui::Stroke::new(3.0, t.accent_text), egui::StrokeKind::Inside);
            ui.painter().circle_filled(at, 12.0, t.accent_text);
            crate::icons::paint(ui, egui::Rect::from_center_size(at, vec2(16.0, 16.0)), "check", 16.0, Color32::WHITE);
        } else {
            ui.painter().circle_stroke(at, 12.0, egui::Stroke::new(2.0, t.text_muted));
        }
    }
    resp
}

/// Take the chosen scores off the start screen (and out of the recent files); the files stay.
fn hide_chosen(app: &mut PdfCraftApp) {
    let Some(chosen) = app.touch.choosing.take() else { return };
    app.recent.retain(|r| !chosen.contains(std::path::Path::new(&r.path)));
    for p in chosen {
        if !app.touch.hidden.contains(&p) {
            app.touch.hidden.push(p);
        }
    }
    app.touch.shelf = None;
}

/// The PDFs the start screen lists: the recent files first, then each library folder's PDFs by
/// name; each once.
fn shelf(app: &PdfCraftApp) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = app.recent.iter().map(|r| PathBuf::from(&r.path)).filter(|p| p.is_file()).collect();
    let is_pdf = |p: &PathBuf| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf"));
    for dir in &app.touch.library {
        let Ok(entries) = std::fs::read_dir(dir) else { continue };
        let mut found: Vec<PathBuf> =
            entries.filter_map(Result::ok).take(MAX_SHELF * 4).map(|e| e.path()).filter(|p| is_pdf(p) && p.is_file()).collect();
        found.sort();
        out.extend(found);
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|p| !app.touch.hidden.contains(p) && seen.insert(p.clone()));
    out.truncate(MAX_SHELF);
    out
}

/// The music view: the document alone, the bar and the music sheet over it.
fn focus_frame(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    // A document shown here for the first time fills the screen's width with its music (its white
    // margins off the sides): first its page's width, then, once the pages are laid out, its ink.
    if let Some(i) = app.active
        && let Some(view) = app.views.get_mut(i)
        && app.touch.fitted != Some(view.id)
    {
        if app.touch.fitting == Some(view.id) && !view.screen_rects.is_empty() {
            app.touch.fitted = Some(view.id);
            app.touch.fitting = None;
            fit_music(app, i);
        } else if app.touch.fitting != Some(view.id) {
            app.touch.fitting = Some(view.id);
            view.set_fit(crate::canvas::Fit::Width);
        }
    }
    two_fingers_pan(app, &ctx);
    // A score taken off the start screen and opened again comes back to it.
    if !app.touch.hidden.is_empty()
        && let Some(path) = app.active.and_then(|i| app.views.get(i)).and_then(|v| app.session.get(v.id)).and_then(|d| d.path.clone())
    {
        app.touch.hidden.retain(|p| p.as_path() != std::path::Path::new(&path));
    }
    // A mark stays chosen only while moving and resizing marks: with any other tool its handles
    // would linger over the score (a placed mark comes out chosen).
    if app.quick_tool != QuickTool::Select
        && let Some(view) = app.active.and_then(|i| app.views.get_mut(i))
        && view.comments.selected.is_some()
    {
        view.comments.clear_selection();
    }
    if !app.touch.music {
        set_bottom_room(0.0);
    }
    let started = std::time::Instant::now();
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(Color32::from_gray(64))).show(ui, |ui| {
        if let Some(i) = app.active {
            crate::canvas::document_area(app, i, ui);
        }
    });
    // ponytail: temporary frame-time probe for the Android scroll stutter; remove once fixed.
    #[cfg(target_os = "android")]
    {
        let (took, gap, down) = (started.elapsed().as_secs_f32() * 1000.0, ctx.input(|i| i.unstable_dt) * 1000.0, ctx.input(|i| i.pointer.any_down()));
        // Only while a finger is down (a drag): between gestures the app rests on purpose.
        if down && (took > 20.0 || gap > 25.0) {
            log::warn!("slow frame: pages {took:.1} ms, since last frame {gap:.1} ms");
        }
        // ponytail: temporary motion probe (finger vs page each frame), for the "jumpy" drag.
        if down && let Some(view) = app.active.and_then(|i| app.views.get(i)) {
            let finger = ctx.input(|i| i.pointer.delta());
            log::info!("drag: dt {gap:.1} ms, finger {:.1} {:.1}, page at {:.1} {:.1}", finger.x, finger.y, view.last_scroll.x, view.last_scroll.y);
        }
    }
    #[cfg(not(target_os = "android"))]
    let _ = started;
    // With the music sheet open, the chosen tool stays chosen after each use.
    if app.touch.music {
        match app.quick_tool {
            QuickTool::Hand => {}
            QuickTool::Select => {
                if let Some(t) = app.touch.tool {
                    app.quick_tool = t;
                }
            }
            t => app.touch.tool = Some(t),
        }
    } else {
        app.touch.tool = None;
    }
    swipe(app, &ctx);
    // A tap on the page while moving it (not on a bar or a jump mark) shows or hides the bar; with
    // any other tool the tap is the tool's.
    let jumped = crate::music_ui::take_jumped();
    let moving = app.quick_tool == QuickTool::Hand;
    let over_bar = ctx.input(|i| i.pointer.interact_pos()).and_then(|p| ctx.layer_id_at(p)).is_some_and(|l| l.order != egui::Order::Background);
    if !jumped && moving && !over_bar && ctx.input(|i| i.pointer.primary_clicked()) {
        app.touch.bar = !app.touch.bar;
    }
    if app.touch.bar {
        bar(app, &ctx);
    }
    if app.touch.music {
        music_sheet(app, &ctx);
    }
}

/// Fit view `i`'s music (its ink, not its white margins) to the screen's width, and remember
/// that zoom; the page's width when that can't be found.
fn fit_music(app: &mut PdfCraftApp, i: usize) {
    // A column holds music when 1.5 % of it is ink: staves cross many, a scan's specks a few.
    if let Err(e) = app.fit_visible_with(i, Some(0.015)) {
        log::warn!("could not fit the music to the screen: {e}");
    }
    app.touch.fit_zoom = app.views.get(i).map(|v| v.zoom);
    FIT_ZOOM.store(app.touch.fit_zoom.unwrap_or(0.0).to_bits(), std::sync::atomic::Ordering::Relaxed);
}

/// The music view's fitted zoom ([`Touch::fit_zoom`]), as f32 bits, for the pages' drag.
static FIT_ZOOM: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// While the score is fitted to the screen (`zoom` no more than the music view's fit), one
/// finger moves the page along one axis only, the one it set off along (up and down, or
/// sideways). Zoomed in, and with two fingers, it moves freely. `moved` is this frame's motion
/// of the drag on `id`.
pub(crate) fn lock_axis(ui: &egui::Ui, id: egui::Id, moved: egui::Vec2, zoom: f32) -> egui::Vec2 {
    let fit = f32::from_bits(FIT_ZOOM.load(std::sync::atomic::Ordering::Relaxed));
    if !on() || fit <= 0.0 || zoom > fit * 1.02 || ui.input(|i| i.multi_touch().is_some()) {
        return moved;
    }
    let key = id.with("touch-axis");
    let total = ui.input(|i| i.pointer.total_drag_delta()).unwrap_or_default();
    let mut axis = ui.data(|d| d.get_temp::<u8>(key));
    if ui.ctx().drag_started_id() == Some(id) || axis.is_none() {
        // 1: sideways, 2: up and down; decided by the drag's start.
        let decided = if total.x.abs() > total.y.abs() { 1 } else { 2 };
        ui.data_mut(|d| d.insert_temp(key, decided));
        axis = Some(decided);
    }
    if ui.ctx().drag_stopped_id() == Some(id) {
        ui.data_mut(|d| d.remove::<u8>(key));
    }
    match axis {
        Some(1) => egui::vec2(moved.x, 0.0),
        _ => egui::vec2(0.0, moved.y),
    }
}

/// A quick sideways swipe with the Hand tool turns the page (left: next, right: previous), while
/// the page fits the screen; zoomed in, a sideways drag pans instead.
fn swipe(app: &mut PdfCraftApp, ctx: &egui::Context) {
    /// How far sideways (points) a swipe goes, at least, and how much more than up or down.
    const MIN: f32 = 90.0;
    let (pressed, released, pos, fingers) =
        ctx.input(|i| (i.pointer.any_pressed(), i.pointer.any_released(), i.pointer.interact_pos(), i.multi_touch().is_some()));
    if app.quick_tool != QuickTool::Hand || fingers {
        app.touch.swipe_from = None;
        return;
    }
    if pressed {
        app.touch.swipe_from = pos;
    }
    if !released {
        return;
    }
    let (Some(from), Some(to)) = (app.touch.swipe_from.take(), pos) else { return };
    let d = to - from;
    if d.x.abs() < MIN || d.x.abs() < d.y.abs() * 2.0 {
        return;
    }
    let Some(view) = app.active.and_then(|i| app.views.get_mut(i)) else { return };
    // Zoomed in past the fitted width, a sideways drag pans instead.
    if view.fit == crate::canvas::Fit::None && app.touch.fit_zoom.is_none_or(|z| view.zoom > z * 1.02) {
        return;
    }
    let (id, current) = (view.id, view.current);
    let last = app.session.get(id).map_or(0, |doc| doc.info.pages.len().saturating_sub(1));
    // In a concert, a swipe past the last page goes on to the next score (and back).
    if let Some((at, n)) = concert_place(app) {
        if d.x < 0.0 && current >= last && at + 1 < n {
            return go_to_piece(app, at + 1);
        }
        if d.x > 0.0 && current == 0 && at > 0 {
            return go_to_piece(app, at - 1);
        }
    }
    let Some(view) = app.active.and_then(|i| app.views.get_mut(i)) else { return };
    let page = if d.x < 0.0 { current.saturating_add(1) } else { current.saturating_sub(1) };
    view.go_to_page(page);
}

/// Close every open score (those with unsaved marks ask first) and show the start screen: a
/// concert opens a tab per piece.
fn close_all(app: &mut PdfCraftApp) {
    app.run_command("file.close_all");
    app.touch.concert.clear();
    app.touch.home = true;
    app.touch.bar = false;
    close_music(app);
}

/// Leave the music marks: back to moving the page.
fn close_music(app: &mut PdfCraftApp) {
    app.touch.music = false;
    app.quick_tool = QuickTool::Hand;
}

/// The music view's bar, floating along the top.
fn bar(app: &mut PdfCraftApp, ctx: &egui::Context) {
    egui::Area::new(egui::Id::unique("touch_bar")).anchor(Align2::LEFT_TOP, vec2(12.0, 12.0)).order(egui::Order::Foreground).show(ctx, |ui| {
        card(ui).show(ui, |ui| {
            ui.set_width(ctx.content_rect().width() - 44.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                if tile(ui, "house", es("Scores", "Partituras"), false).clicked() {
                    app.touch.home = true;
                    app.touch.bar = false;
                    close_music(app);
                }
                if let Some((at, n)) = concert_place(app) {
                    if tile(ui, "chevron-left", es("Previous", "Anterior"), false).clicked() && at > 0 {
                        go_to_piece(app, at - 1);
                    }
                    ui.label(RichText::new(format!("{}/{n}", at + 1)).size(16.0).strong());
                    if tile(ui, "chevron-right", es("Next", "Siguiente"), false).clicked() && at + 1 < n {
                        go_to_piece(app, at + 1);
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    if tile(ui, "chevron-up", es("Hide", "Ocultar"), false).clicked() {
                        app.touch.bar = false;
                    }
                    if app.views.len() > 1 && tile(ui, "circle-x", es("Close all", "Cerrar todas"), false).clicked() {
                        close_all(app);
                    }
                    if tile(ui, "sliders-horizontal", es("Editor", "Editor"), false).clicked() {
                        app.touch.focus = false;
                        app.touch.bar = false;
                        // The editor starts out moving the page, as fingers expect.
                        close_music(app);
                    }
                    if tile(ui, "maximize", es("Fit", "Ajustar"), false).clicked()
                        && let Some(view) = app.active.and_then(|i| app.views.get_mut(i))
                    {
                        // The whole sheet, as large as the screen takes it.
                        view.set_fit(crate::canvas::Fit::Page);
                    }
                    if tile(ui, "♪", es("Marks", "Marcas"), app.touch.music).clicked() {
                        if app.touch.music {
                            close_music(app);
                        } else {
                            app.touch.music = true;
                            app.touch.folded = false;
                        }
                    }
                    if tile(ui, "save", es("Save", "Guardar"), false).clicked() {
                        app.run_command("file.save");
                    }
                    if tile(ui, "redo-2", es("Redo", "Rehacer"), false).clicked() {
                        app.run_command("edit.redo");
                    }
                    if tile(ui, "undo-2", es("Undo", "Deshacer"), false).clicked() {
                        app.run_command("edit.undo");
                    }
                    if let Some(name) = app.active.and_then(|i| crate::canvas::tab_label(app, i)) {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.add(egui::Label::new(RichText::new(name).size(17.0).strong()).truncate());
                        });
                    }
                });
            });
        });
    });
}

/// With the Select tool: make the selected mark larger or smaller (about its middle), or delete
/// it. A tap on a mark selects it; dragging it moves it, and its corner handles resize it.
fn selection_tiles(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    let Some(i) = app.active else { return };
    let Some(view) = app.views.get(i) else { return };
    let Some((page, index)) = view.comments.selected else {
        ui.label(
            RichText::new(es("Tap a mark on the score to move it or change its size", "Toca una marca de la partitura para moverla o cambiar su tamaño"))
                .size(16.0)
                .strong(),
        );
        return;
    };
    let Some(rect) = app.session.get(view.id).and_then(|d| d.info.annotations.iter().find(|a| a.page == page && a.index == index).map(|a| a.rect))
    else {
        return;
    };
    let scale = |f: f64| {
        let [x0, y0, x1, y1] = rect.map(f64::from);
        let (cx, cy, hw, hh) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0, (x1 - x0) / 2.0 * f, (y1 - y0) / 2.0 * f);
        pdfcraft_engine::Edit::ResizeAnnotation { page, index, rect: [cx - hw, cy - hh, cx + hw, cy + hh] }
    };
    let mut edit = None;
    if tile(ui, "zoom-in", es("Larger", "Más grande"), false).clicked() {
        edit = Some(scale(1.25));
    }
    if tile(ui, "zoom-out", es("Smaller", "Más pequeña"), false).clicked() && (rect[2] - rect[0]).min(rect[3] - rect[1]) > 4.0 {
        edit = Some(scale(0.8));
    }
    if tile(ui, "trash-2", es("Delete", "Eliminar"), false).clicked() {
        edit = Some(pdfcraft_engine::Edit::DeleteAnnotation { page, index });
    }
    // A written note turns over (stem down) and takes or loses its dot.
    let note = app
        .session
        .get(view.id)
        .and_then(|d| d.info.annotations.iter().find(|a| a.page == page && a.index == index))
        .and_then(|a| Some((crate::music_ui::parse_note(a.contents.as_deref()?)?, a.author.clone().unwrap_or_default())));
    if let Some(((kind, down, dot), author)) = note {
        let turn = tile(ui, "rotate-cw", es("Turn", "Girar"), down).clicked();
        let dotted = tile(ui, "circle-dot", es("Dot", "Puntillo"), dot).clicked();
        if turn || dotted {
            let (down, new_dot) = (down ^ turn, dot ^ dotted);
            // The dot widens the box (on the right of the head as shown); the head stays put.
            let [x0, y0, x1, y1] = rect.map(f64::from);
            let room = if new_dot == dot { 0.0 } else { pdfcraft_engine::NOTE_DOT_ROOM * (y1 - y0) / kind.size().1.max(1.0) };
            let x1 = if new_dot { x1 + room } else { (x1 - room).max(x0 + 1.0) };
            let by = crate::music_ui::note_flags(down, new_dot);
            let contents = crate::music_ui::note_contents(kind, by.as_deref());
            let shape = pdfcraft_engine::Shape::Stamp { rect: [x0, y0, x1, y1], stamp: kind, by };
            let add = pdfcraft_engine::NewAnnotation { page, style: pdfcraft_engine::Style::default_for(&shape), shape, contents, author };
            // One step to undo: the note is replaced by its turned or dotted self.
            edit = Some(pdfcraft_engine::Edit::Batch {
                label: es("Change note", "Cambiar nota").to_owned(),
                edits: vec![pdfcraft_engine::Edit::DeleteAnnotation { page, index }, pdfcraft_engine::Edit::AddAnnotation(add)],
            });
        }
    }
    ui.label(
        RichText::new(es("Drag the mark to move it · pull a corner to resize", "Arrastra la marca para moverla · tira de una esquina para el tamaño"))
            .size(15.0)
            .weak(),
    );
    if let Some(e) = edit
        && let Some(view) = app.views.get_mut(i)
    {
        if matches!(e, pdfcraft_engine::Edit::DeleteAnnotation { .. } | pdfcraft_engine::Edit::Batch { .. }) {
            view.comments.selected = None;
        }
        view.pending_edit = Some(e);
        // The pages apply it on the next frame.
        ui.ctx().request_repaint();
    }
}

/// The highlighter's colours, and how wide it marks.
const HIGHLIGHTS: [[f64; 3]; 6] = [[1.0, 0.92, 0.0], [0.45, 0.9, 0.3], [1.0, 0.45, 0.75], [0.35, 0.7, 1.0], [1.0, 0.6, 0.1], [0.75, 0.5, 1.0]];
const HIGHLIGHT_WIDTHS: [f64; 3] = [6.0, 12.0, 20.0];

/// Swatches for the highlighter's colour, then its widths.
fn highlighter_colours(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    use crate::comments::CommentTool;
    let t = crate::theme::Tokens::get(ui.ctx());
    let mut style = app.comment_prefs.style(CommentTool::FreehandHighlight);
    let mut changed = false;
    for c in HIGHLIGHTS {
        let on = style.color.iter().zip(c).all(|(a, b)| (a - b).abs() < 0.02);
        let (rect, resp) = ui.allocate_exact_size(vec2(48.0, 62.0), egui::Sense::click());
        let fill = Color32::from_rgb((c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8);
        ui.painter().circle(rect.center(), 17.0, fill, egui::Stroke::new(if on { 3.0 } else { 1.0 }, if on { t.text } else { t.border }));
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "colour"));
        if resp.clicked() {
            style.color = c;
            changed = true;
        }
    }
    ui.add_space(12.0);
    for w in HIGHLIGHT_WIDTHS {
        let on = (style.width - w).abs() < 0.5;
        let (rect, resp) = ui.allocate_exact_size(vec2(44.0, 62.0), egui::Sense::click());
        if on {
            ui.painter().rect_filled(rect, 14.0, t.accent_soft);
        }
        let h = (w as f32 * 0.9).min(20.0);
        ui.painter().rect_filled(egui::Rect::from_center_size(rect.center(), vec2(28.0, h)), 3.0, if on { t.accent_text } else { t.icon });
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "width"));
        if resp.clicked() {
            style.width = w;
            changed = true;
        }
    }
    if changed {
        app.comment_prefs.set_style(CommentTool::FreehandHighlight, style);
    }
}

/// The eraser's sizes: its thickness setting, which sets how wide it rubs (`comments::paint`).
const ERASER_SIZES: [(f64, &str); 3] = [(2.0, "S"), (6.0, "M"), (12.0, "L")];

/// The eraser's three sizes, each a dot as wide as it rubs (to scale).
fn eraser_sizes(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    use crate::comments::CommentTool;
    let t = crate::theme::Tokens::get(ui.ctx());
    let mut style = app.comment_prefs.style(CommentTool::Eraser);
    for (width, name) in ERASER_SIZES {
        let on = (style.width - width).abs() < 0.5;
        let (rect, resp) = ui.allocate_exact_size(vec2(44.0, 62.0), egui::Sense::click());
        if on {
            ui.painter().rect_filled(rect, 14.0, t.accent_soft);
        }
        let r = (3.0 + width as f32 * 1.3).min(18.0);
        ui.painter().circle_filled(rect.center(), r, if on { t.accent_text } else { t.icon });
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, name));
        if resp.clicked() {
            style.width = width;
            app.comment_prefs.set_style(CommentTool::Eraser, style.clone());
        }
    }
}

/// The music marks, the pen and the eraser, floating along the bottom; folded, one row.
fn music_sheet(app: &mut PdfCraftApp, ctx: &egui::Context) {
    use crate::comments::CommentTool;
    // One area per fold state: an area anchored to the bottom is placed by the size it had the frame
    // before, so a sheet unfolded in the folded one's area grew down off the screen.
    let shown = egui::Area::new(egui::Id::unique(("touch_music", app.touch.folded)))
        .anchor(Align2::LEFT_BOTTOM, vec2(12.0, -12.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            card(ui).show(ui, |ui| {
                ui.set_width(ctx.content_rect().width() - 44.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    let tool = app.quick_tool;
                    if tile(ui, "hand", es("Pan", "Desplazar"), tool == QuickTool::Hand).clicked() {
                        app.quick_tool = QuickTool::Hand;
                        app.touch.tool = None;
                    }
                    let pen = QuickTool::Comment(CommentTool::Ink);
                    if tile(ui, "pencil", es("Pencil", "Lápiz"), tool == pen).clicked() {
                        app.quick_tool = pen;
                    }
                    let marker = QuickTool::Comment(CommentTool::FreehandHighlight);
                    if tile(ui, "highlighter", es("Highlighter", "Subrayador"), tool == marker).clicked() {
                        app.quick_tool = marker;
                    }
                    let eraser = QuickTool::Comment(CommentTool::Eraser);
                    if tile(ui, "eraser", es("Erase", "Borrar"), tool == eraser).clicked() {
                        app.quick_tool = eraser;
                    }
                    let select = QuickTool::Select;
                    // Named for what it's for: a player looks for "move this mark", not "select".
                    if tile(ui, "move", es("Move / resize", "Mover / tamaño"), tool == select).clicked() {
                        app.quick_tool = select;
                        app.touch.tool = None;
                    }
                    if tile(ui, "undo-2", es("Undo", "Deshacer"), false).clicked() {
                        app.run_command("edit.undo");
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        if tile(ui, "x", es("Close", "Cerrar"), false).clicked() {
                            close_music(app);
                        }
                        let (icon, text) =
                            if app.touch.folded { ("chevron-up", es("Marks", "Marcas")) } else { ("chevron-down", es("Fold", "Plegar")) };
                        if tile(ui, icon, text, false).clicked() {
                            app.touch.folded = !app.touch.folded;
                        }
                        if tile(ui, "panels-top-left", es("Bar", "Barra"), app.touch.bar).clicked() {
                            app.touch.bar = !app.touch.bar;
                        }
                    });
                });
                // What the chosen tool offers: the selected mark's size, the eraser's, the mark placed.
                let tool = app.quick_tool;
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    match tool {
                        QuickTool::Select => selection_tiles(app, ui),
                        QuickTool::Comment(CommentTool::Eraser) => eraser_sizes(app, ui),
                        QuickTool::Comment(CommentTool::FreehandHighlight) => highlighter_colours(app, ui),
                        QuickTool::Stamp(k) => {
                            let text = format!("{} {}", es("Placing", "Poniendo"), k.label());
                            ui.label(RichText::new(text).size(16.0).strong());
                        }
                        _ => {}
                    }
                });
                if app.touch.folded {
                    return;
                }
                let hint = match app.quick_tool {
                    QuickTool::Comment(CommentTool::Ink) => es("Write with your finger on the score.", "Escribe con el dedo sobre la partitura."),
                    QuickTool::Comment(CommentTool::Eraser) => {
                        es("Rub over a mark or a drawing to remove it.", "Pasa el dedo por una marca o un trazo para borrarlo.")
                    }
                    QuickTool::Comment(CommentTool::FreehandHighlight) => {
                        es("Choose a colour and run your finger over what matters.", "Elige un color y pasa el dedo por lo importante.")
                    }
                    QuickTool::Select => es(
                        "Larger and Smaller change the chosen mark's size in steps; Pan goes back to moving the page.",
                        "Más grande y Más pequeña cambian el tamaño de la marca elegida a saltos; Desplazar vuelve a mover la página.",
                    ),
                    QuickTool::Stamp(k) if k.stretches() => es(
                        "Drag across the score to draw it as long as you need.",
                        "Arrastra sobre la partitura para dibujarlo del largo que quieras.",
                    ),
                    _ => es("Choose a mark, then tap the score to place it.", "Elige una marca y toca la partitura para ponerla."),
                };
                ui.label(RichText::new(hint).size(15.0).weak());
                // Always this tall: a scroll area left to shrink to its content came back from
                // folding shorter than before, cutting the marks off.
                let palette_h = ctx.content_rect().height() * 0.34;
                egui::ScrollArea::vertical().auto_shrink([false, false]).min_scrolled_height(palette_h).max_height(palette_h).show(ui, |ui| {
                    crate::music_ui::palette(app, ui, true);
                });
            });
        });
    // The pages scroll on past the last one by as much as the sheet covers, so its end can be
    // brought up above the sheet and marked.
    set_bottom_room(ctx.content_rect().bottom() - shown.response.rect.top() + 12.0);
}

/// In the full editor with a document open: a button back to the music view.
pub(crate) fn editor_overlay(app: &mut PdfCraftApp, ctx: &egui::Context) {
    note_touch(app, ctx);
    if !app.touch.enabled || app.touch.focus || app.active.is_none() {
        return;
    }
    set_bottom_room(0.0);
    two_fingers_pan(app, ctx);
    // Bottom centre, small: the corners hold the editor's own tools. A drag moves it off what it
    // covers; a tap goes to the music view.
    let offset = app.touch.music_button;
    egui::Area::new(egui::Id::unique("touch_reading")).anchor(Align2::CENTER_BOTTOM, vec2(0.0, -56.0) + offset).order(egui::Order::Foreground).show(
        ctx,
        |ui| {
            let accent = ui.visuals().selection.bg_fill;
            let b = egui::Button::new(RichText::new(es("♪ Music view", "♪ Modo música")).size(18.0).strong().color(Color32::WHITE))
                .fill(accent)
                .min_size(vec2(150.0, 44.0))
                .corner_radius(22.0)
                .sense(egui::Sense::click_and_drag());
            let resp = ui.add(b);
            if resp.dragged() {
                // Kept on the screen: at most the screen's half width either side, and between
                // the top and the bottom edge.
                let screen = ctx.content_rect();
                let max = vec2((screen.width() / 2.0 - 80.0).max(0.0), (screen.height() - 120.0).max(0.0));
                let moved = app.touch.music_button + resp.drag_delta();
                app.touch.music_button = vec2(moved.x.clamp(-max.x, max.x), moved.y.clamp(-max.y, 40.0));
            } else if resp.clicked() {
                app.touch.focus = true;
                app.touch.bar = false;
                app.quick_tool = QuickTool::Hand;
            }
        },
    );
}

#[cfg(target_os = "android")]
pub(crate) use browser::Browser;

/// The app's own file browser (Android): open, save and folder panels as one finger-sized list.
#[cfg(target_os = "android")]
mod browser {
    use std::path::{Path, PathBuf};

    use egui::RichText;

    use super::{big, es};
    use crate::PdfCraftApp;
    use crate::files::PickTarget;
    use crate::pickers::{Ask, Then};

    /// Shared storage, where Download and Documents are.
    const STORAGE: &str = "/storage/emulated/0";
    /// The most entries one folder lists.
    const MAX_ENTRIES: usize = 2000;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Kind {
        File,
        Files,
        Folder,
        Save,
    }

    pub(crate) struct Browser {
        kind: Kind,
        title: String,
        /// Extensions to list (lowercase, no dot); empty lists every file.
        exts: Vec<String>,
        dir: PathBuf,
        /// Save: the file name.
        name: String,
        /// Files: the chosen files.
        chosen: Vec<PathBuf>,
        target: Option<PickTarget>,
        then: Option<Then>,
    }

    impl Browser {
        pub(crate) fn new(ask: Ask, target: Option<PickTarget>, then: Then, last: Option<PathBuf>) -> Self {
            let (kind, d) = match ask {
                Ask::File(d) => (Kind::File, d),
                Ask::Files(d) => (Kind::Files, d),
                Ask::Folder(d) => (Kind::Folder, d),
                Ask::Save(d) => (Kind::Save, d),
            };
            let exts =
                d.filters().iter().flat_map(|(_, e)| e.iter()).map(|e| e.trim_start_matches('.').to_ascii_lowercase()).filter(|e| e != "*").collect();
            let start = [d.directory().map(Path::to_path_buf), last, Some(PathBuf::from(STORAGE).join("Download"))]
                .into_iter()
                .flatten()
                .find(|p| p.is_dir())
                .unwrap_or_else(|| PathBuf::from(STORAGE));
            Self { kind, title: d.title().to_owned(), exts, dir: start, name: d.file_name().to_owned(), chosen: Vec::new(), target, then: Some(then) }
        }

        fn lists(&self, p: &Path) -> bool {
            self.exts.is_empty() || p.extension().and_then(|e| e.to_str()).is_some_and(|e| self.exts.iter().any(|x| x.eq_ignore_ascii_case(e)))
        }
    }

    /// What the browser was told this frame.
    enum Outcome {
        Chosen(Vec<PathBuf>),
        Cancel,
    }

    /// Show the browser while it's open; hand the choice (or a cancel) to the picker that asked.
    pub(crate) fn show(app: &mut PdfCraftApp, ctx: &egui::Context) {
        let Some(b) = app.touch.browser.as_mut() else { return };
        let mut outcome = None;
        let screen = ctx.content_rect();
        egui::Modal::new(egui::Id::unique("touch_browser")).show(ctx, |ui| {
            ui.set_width((screen.width() * 0.9).min(900.0));
            let title = if b.title.is_empty() {
                match b.kind {
                    Kind::Save => es("Save as", "Guardar como"),
                    Kind::Folder => es("Choose a folder", "Elegir carpeta"),
                    _ => es("Open", "Abrir"),
                }
                .to_owned()
            } else {
                b.title.clone()
            };
            ui.label(RichText::new(title).size(22.0).strong());
            ui.horizontal_wrapped(|ui| {
                if big(ui, "⬆").clicked()
                    && let Some(up) = b.dir.parent()
                {
                    b.dir = up.to_path_buf();
                }
                for (label, sub) in [(es("Downloads", "Descargas"), "Download"), (es("Documents", "Documentos"), "Documents"), (es("Storage", "Almacenamiento"), "")] {
                    if big(ui, label).clicked() {
                        b.dir = PathBuf::from(STORAGE).join(sub);
                    }
                }
            });
            ui.label(RichText::new(b.dir.display().to_string()).size(14.0).weak());
            let (mut dirs, mut files) = (Vec::new(), Vec::new());
            let listed = std::fs::read_dir(&b.dir).map(|it| {
                for e in it.filter_map(Result::ok).take(MAX_ENTRIES) {
                    let p = e.path();
                    let hidden = p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('.'));
                    if hidden {
                        continue;
                    }
                    if p.is_dir() {
                        dirs.push(p);
                    } else if b.kind != Kind::Folder && b.lists(&p) {
                        files.push(p);
                    }
                }
            });
            dirs.sort();
            files.sort();
            egui::ScrollArea::vertical().max_height(screen.height() * 0.55).auto_shrink([false, true]).show(ui, |ui| {
                if listed.is_err() {
                    ui.label(
                        RichText::new(es(
                            "This folder can't be read. Allow PdfCraft to manage all files: Settings ▸ Apps ▸ PdfCraft ▸ Permissions ▸ Files.",
                            "No se puede leer esta carpeta. Permite a PdfCraft gestionar todos los archivos: Ajustes ▸ Aplicaciones ▸ PdfCraft ▸ Permisos ▸ Archivos.",
                        ))
                        .size(16.0),
                    );
                }
                let row = |ui: &mut egui::Ui, text: String, selected: bool| {
                    ui.add_sized([ui.available_width(), 48.0], egui::Button::new(RichText::new(text).size(18.0)).selected(selected).wrap_mode(egui::TextWrapMode::Truncate))
                };
                for d in &dirs {
                    let n = d.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                    if row(ui, format!("📁  {n}"), false).clicked() {
                        b.dir = d.clone();
                    }
                }
                for f in &files {
                    let n = f.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                    let selected = b.chosen.contains(f) || (b.kind == Kind::Save && n == b.name);
                    if row(ui, format!("📄  {n}"), selected).clicked() {
                        match b.kind {
                            Kind::File => outcome = Some(Outcome::Chosen(vec![f.clone()])),
                            Kind::Files => {
                                if let Some(i) = b.chosen.iter().position(|c| c == f) {
                                    b.chosen.remove(i);
                                } else {
                                    b.chosen.push(f.clone());
                                }
                            }
                            Kind::Save => b.name = n,
                            Kind::Folder => {}
                        }
                    }
                }
                if listed.is_ok() && dirs.is_empty() && files.is_empty() {
                    ui.label(RichText::new(es("Nothing here.", "No hay nada aquí.")).size(16.0).weak());
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                if big(ui, es("Cancel", "Cancelar")).clicked() {
                    outcome = Some(Outcome::Cancel);
                }
                match b.kind {
                    Kind::Save => {
                        ui.add(egui::TextEdit::singleline(&mut b.name).font(egui::FontId::proportional(20.0)).desired_width(ui.available_width() - 180.0));
                        let target = b.dir.join(b.name.trim());
                        // Saving over a file is said plainly on the button.
                        let label = if target.is_file() { es("Replace", "Reemplazar") } else { es("Save here", "Guardar aquí") };
                        if !b.name.trim().is_empty() && big(ui, label).clicked() {
                            outcome = Some(Outcome::Chosen(vec![target]));
                        }
                    }
                    Kind::Folder => {
                        if big(ui, es("Use this folder", "Usar esta carpeta")).clicked() {
                            outcome = Some(Outcome::Chosen(vec![b.dir.clone()]));
                        }
                    }
                    Kind::Files => {
                        let n = b.chosen.len();
                        if n > 0 && big(ui, &format!("{} ({n})", es("Open", "Abrir"))).clicked() {
                            outcome = Some(Outcome::Chosen(b.chosen.clone()));
                        }
                    }
                    Kind::File => {}
                }
            });
        });
        let Some(outcome) = outcome else { return };
        let Some(mut b) = app.touch.browser.take() else { return };
        app.touch.last_dir = Some(b.dir.clone());
        let paths = match outcome {
            Outcome::Chosen(p) => p,
            // An empty pick is a cancel: the picker's flag is released and nothing runs.
            Outcome::Cancel => Vec::new(),
        };
        if let Some(then) = b.then.take() {
            app.pickers.deliver(b.target, paths, then);
        }
    }
}

/// The file browser, where there is one (Android).
pub(crate) fn browser(app: &mut PdfCraftApp, ctx: &egui::Context) {
    #[cfg(target_os = "android")]
    browser::show(app, ctx);
    #[cfg(not(target_os = "android"))]
    let _ = (app, ctx);
}
