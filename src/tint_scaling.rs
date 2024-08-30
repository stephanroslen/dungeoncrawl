use crate::prelude::*;

pub fn tint_scale_calc(dist: Option<f32>, max_dist: f32) -> f32 {
    match dist {
        None => FOREGROUND_MIN,
        Some(dist) => {
            let dist_scale = (max_dist - dist) / max_dist;
            FOREGROUND_MIN + (1.0 - FOREGROUND_MIN) * dist_scale
        }
    }
}

fn tint_rgba_conversion(original: RGBA, scale: f32) -> RGBA {
    RGBA {
        r: original.r * scale,
        g: original.g * scale,
        b: original.b * scale,
        a: original.a,
    }
}

pub fn tint_colorpair_conversion(original: ColorPair, scale: f32) -> ColorPair {
    ColorPair {
        fg: tint_rgba_conversion(original.fg, scale),
        bg: original.bg,
    }
}
