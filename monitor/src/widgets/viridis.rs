use lazy_static::lazy_static;
use tui::style::Color;

lazy_static! {
    pub static ref VIRIDIS: Vec<Color> = {
        let n = 256;
        let mut colors = Vec::with_capacity(n);
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            // Formules classiques pour le colormap "jet"
            let r = ((4.0 * x - 1.5).min(-4.0 * x + 4.5)).max(0.0).min(1.0);
            let g = ((4.0 * x - 0.5).min(-4.0 * x + 3.5)).max(0.0).min(1.0);
            let b = ((4.0 * x + 0.5).min(-4.0 * x + 2.5)).max(0.0).min(1.0);
            colors.push(Color::Rgb(
                (r * 255.0).round() as u8,
                (g * 255.0).round() as u8,
                (b * 255.0).round() as u8,
            ));
        }
        colors
    };
}
