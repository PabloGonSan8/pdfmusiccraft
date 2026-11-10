//! Music marks: the dynamics, hairpins, tempo words, articulations and signs a performer writes
//! into a score. Words are set in the standard Times faces (never embedded); every sign is a
//! path drawn here, scaled to the stamp's box, so a resized mark (a long hairpin) stays crisp.

use super::{Dict, Object, Rgb, Stream, ellipse, font_resources, form, literal, n, rg, rg_stroke, win_ansi};
use crate::StampKind;

/// The appearance of music mark `kind` drawn upright in `rect`.
/// `text` is a number mark's number (its `/PCByLine`); other marks draw their label.
pub(super) fn draw(kind: StampKind, rect: [f64; 4], col: Rgb, text: Option<&str>, opacity: f64, mut res: Dict) -> Stream {
    let [x0, y0, x1, y1] = rect;
    let (w, h) = ((x1 - x0).max(0.0), (y1 - y0).max(0.0));
    let (cx, cy) = (x0 + w / 2.0, y0 + h / 2.0);
    // Stroke width follows the mark's smaller side, so a stretched hairpin keeps thin lines.
    let lw = (w.min(h) * 0.1).clamp(0.6, 2.5);
    let mut c = String::new();
    if opacity < 1.0 {
        c.push_str("/GS0 gs\n");
    }
    c.push_str(&format!("{}{}{} w 1 J 1 j\n", rg(col), rg_stroke(col), n(lw)));
    let line = |pts: &[(f64, f64)]| -> String {
        let mut s = String::new();
        for (i, (x, y)) in pts.iter().enumerate() {
            s.push_str(&format!("{} {} {} ", n(*x), n(*y), if i == 0 { "m" } else { "l" }));
        }
        s.push_str("S\n");
        s
    };
    let (l, r, b, t) = (x0 + lw, x1 - lw, y0 + lw, y1 - lw);
    // One stroked Bézier curve from `a` to `z`.
    let curve = |a: (f64, f64), p: (f64, f64), q: (f64, f64), z: (f64, f64)| {
        format!("{} {} m {} {} {} {} {} {} c S\n", n(a.0), n(a.1), n(p.0), n(p.1), n(q.0), n(q.1), n(z.0), n(z.1))
    };
    if let Some(face) = kind.music_face() {
        // The word centred in the box, as large as the box allows.
        let label = match kind {
            StampKind::Number => text.filter(|t| !t.is_empty()).unwrap_or("#"),
            StampKind::JumpFrom | StampKind::JumpTo => text.filter(|t| !t.is_empty()).unwrap_or("A"),
            _ => kind.label(),
        };
        // Fingering and rehearsal letters are upright capitals and digits: centred on their
        // height. Lowercase dynamics and words sit on a baseline that leaves room for p's tail.
        let upright = face == pdfcraft_fonts::Std14::TimesBold;
        let boxed = kind.is_rehearsal() || matches!(kind, StampKind::JumpFrom | StampKind::JumpTo);
        let mut size = if boxed { h * 0.62 } else { h / 1.25 };
        // A jump's letter sits right of its arrow.
        let (tx, room) = if kind == StampKind::JumpFrom { (x0 + w * 0.7, w * 0.5) } else { (cx, w) };
        let tw = face.text_width(label, size);
        if tw > room * 0.92 && tw > 0.0 {
            size *= room * 0.92 / tw;
        }
        let tw = face.text_width(label, size);
        if kind.is_rehearsal() {
            c.push_str(&format!("{} {} {} {} re S\n", n(x0 + lw / 2.0), n(y0 + lw / 2.0), n(w - lw), n(h - lw)));
        }
        if kind == StampKind::JumpTo {
            c.push_str(&ellipse(x0 + lw / 2.0, y0 + lw / 2.0, x1 - lw / 2.0, y1 - lw / 2.0));
            c.push_str("S\n");
        }
        if kind == StampKind::JumpFrom {
            // An arrow pointing on, into the letter.
            let (a0, a1, head) = (x0 + lw, x0 + w * 0.42, h * 0.22);
            c.push_str(&line(&[(a0, cy), (a1, cy)]));
            c.push_str(&line(&[(a1 - head, cy + head), (a1, cy), (a1 - head, cy - head)]));
        }
        let base = if upright { cy - size * 0.33 } else { y0 + h * 0.3 };
        let name = crate::text_style::resource_name(face);
        let mut out = c.into_bytes();
        out.extend(format!("BT\n/{name} {} Tf\n1 0 0 1 {} {} Tm ", n(size), n(tx - tw / 2.0), n(base)).bytes());
        out.extend(literal(&win_ansi(label)));
        out.extend_from_slice(b" Tj\nET\n");
        res.set(b"Font".to_vec(), Object::Dict(font_resources(face)));
        return form(rect, &out, res);
    }
    use StampKind::*;
    match kind {
        Crescendo => c.push_str(&line(&[(r, t), (l, cy), (r, b)])),
        Diminuendo | Accent => c.push_str(&line(&[(l, t), (r, cy), (l, b)])),
        Staccato => {
            let rad = w.min(h) * 0.3;
            c.push_str(&ellipse(cx - rad, cy - rad, cx + rad, cy + rad));
            c.push_str("f\n");
        }
        Tenuto => c.push_str(&format!("{} {} {} {} re f\n", n(l), n(cy - h * 0.12), n(r - l), n(h * 0.24))),
        Marcato => c.push_str(&line(&[(l, b), (cx, t), (r, b)])),
        UpBow => c.push_str(&line(&[(l, t), (cx, b), (r, t)])),
        DownBow => {
            // Two legs under a heavy bar.
            c.push_str(&line(&[(l, b), (l, t)]));
            c.push_str(&line(&[(r, b), (r, t)]));
            c.push_str(&format!("{} {} {} {} re f\n", n(l - lw / 2.0), n(t - h * 0.22), n(r - l + lw), n(h * 0.22 + lw / 2.0)));
        }
        Fermata => {
            // An upper half ellipse over a dot.
            let (rx, ry, base) = ((r - l) / 2.0, (t - b) * 0.85, b + (t - b) * 0.1);
            let k = 0.552_284_75;
            c.push_str(&format!(
                "{} {} m {} {} {} {} {} {} c {} {} {} {} {} {} c S\n",
                n(l),
                n(base),
                n(l),
                n(base + ry * k),
                n(cx - rx * k),
                n(base + ry),
                n(cx),
                n(base + ry),
                n(cx + rx * k),
                n(base + ry),
                n(r),
                n(base + ry * k),
                n(r),
                n(base),
            ));
            let rad = (h * 0.12).max(lw);
            c.push_str(&ellipse(cx - rad, base + rad * 0.2, cx + rad, base + rad * 2.2));
            c.push_str("f\n");
        }
        BreathMark => {
            // A comma: a filled head with a tail curling down and left.
            let rad = (w.min(h) * 0.3).max(lw);
            let (hx, hy) = (cx + w * 0.1, t - rad);
            c.push_str(&ellipse(hx - rad, hy - rad, hx + rad, hy + rad));
            c.push_str("f\n");
            c.push_str(&curve((hx + rad, hy), (hx + rad, b + h * 0.3), (l + w * 0.2, b), (l, b)));
        }
        Caesura => {
            let d = w * 0.35;
            c.push_str(&line(&[(l, b), (l + d, t)]));
            c.push_str(&line(&[(r - d, b), (r, t)]));
        }
        PedalUp => {
            // The release asterisk: a small ring with four strokes through it.
            let rad = w.min(h) * 0.2;
            c.push_str(&ellipse(cx - rad, cy - rad, cx + rad, cy + rad));
            c.push_str("S\n");
            let (hw, hh) = ((r - l) / 2.0, (t - b) / 2.0);
            let d = 0.707;
            for (dx, dy) in [(hw, 0.0), (0.0, hh), (hw * d, hh * d), (hw * d, -hh * d)] {
                c.push_str(&line(&[(cx - dx, cy - dy), (cx - dx * 0.4, cy - dy * 0.4)]));
                c.push_str(&line(&[(cx + dx * 0.4, cy + dy * 0.4), (cx + dx, cy + dy)]));
            }
        }
        Glasses => {
            // "Watch the conductor": two lenses and a bridge.
            let rad = ((r - l) * 0.22).min((t - b) / 2.0);
            let (lx, rx) = (l + rad, r - rad);
            c.push_str(&ellipse(lx - rad, cy - rad, lx + rad, cy + rad));
            c.push_str(&ellipse(rx - rad, cy - rad, rx + rad, cy + rad));
            c.push_str("S\n");
            c.push_str(&curve((lx + rad, cy), (cx - rad * 0.3, cy + rad * 0.6), (cx + rad * 0.3, cy + rad * 0.6), (rx - rad, cy)));
        }
        SlurAbove | SlurBelow => {
            // An engraved slur: a crescent, thick in the middle and pointed at the ends. A
            // Bézier's control points sit a third further out than the peak they make.
            let (base, peak, thick) = if kind == SlurAbove { (b, t, -lw * 1.2) } else { (t, b, lw * 1.2) };
            let (outer, inner) = (base + (peak - base) / 0.75, base + (peak + thick - base) / 0.75);
            let (q1, q3) = (l + (r - l) * 0.25, r - (r - l) * 0.25);
            c.push_str(&format!(
                "{} {} m {} {} {} {} {} {} c {} {} {} {} {} {} c f\n",
                n(l),
                n(base),
                n(q1),
                n(outer),
                n(q3),
                n(outer),
                n(r),
                n(base),
                n(q3),
                n(inner),
                n(q1),
                n(inner),
                n(l),
                n(base),
            ));
        }
        Segno => {
            // An S (two curves), a slash through it from bottom left to top right, and a dot on
            // each side.
            let (sl, sr) = (l + w * 0.2, r - w * 0.2);
            let (top, mid, bot) = (t - h * 0.08, cy, b + h * 0.08);
            c.push_str(&curve((sr, top - h * 0.12), (sr, top + h * 0.06), (sl, top + h * 0.06), (sl, (top + mid) / 2.0)));
            c.push_str(&curve((sl, (top + mid) / 2.0), (sl, mid), (sr, mid), (sr, (mid + bot) / 2.0)));
            c.push_str(&curve((sr, (mid + bot) / 2.0), (sr, bot - h * 0.06), (sl, bot - h * 0.06), (sl, bot + h * 0.12)));
            c.push_str(&line(&[(l, b + h * 0.15), (r, t - h * 0.15)]));
            let rad = (w * 0.09).max(lw * 0.6);
            for (dx, dy) in [(l + rad, mid - h * 0.12), (r - rad, mid + h * 0.12)] {
                c.push_str(&ellipse(dx - rad, dy - rad, dx + rad, dy + rad));
                c.push_str("f\n");
            }
        }
        Coda => {
            // A ring with a cross through it, the arms reaching past the ring.
            let (rx, ry) = ((r - l) * 0.32, (t - b) * 0.36);
            c.push_str(&ellipse(cx - rx, cy - ry, cx + rx, cy + ry));
            c.push_str("S\n");
            c.push_str(&line(&[(l, cy), (r, cy)]));
            c.push_str(&line(&[(cx, b), (cx, t)]));
        }
        // Every other mark is a word, drawn above.
        _ => {}
    }
    form(rect, c.as_bytes(), res)
}
