"""Draws PdfMusicCraft's launcher icon (contributor-original): a white eighth-note pair on a
rounded deep-blue square, with a folded page corner. Writes res/mipmap-*/ic_launcher.png.

    python apps/pdfcraft-android/make_icon.py
"""
import os
from PIL import Image, ImageDraw

HERE = os.path.dirname(os.path.abspath(__file__))
SIZES = {"mdpi": 48, "hdpi": 72, "xhdpi": 96, "xxhdpi": 144, "xxxhdpi": 192}


def draw(n):
    s = 4  # supersample, then shrink for smooth edges
    big = n * s
    img = Image.new("RGBA", (big, big), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    r = big * 0.22
    d.rounded_rectangle([0, 0, big - 1, big - 1], radius=r, fill=(28, 52, 120, 255))
    # A page with its corner folded.
    m = big * 0.2
    page = [(m, m * 0.8), (big - m * 1.6, m * 0.8), (big - m, m * 1.4), (big - m, big - m * 0.8), (m, big - m * 0.8)]
    d.polygon(page, fill=(250, 248, 240, 255))
    d.polygon([(big - m * 1.6, m * 0.8), (big - m * 1.6, m * 1.4), (big - m, m * 1.4)], fill=(205, 210, 225, 255))
    # Two beamed eighth notes.
    ink = (28, 52, 120, 255)
    head_w, head_h = big * 0.13, big * 0.1
    x1, x2 = big * 0.36, big * 0.6
    y1, y2 = big * 0.7, big * 0.64
    stem = big * 0.032
    top1, top2 = big * 0.36, big * 0.30
    for x, y in ((x1, y1), (x2, y2)):
        d.ellipse([x - head_w, y - head_h, x + head_w * 0.2, y + head_h * 0.6], fill=ink)
    d.rectangle([x1 + head_w * 0.2 - stem, top1, x1 + head_w * 0.2, y1], fill=ink)
    d.rectangle([x2 + head_w * 0.2 - stem, top2, x2 + head_w * 0.2, y2], fill=ink)
    beam = big * 0.06
    d.polygon(
        [(x1 + head_w * 0.2 - stem, top1), (x2 + head_w * 0.2, top2), (x2 + head_w * 0.2, top2 + beam), (x1 + head_w * 0.2 - stem, top1 + beam)],
        fill=ink,
    )
    return img.resize((n, n), Image.LANCZOS)


for density, n in SIZES.items():
    out = os.path.join(HERE, "res", f"mipmap-{density}")
    os.makedirs(out, exist_ok=True)
    draw(n).save(os.path.join(out, "ic_launcher.png"))
print("ok")
