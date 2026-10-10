//! Design tokens (colours, sizes, fonts). Air edition: the GROOVEHAUS "Darkroom" design system.
//! Obsidian canvas, bone type, a single sun-yellow accent, hairline rules, square corners, no
//! gloss. Three type voices: Rajdhani (system/UI), Bebas Neue (display), Cormorant Garamond
//! italic (editorial). Source palette: bone #F2E9D5 at 55 % / 50 % / 30 % / 25 % / 18 % over black.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, Visuals};

pub const FONT_SEMIBOLD: &str = "semibold";
/// Bebas Neue: headlines, always uppercase.
pub const FONT_DISPLAY: &str = "display";
/// Cormorant Garamond Italic: the brand's human voice (captions, notes).
pub const FONT_EDITORIAL: &str = "editorial";

#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    /// Top bar, side panels, bottom bar, tool strip.
    pub chrome: Color32,
    /// Photo canvas (detail view) and filmstrip.
    pub canvas: Color32,
    /// Grid background behind the cells.
    pub grid_bg: Color32,
    pub cell: Color32,
    pub cell_selected: Color32,
    pub divider: Color32,
    pub inset: Color32,
    pub field: Color32,
    pub field_border: Color32,
    pub button: Color32,
    pub button_border: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub tool_active: Color32,
    pub text: Color32,
    pub text_label: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    /// Icon glyphs in the chrome.
    pub icon: Color32,
    pub track: Color32,
    pub thumb: Color32,
    pub thumb_hover: Color32,
    pub accent: Color32,
    pub star: Color32,
    pub pick: Color32,
    pub reject: Color32,
    /// Cautionary notices (e.g. a raw shown from its embedded preview): a muted amber.
    pub caution: Color32,
    pub mask_overlay: Color32,
    // metrics (points)
    pub top_bar_h: f32,
    pub bottom_bar_h: f32,
    pub panel_w: f32,
    pub strip_w: f32,
    pub slider_row_h: f32,
    pub section_h: f32,
    pub film_h: f32,
}

impl Default for Tokens {
    fn default() -> Self {
        let c = |r: u8, g: u8, b: u8| Color32::from_rgb(r, g, b);
        Tokens {
            chrome: c(0x00, 0x00, 0x00),
            canvas: c(0x00, 0x00, 0x00),
            grid_bg: c(0x00, 0x00, 0x00),
            cell: c(0x1c, 0x1c, 0x1c),          // ash
            cell_selected: c(0x2c, 0x2a, 0x26), // bone @ 18 %: selection must differ from `cell`
            divider: c(0x2c, 0x2a, 0x26),       // bone @ 18 %
            inset: c(0x0b, 0x0b, 0x0a),
            field: c(0x00, 0x00, 0x00),
            field_border: c(0x3d, 0x3a, 0x35), // bone @ 25 %
            button: c(0x00, 0x00, 0x00),
            button_border: c(0x3d, 0x3a, 0x35),
            hover: c(0x1c, 0x1c, 0x1c),
            pressed: c(0x26, 0x25, 0x20),
            tool_active: c(0x1c, 0x1c, 0x1c),
            text: c(0xf2, 0xe9, 0xd5),          // bone
            text_label: c(0x85, 0x80, 0x75),    // bone @ 55 %
            text_dim: c(0x79, 0x74, 0x6a),      // bone @ 50 %: 4.5:1 on black (the mockup's 30 % is 2.4:1)
            text_disabled: c(0x3d, 0x3a, 0x35), // bone @ 25 %
            icon: c(0x85, 0x80, 0x75),
            track: c(0x49, 0x46, 0x40), // bone @ 30 %
            thumb: c(0xf2, 0xe9, 0xd5),
            thumb_hover: c(0xff, 0xf7, 0xe0),
            accent: c(0xe9, 0xb5, 0x47), // sun: one per frame
            star: c(0xf2, 0xe9, 0xd5),
            pick: c(0xf2, 0xe9, 0xd5),
            reject: c(0xdc, 0x38, 0x2f), // flame
            caution: c(0xe9, 0xb5, 0x47),
            mask_overlay: Color32::from_rgba_unmultiplied(0xdc, 0x38, 0x2f, 110),
            top_bar_h: 52.0,
            bottom_bar_h: 52.0,
            panel_w: 340.0,
            strip_w: 56.0,
            slider_row_h: 45.0,
            section_h: 52.0,
            film_h: 112.0,
        }
    }
}

impl Tokens {
    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_default()
    }
    pub fn font(&self, size: f32) -> FontId {
        FontId::proportional(size)
    }
    pub fn semibold(&self, size: f32) -> FontId {
        FontId::new(size, FontFamily::Name(FONT_SEMIBOLD.into()))
    }
    pub fn display(&self, size: f32) -> FontId {
        FontId::new(size, FontFamily::Name(FONT_DISPLAY.into()))
    }
    pub fn editorial(&self, size: f32) -> FontId {
        FontId::new(size, FontFamily::Name(FONT_EDITORIAL.into()))
    }
}

pub fn install_fonts(ctx: &egui::Context) {
    ctx.set_fonts(font_definitions(lightcraft_engine::CRAFT_FONTS));
}

/// Inter (bundled) for Latin text, egui's default fonts, then the craft-fonts CJK faces as the last
/// fallback of every family — the active language's own script first, so shared Han characters keep
/// that language's forms. Without craft-fonts (`craft` empty) CJK text has no glyphs and shows as
/// boxes.
pub fn font_definitions(craft: &'static [lightcraft_engine::CraftFont]) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("Inter".into(), Arc::new(FontData::from_static(include_bytes!("../../../assets/fonts/Inter-Regular.ttf"))));
    fonts.font_data.insert("Inter-SemiBold".into(), Arc::new(FontData::from_static(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf"))));
    // GROOVEHAUS brand fonts (OFL). Inter stays behind them for glyphs they lack.
    fonts.font_data.insert("Rajdhani".into(), Arc::new(FontData::from_static(include_bytes!("../../../assets/fonts/Rajdhani-Medium.ttf"))));
    fonts
        .font_data
        .insert("Rajdhani-SemiBold".into(), Arc::new(FontData::from_static(include_bytes!("../../../assets/fonts/Rajdhani-SemiBold.ttf"))));
    fonts.font_data.insert("Bebas".into(), Arc::new(FontData::from_static(include_bytes!("../../../assets/fonts/BebasNeue-Regular.ttf"))));
    fonts
        .font_data
        .insert("Cormorant-Italic".into(), Arc::new(FontData::from_static(include_bytes!("../../../assets/fonts/CormorantGaramond-Italic.ttf"))));
    // Craft-fonts faces in preference order for a family drawn in `style`.
    let fallback = |style: &str| {
        lightcraft_engine::fonts::cjk_fallback(craft, crate::i18n::language().script(), style)
            .into_iter()
            .map(craft_font_name)
            .collect::<Vec<String>>()
    };
    let (regular, bold) = (fallback("Regular"), fallback("Bold"));
    for name in regular.iter().chain(bold.iter()) {
        if let Some(font) = craft.iter().find(|font| &craft_font_name(font) == name) {
            fonts.font_data.insert(name.clone(), Arc::new(FontData::from_static(font.bytes)));
        }
    }
    let defaults: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let mut prop = vec!["Rajdhani".to_string(), "Inter".to_string()];
    prop.extend(defaults.iter().cloned());
    prop.extend(regular);
    fonts.families.insert(FontFamily::Proportional, prop.clone());
    let mut semi = vec!["Rajdhani-SemiBold".to_string(), "Inter-SemiBold".to_string()];
    semi.extend(defaults);
    semi.extend(bold);
    let (display_tail, editorial_tail) = (semi.clone(), prop.clone());
    fonts.families.insert(FontFamily::Name(FONT_SEMIBOLD.into()), semi);
    let mut display = vec!["Bebas".to_string()];
    display.extend(display_tail);
    fonts.families.insert(FontFamily::Name(FONT_DISPLAY.into()), display);
    let mut editorial = vec!["Cormorant-Italic".to_string()];
    editorial.extend(editorial_tail);
    fonts.families.insert(FontFamily::Name(FONT_EDITORIAL.into()), editorial);
    fonts.families.entry(FontFamily::Monospace).or_default().extend(fallback("Regular"));
    fonts
}

/// The embedded font families for the About box: Inter, plus the craft-fonts families when built
/// with them.
pub fn font_credits() -> String {
    let mut families = vec!["Rajdhani", "Bebas Neue", "Cormorant Garamond", "Inter"];
    for f in lightcraft_engine::CRAFT_FONTS {
        if !families.contains(&f.family) {
            families.push(f.family);
        }
    }
    families.join(" / ")
}

fn craft_font_name(f: &lightcraft_engine::CraftFont) -> String {
    format!("craft-fonts {} {}", f.family, f.style)
}

pub fn apply(ctx: &egui::Context) {
    let t = Tokens::default();
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, t));
    let mut v = Visuals::dark();
    v.panel_fill = t.chrome;
    v.window_fill = t.chrome;
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.inset;
    v.window_stroke = Stroke::new(1.0, t.button_border);
    // severe: radii are essentially zero (design system: chips and inputs 2 px at most)
    v.window_corner_radius = CornerRadius::same(2);
    v.menu_corner_radius = CornerRadius::same(2);
    v.selection.bg_fill = t.accent.gamma_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.override_text_color = Some(t.text_label);
    // matte: shadows only on floating overlays
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 24, spread: 0, color: Color32::from_black_alpha(140) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 24, spread: 0, color: Color32::from_black_alpha(140) };
    let w = &mut v.widgets;
    for (wv, fill, stroke_c) in [
        (&mut w.noninteractive, t.chrome, t.divider),
        (&mut w.inactive, t.button, t.button_border),
        (&mut w.hovered, t.hover, t.text_label),
        (&mut w.active, t.pressed, t.text),
        (&mut w.open, t.hover, t.text_label),
    ] {
        wv.bg_fill = fill;
        wv.weak_bg_fill = fill;
        wv.corner_radius = CornerRadius::same(2);
        wv.fg_stroke = Stroke::new(1.0, t.text_label);
        wv.bg_stroke = Stroke::new(1.0, stroke_c);
    }
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(8.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        // Rajdhani runs small for its point size: one point up from the Inter-era sizes
        s.text_styles.insert(egui::TextStyle::Body, FontId::proportional(14.0));
        s.text_styles.insert(egui::TextStyle::Button, FontId::proportional(14.0));
        s.text_styles.insert(egui::TextStyle::Small, FontId::proportional(12.0));
        s.text_styles.insert(egui::TextStyle::Heading, FontId::new(18.0, FontFamily::Name(FONT_SEMIBOLD.into())));
        // matte motion: things settle, they don't bounce (design system: 140 ms)
        s.animation_time = 0.14;
    });
}

#[cfg(test)]
mod tests {
    use super::Tokens;

    /// WCAG relative luminance of an sRGB colour.
    fn luminance(c: egui::Color32) -> f32 {
        let lin = |v: u8| {
            let v = f32::from(v) / 255.0;
            if v <= 0.039_28 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
    }

    fn contrast(a: egui::Color32, b: egui::Color32) -> f32 {
        let (la, lb) = (luminance(a), luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    }

    #[test]
    fn readable_text_meets_wcag_aa_on_the_chrome() {
        let t = Tokens::default();
        for (name, c) in [("text", t.text), ("text_label", t.text_label), ("text_dim", t.text_dim), ("accent", t.accent)] {
            let r = contrast(c, t.chrome);
            assert!(r >= 4.5, "{name} is {r:.2}:1 on the chrome");
        }
    }

    #[test]
    fn selected_cells_differ_from_unselected_ones() {
        let t = Tokens::default();
        assert_ne!(t.cell, t.cell_selected);
        assert_ne!(t.canvas, t.cell_selected, "filmstrip selection is drawn on the canvas");
    }
}
