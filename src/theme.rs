//! Centralized color and theme system for the breathing visualizer

use crate::animation::lerp_u8;
use crate::techniques::PhaseName;
use ratatui::style::Color;

/// Main theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub background_dark: Color,
    pub phase_colors: PhaseColorScheme,
    pub ui: UiColors,
}

/// Colors for UI elements
#[derive(Debug, Clone)]
pub struct UiColors {
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
}

/// Color scheme for each breathing phase
#[derive(Debug, Clone)]
pub struct PhaseColorScheme {
    pub inhale: PhaseColors,
    pub hold: PhaseColors,
    pub exhale: PhaseColors,
    pub hold_empty: PhaseColors,
}

/// Colors for a single phase
#[derive(Debug, Clone, Copy)]
pub struct PhaseColors {
    pub primary: Color,      // Main color for the phase
    pub glow: Color,         // Outer glow/halo color
    pub text: Color,         // Text label color
    pub particle: Color,     // Particle color
    pub core: Color,         // Inner core glow
    pub ambient: Color,      // Background ambient color
}

impl PhaseColors {
    pub const fn new(
        primary: Color,
        glow: Color,
        text: Color,
        particle: Color,
        core: Color,
        ambient: Color,
    ) -> Self {
        Self {
            primary,
            glow,
            text,
            particle,
            core,
            ambient,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Default dark theme - the main visual style
    pub fn dark() -> Self {
        Self {
            background: Color::Rgb(10, 22, 40),
            background_dark: Color::Rgb(5, 11, 20),
            phase_colors: PhaseColorScheme::default(),
            ui: UiColors {
                text_primary: Color::White,
                text_secondary: Color::Rgb(148, 163, 184),
                text_muted: Color::Rgb(100, 116, 139),
                accent: Color::Rgb(74, 144, 217),
                border: Color::Rgb(30, 41, 59),
                success: Color::Rgb(34, 197, 94),
                warning: Color::Rgb(201, 162, 39),
            },
        }
    }

    /// Get phase colors for a specific phase
    pub fn get_phase_colors(&self, phase: PhaseName) -> &PhaseColors {
        match phase {
            PhaseName::Inhale => &self.phase_colors.inhale,
            PhaseName::Hold => &self.phase_colors.hold,
            PhaseName::Exhale => &self.phase_colors.exhale,
            PhaseName::HoldAfterExhale => &self.phase_colors.hold_empty,
        }
    }
}

impl Default for PhaseColorScheme {
    fn default() -> Self {
        Self {
            // Inhale: Cool blue tones - fresh air, expansion
            inhale: PhaseColors::new(
                Color::Rgb(74, 144, 217),   // Primary: Arctic blue
                Color::Rgb(100, 180, 255),  // Glow: Light sky blue
                Color::Rgb(74, 144, 217),   // Text: Arctic blue
                Color::Rgb(150, 200, 255),  // Particle: Bright blue
                Color::Rgb(180, 220, 255),  // Core: Near white blue
                Color::Rgb(30, 60, 100),    // Ambient: Deep blue
            ),

            // Hold (full): Golden/amber - energy, warmth, power
            hold: PhaseColors::new(
                Color::Rgb(201, 162, 39),   // Primary: Gold
                Color::Rgb(255, 200, 80),   // Glow: Bright gold
                Color::Rgb(201, 162, 39),   // Text: Gold
                Color::Rgb(255, 220, 120),  // Particle: Light gold
                Color::Rgb(255, 240, 180),  // Core: Warm white
                Color::Rgb(80, 60, 20),     // Ambient: Deep gold
            ),

            // Exhale: Purple/violet - release, calm, letting go
            exhale: PhaseColors::new(
                Color::Rgb(139, 92, 246),   // Primary: Purple
                Color::Rgb(180, 140, 255),  // Glow: Light purple
                Color::Rgb(139, 92, 246),   // Text: Purple
                Color::Rgb(200, 170, 255),  // Particle: Soft purple
                Color::Rgb(220, 200, 255),  // Core: Light violet
                Color::Rgb(50, 30, 80),     // Ambient: Deep purple
            ),

            // Hold (empty): Slate/gray - stillness, peace, anticipation
            hold_empty: PhaseColors::new(
                Color::Rgb(100, 116, 139),  // Primary: Slate
                Color::Rgb(140, 160, 180),  // Glow: Light slate
                Color::Rgb(100, 116, 139),  // Text: Slate
                Color::Rgb(160, 180, 200),  // Particle: Light gray
                Color::Rgb(180, 200, 220),  // Core: Near white gray
                Color::Rgb(30, 40, 50),     // Ambient: Deep slate
            ),
        }
    }
}

/// Blend between two phase color sets
pub fn blend_phase_colors(from: &PhaseColors, to: &PhaseColors, t: f64) -> PhaseColors {
    PhaseColors {
        primary: blend_color(from.primary, to.primary, t),
        glow: blend_color(from.glow, to.glow, t),
        text: blend_color(from.text, to.text, t),
        particle: blend_color(from.particle, to.particle, t),
        core: blend_color(from.core, to.core, t),
        ambient: blend_color(from.ambient, to.ambient, t),
    }
}

/// Blend two colors together
pub fn blend_color(from: Color, to: Color, t: f64) -> Color {
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            Color::Rgb(
                lerp_u8(r1, r2, t),
                lerp_u8(g1, g2, t),
                lerp_u8(b1, b2, t),
            )
        }
        _ => if t < 0.5 { from } else { to },
    }
}

/// Apply opacity to a color (multiply RGB by opacity factor)
pub fn with_opacity(color: Color, opacity: f64) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            let opacity = opacity.clamp(0.0, 1.0);
            Color::Rgb(
                (r as f64 * opacity) as u8,
                (g as f64 * opacity) as u8,
                (b as f64 * opacity) as u8,
            )
        }
        _ => color,
    }
}

/// Brighten a color by a factor (1.0 = no change, >1.0 = brighter)
#[allow(dead_code)]
pub fn brighten(color: Color, factor: f64) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            Color::Rgb(
                ((r as f64 * factor).min(255.0)) as u8,
                ((g as f64 * factor).min(255.0)) as u8,
                ((b as f64 * factor).min(255.0)) as u8,
            )
        }
        _ => color,
    }
}

/// Create a color with custom RGB that can be derived from technique color
#[allow(dead_code)]
pub fn technique_to_phase_colors(r: u8, g: u8, b: u8) -> PhaseColors {
    PhaseColors {
        primary: Color::Rgb(r, g, b),
        glow: Color::Rgb(
            ((r as f64 * 1.3).min(255.0)) as u8,
            ((g as f64 * 1.3).min(255.0)) as u8,
            ((b as f64 * 1.3).min(255.0)) as u8,
        ),
        text: Color::Rgb(r, g, b),
        particle: Color::Rgb(
            ((r as f64 * 0.8) + 50.0).min(255.0) as u8,
            ((g as f64 * 0.8) + 70.0).min(255.0) as u8,
            ((b as f64 * 0.8) + 100.0).min(255.0) as u8,
        ),
        core: Color::Rgb(
            ((r as f64 * 0.5) + 128.0).min(255.0) as u8,
            ((g as f64 * 0.5) + 128.0).min(255.0) as u8,
            ((b as f64 * 0.5) + 128.0).min(255.0) as u8,
        ),
        ambient: Color::Rgb(
            (r as f64 * 0.3) as u8,
            (g as f64 * 0.3) as u8,
            (b as f64 * 0.3) as u8,
        ),
    }
}

/// Get the default theme
pub fn default_theme() -> Theme {
    Theme::dark()
}
