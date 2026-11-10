//! The music marks palette (dynamics, hairpins, articulations, fingering, numbers…), shared by
//! the Stamps panel and the touch interface's music sheet. Choosing a mark makes it the stamp
//! tool; each tap on the page places one.
//!
//! The names are in Spanish when the interface is (the shared catalogs don't have these rows);
//! dynamics and Italian terms read the same in every language.

use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering};

use egui::{RichText, vec2};
use pdfcraft_engine::StampKind;

use crate::{PdfCraftApp, QuickTool};

/// The number the next number mark shows.
static NUMBER: AtomicU32 = AtomicU32::new(1);
/// Whether placing a number mark moves on to the next number (counting bars or beats).
static COUNT_UP: AtomicBool = AtomicBool::new(true);

/// The number for a number mark being placed now, moving on to the next when counting up.
pub(crate) fn take_number() -> String {
    let n = NUMBER.load(Ordering::Relaxed);
    if COUNT_UP.load(Ordering::Relaxed) {
        NUMBER.store(n.saturating_add(1).min(9999), Ordering::Relaxed);
    }
    n.to_string()
}

/// The letters named jumps use.
const JUMP_LETTERS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
/// The letter the next named jump gets (an index into [`JUMP_LETTERS`]).
static JUMP_LETTER: AtomicU8 = AtomicU8::new(0);

/// The letter for a named jump being placed now.
pub(crate) fn jump_letter() -> String {
    JUMP_LETTERS.get(usize::from(JUMP_LETTER.load(Ordering::Relaxed))).unwrap_or(&"A").to_string()
}

/// The text a jump mark carries, which is how a tap knows where it leads: "→ A" and "◎ A" for
/// a named jump's start and end (`by` is its letter), the label for segno, coda, D.S., D.C.;
/// empty for every other mark.
pub(crate) fn jump_contents(kind: StampKind, by: Option<&str>) -> String {
    match kind {
        StampKind::JumpFrom => format!("→ {}", by.unwrap_or("A")),
        StampKind::JumpTo => format!("◎ {}", by.unwrap_or("A")),
        k if k.jump_family().is_some() => k.label().to_owned(),
        _ => String::new(),
    }
}

/// `en`, or `es` when the interface is in Spanish.
pub(crate) fn es(en: &'static str, es: &'static str) -> &'static str {
    if crate::i18n::current().code() == "es" { es } else { en }
}

/// A mark's name on its button.
fn name(kind: StampKind) -> &'static str {
    use StampKind::*;
    match kind {
        Crescendo => es("Crescendo <", "Crescendo <"),
        Diminuendo => es("Diminuendo >", "Diminuendo >"),
        Accent => es("Accent", "Acento"),
        Staccato => es("Staccato", "Staccato"),
        Tenuto => es("Tenuto", "Tenuto"),
        Marcato => es("Marcato", "Marcato"),
        Fermata => es("Fermata", "Calderón"),
        BreathMark => es("Breath mark", "Respiración"),
        Caesura => es("Caesura //", "Cesura //"),
        UpBow => es("Up bow", "Arco arriba"),
        DownBow => es("Down bow", "Arco abajo"),
        PedalUp => es("Pedal release", "Soltar pedal"),
        Glasses => es("Watch (glasses)", "Mirar (gafas)"),
        SlurAbove => es("Slur above", "Ligadura arriba"),
        SlurBelow => es("Slur below", "Ligadura abajo"),
        Number => es("Number", "Número"),
        Segno => es("Segno (S)", "Segno (S)"),
        Coda => es("Coda (O)", "Coda (O)"),
        DalSegno => es("D.S. → segno", "D.S. → segno"),
        ToCoda => es("To Coda → coda", "To Coda → coda"),
        DaCapo => es("D.C. → start", "D.C. → inicio"),
        other => other.label(),
    }
}

/// The palette's sections, in order.
fn sections() -> [(&'static str, &'static [StampKind]); 9] {
    use StampKind::*;
    [
        (es("Dynamics", "Dinámicas"), &[Ppp, Pp, Piano, Mp, Mf, Forte, Ff, Fff, Sfz, Fp]),
        (es("Hairpins and slurs", "Reguladores y ligaduras"), &[Crescendo, Diminuendo, SlurAbove, SlurBelow]),
        (es("Tempo", "Tempo"), &[Cresc, Dim, Rit, Accel, ATempo]),
        (es("Articulation", "Articulación"), &[Accent, Staccato, Tenuto, Marcato, Fermata, BreathMark, Caesura]),
        (es("Bowing and pedal", "Arco y pedal"), &[UpBow, DownBow, PedalDown, PedalUp, Glasses]),
        (es("Fingering", "Digitación"), &[Finger1, Finger2, Finger3, Finger4, Finger5]),
        (
            es("Rehearsal letters", "Letras de ensayo"),
            &[RehearsalA, RehearsalB, RehearsalC, RehearsalD, RehearsalE, RehearsalF, RehearsalG, RehearsalH],
        ),
        (es("Numbers", "Números"), &[Number]),
        (
            es("Jumps: tap one on the score to go to the other", "Saltos: toca uno en la partitura para ir al otro"),
            &[JumpFrom, JumpTo, Segno, DalSegno, Coda, ToCoda, DaCapo, Fine],
        ),
    ]
}

/// The palette; `touch` makes the buttons finger-sized.
pub(crate) fn palette(app: &mut PdfCraftApp, ui: &mut egui::Ui, touch: bool) {
    let size = if touch { 22.0 } else { 13.0 };
    let min = if touch { vec2(56.0, 48.0) } else { vec2(0.0, 0.0) };
    if touch {
        ui.spacing_mut().item_spacing = vec2(10.0, 10.0);
    }
    for (title, kinds) in sections() {
        ui.label(RichText::new(title).strong().size(if touch { 16.0 } else { 12.0 }));
        ui.horizontal_wrapped(|ui| {
            if kinds.contains(&StampKind::JumpFrom) {
                jump_letters(ui, size, min);
            }
            for &kind in kinds {
                let active = app.quick_tool == QuickTool::Stamp(kind);
                let letter = jump_letter();
                let label = match kind {
                    StampKind::JumpFrom => format!("→ {letter}  {}", es("(jump)", "(salto)")),
                    StampKind::JumpTo => format!("{letter}  {}", es("(lands here)", "(destino)")),
                    _ => name(kind).to_owned(),
                };
                let mut text = RichText::new(label).size(size);
                if kind.music_face() == Some(pdfcraft_fonts::Std14::TimesBoldItalic) || kind.music_face() == Some(pdfcraft_fonts::Std14::TimesItalic)
                {
                    text = text.italics().strong();
                }
                if ui.add(egui::Button::new(text).selected(active).min_size(min)).clicked() {
                    app.quick_tool = QuickTool::Stamp(kind);
                }
            }
            if kinds.contains(&StampKind::Number) {
                number_controls(ui, size, min);
            }
        });
        ui.add_space(if touch { 6.0 } else { 2.0 });
    }
}

/// The named jumps' letters: the one chosen is the next jump's.
fn jump_letters(ui: &mut egui::Ui, size: f32, min: egui::Vec2) {
    ui.label(RichText::new(es("Letter:", "Letra:")).size(size * 0.75));
    let chosen = usize::from(JUMP_LETTER.load(Ordering::Relaxed));
    for (i, l) in JUMP_LETTERS.iter().enumerate() {
        if ui.add(egui::Button::new(RichText::new(*l).size(size).strong()).selected(i == chosen).min_size(min * vec2(0.8, 1.0))).clicked() {
            JUMP_LETTER.store(u8::try_from(i).unwrap_or(0), Ordering::Relaxed);
        }
    }
    ui.end_row();
}

/// The next number, − and + to change it, and whether it counts up.
fn number_controls(ui: &mut egui::Ui, size: f32, min: egui::Vec2) {
    let mut n = NUMBER.load(Ordering::Relaxed);
    if ui.add(egui::Button::new(RichText::new("−").size(size)).min_size(min)).clicked() {
        n = n.saturating_sub(1);
    }
    ui.add(egui::DragValue::new(&mut n).range(0..=9999).speed(0.2));
    if ui.add(egui::Button::new(RichText::new("+").size(size)).min_size(min)).clicked() {
        n = n.saturating_add(1).min(9999);
    }
    NUMBER.store(n, Ordering::Relaxed);
    let mut up = COUNT_UP.load(Ordering::Relaxed);
    ui.add(crate::widgets::checkbox(&mut up, RichText::new(es("Count up", "Contar +1")).size(size * 0.75)));
    COUNT_UP.store(up, Ordering::Relaxed);
}

/// Set when a tap followed a jump mark this frame, so the reading view doesn't also take the tap.
static JUMPED: AtomicBool = AtomicBool::new(false);

/// A tap followed a jump mark.
pub(crate) fn note_jump() {
    JUMPED.store(true, Ordering::Relaxed);
}

/// Whether a tap followed a jump mark since the last call.
pub(crate) fn take_jumped() -> bool {
    JUMPED.swap(false, Ordering::Relaxed)
}

/// Which end of a jump a mark is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum End {
    /// Where the player leaves: "→ A", D.S., To Coda.
    From,
    /// Where they land: "A", the segno, the coda.
    To,
    /// D.C.: back to the start.
    Start,
}

/// The jump a stamp is part of, by the text placing one gives it ([`jump_contents`]): which
/// jump (a named jump's letter, "segno", "coda") and which end.
fn jump_key(a: &pdfcraft_render::Annotation) -> Option<(String, End)> {
    if a.subtype != "Stamp" {
        return None;
    }
    let text = a.contents.as_deref()?;
    if let Some(l) = text.strip_prefix("→ ") {
        return Some((format!("jump {l}"), End::From));
    }
    if let Some(l) = text.strip_prefix("◎ ") {
        return Some((format!("jump {l}"), End::To));
    }
    use StampKind::*;
    let kind = [Segno, DalSegno, Coda, ToCoda, DaCapo].into_iter().find(|k| k.label() == text)?;
    Some(match kind {
        Segno => ("segno".into(), End::To),
        DalSegno => ("segno".into(), End::From),
        Coda => ("coda".into(), End::To),
        ToCoda => ("coda".into(), End::From),
        _ => ("start".into(), End::Start),
    })
}

/// Where a tap at `at` (PDF user space) on page `page` goes when it lands on a jump mark: from
/// where a jump leaves, the next place it lands (D.S. to the segno, "→ A" to "A"), and from
/// where it lands back to the next place that leaves for it, both in reading order and round to
/// the start; D.C. goes to the first page. As a page and how far down it the view's top should be
/// (0 to 1). `None` off a jump mark, or with nothing to go to.
pub(crate) fn jump_target(info: &pdfcraft_render::DocInfo, page: usize, at: [f32; 2]) -> Option<(usize, f32)> {
    // A little slack around the mark: a fingertip is wider than a segno.
    const SLACK: f32 = 6.0;
    let inside = |r: [f32; 4]| at[0] >= r[0] - SLACK && at[0] <= r[2] + SLACK && at[1] >= r[1] - SLACK && at[1] <= r[3] + SLACK;
    let tapped = info.annotations.iter().find(|a| a.page == page && inside(a.rect) && jump_key(a).is_some())?;
    let (key, end) = jump_key(tapped)?;
    let want = match end {
        End::Start => return Some((0, 0.0)),
        End::From => End::To,
        End::To => End::From,
    };
    // The jump's marks in reading order: by page, then down it (view space has y down).
    let top = |a: &pdfcraft_render::Annotation| {
        let p = info.pages.get(a.page)?;
        let v = p.user_to_view(a.rect[0], a.rect[3]);
        let v2 = p.user_to_view(a.rect[2], a.rect[1]);
        Some((v[1].min(v2[1]), p.height))
    };
    let mut marks: Vec<(usize, f32, f32, usize, End)> = info
        .annotations
        .iter()
        .filter_map(|a| jump_key(a).filter(|(k, _)| *k == key).and_then(|(_, e)| top(a).map(|(y, h)| (a.page, y, h, a.index, e))))
        .collect();
    marks.sort_by(|a, b| (a.0, a.1).partial_cmp(&(b.0, b.1)).unwrap_or(std::cmp::Ordering::Equal));
    let at_mark = marks.iter().position(|m| m.0 == tapped.page && m.3 == tapped.index)?;
    let n = marks.len();
    let &(p, y, h, _, _) = (1..n).filter_map(|k| marks.get((at_mark + k) % n)).find(|m| m.4 == want)?;
    // A little of the music above the mark stays in view.
    let frac = if h > 0.0 { ((y - 60.0) / h).clamp(0.0, 1.0) } else { 0.0 };
    Some((p, frac))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A blank 200 × 300 pt page.
    fn one_page_pdf() -> Vec<u8> {
        let objs = [
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Kids [3 0 R] /Count 1 /MediaBox [0 0 200 300] >>",
            "<< /Type /Page /Parent 2 0 R >>",
        ];
        let mut out = b"%PDF-1.7
"
        .to_vec();
        let mut offsets = Vec::new();
        for (i, o) in objs.iter().enumerate() {
            offsets.push(out.len());
            out.extend_from_slice(
                format!(
                    "{} 0 obj
{o}
endobj
",
                    i + 1
                )
                .as_bytes(),
            );
        }
        let xref = out.len();
        out.extend_from_slice(
            format!(
                "xref
0 {}
0000000000 65535 f 
",
                objs.len() + 1
            )
            .as_bytes(),
        );
        for o in offsets {
            out.extend_from_slice(
                format!(
                    "{o:010} 00000 n 
"
                )
                .as_bytes(),
            );
        }
        out.extend_from_slice(
            format!(
                "trailer
<< /Size {} /Root 1 0 R >>
startxref
{xref}
%%EOF
",
                objs.len() + 1
            )
            .as_bytes(),
        );
        out
    }

    #[test]
    fn a_tap_on_a_jump_mark_goes_to_the_other() {
        use pdfcraft_engine::{Edit, NewAnnotation, Shape, Style};
        let mut app = PdfCraftApp::new();
        app.open_bytes("a.pdf", None, one_page_pdf()).unwrap();
        let id = app.views[0].id;
        // A segno near the top, a D.S. near the bottom, and a forte that isn't a jump.
        for (kind, rect) in [
            (StampKind::Segno, [20.0, 250.0, 40.0, 276.0]),
            (StampKind::DalSegno, [100.0, 20.0, 140.0, 36.0]),
            (StampKind::Forte, [100.0, 150.0, 120.0, 170.0]),
        ] {
            let shape = Shape::Stamp { rect, stamp: kind, by: None };
            let contents = jump_contents(kind, None);
            let add = NewAnnotation { page: 0, style: Style::default_for(&shape), shape, contents, author: String::new() };
            app.session.apply(id, Edit::AddAnnotation(add)).unwrap();
        }
        let info = &app.session.get(id).unwrap().info;
        // From the D.S. to the segno: the page's top (the segno is 24 pt down, less the margin).
        assert_eq!(jump_target(info, 0, [120.0, 28.0]), Some((0, 0.0)));
        // And back: the D.S. is 264 pt down a 300 pt page; the view's top is 60 pt above it.
        let (page, frac) = jump_target(info, 0, [30.0, 260.0]).unwrap();
        assert_eq!(page, 0);
        assert!((frac - 204.0 / 300.0).abs() < 0.01, "{frac}");
        // Not on a jump mark: no jump.
        assert_eq!(jump_target(info, 0, [110.0, 160.0]), None);
        assert_eq!(jump_target(info, 0, [180.0, 100.0]), None);
    }

    #[test]
    fn named_jumps_go_from_their_start_to_their_end_and_back() {
        use pdfcraft_engine::{Edit, NewAnnotation, Shape, Style};
        let mut app = PdfCraftApp::new();
        app.open_bytes("a.pdf", None, one_page_pdf()).unwrap();
        let id = app.views[0].id;
        // "→ A" low on the page, "A" high up; "→ B" in the middle with no "B" yet.
        for (kind, letter, rect) in [
            (StampKind::JumpFrom, "A", [20.0, 20.0, 58.0, 42.0]),
            (StampKind::JumpTo, "A", [150.0, 240.0, 174.0, 264.0]),
            (StampKind::JumpFrom, "B", [20.0, 140.0, 58.0, 162.0]),
        ] {
            let shape = Shape::Stamp { rect, stamp: kind, by: Some(letter.into()) };
            let contents = jump_contents(kind, Some(letter));
            let add = NewAnnotation { page: 0, style: Style::default_for(&shape), shape, contents, author: String::new() };
            app.session.apply(id, Edit::AddAnnotation(add)).unwrap();
        }
        let info = &app.session.get(id).unwrap().info;
        // "→ A" to "A": "A" is 36 pt down, the view's top 60 pt above it (the page's top).
        assert_eq!(jump_target(info, 0, [40.0, 30.0]), Some((0, 0.0)));
        // "A" back to "→ A": 258 pt down, so 198 pt.
        let (_, frac) = jump_target(info, 0, [160.0, 250.0]).unwrap();
        assert!((frac - 198.0 / 300.0).abs() < 0.01, "{frac}");
        // "→ B" has nowhere to land.
        assert_eq!(jump_target(info, 0, [40.0, 150.0]), None);
    }

    #[test]
    fn every_music_mark_is_in_one_section() {
        let mut seen: Vec<StampKind> = sections().iter().flat_map(|(_, k)| k.iter().copied()).collect();
        assert_eq!(seen.len(), StampKind::MUSIC.len());
        seen.dedup();
        assert!(StampKind::MUSIC.iter().all(|k| seen.contains(k)));
    }
}
