//! Animation utilities: easing functions and interpolation helpers

use ratatui::style::Color;
use std::f64::consts::PI;

// ============================================================================
// EASING FUNCTIONS
// ============================================================================

/// Sine easing - smooth start and end (existing, moved here)
pub fn ease_in_out_sine(t: f64) -> f64 {
    -(f64::cos(PI * t) - 1.0) / 2.0
}

/// Cubic easing - smooth acceleration and deceleration
pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Cubic ease out - fast start, slow end
pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Elastic ease out - bouncy overshoot effect
#[allow(dead_code)]
pub fn ease_out_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        let c4 = (2.0 * PI) / 3.0;
        2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
    }
}

/// Custom organic breathing curve - combines sine with slight acceleration
/// Creates a more natural feeling than pure sine
pub fn ease_breath(t: f64) -> f64 {
    // Combine sine with a slight cubic influence for organic feel
    let sine_part = ease_in_out_sine(t);
    let cubic_part = ease_in_out_cubic(t);
    // 70% sine, 30% cubic for natural breathing rhythm
    sine_part * 0.7 + cubic_part * 0.3
}

/// Quadratic ease in - slow start
#[allow(dead_code)]
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Quadratic ease out - slow end
#[allow(dead_code)]
pub fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

// ============================================================================
// INTERPOLATION HELPERS
// ============================================================================

/// Linear interpolation between two values
#[inline]
#[allow(dead_code)]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Linear interpolation between two u8 values
#[inline]
pub fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    let t = t.clamp(0.0, 1.0);
    (a as f64 + (b as f64 - a as f64) * t).round() as u8
}

/// Linear interpolation between two RGB colors
#[allow(dead_code)]
pub fn lerp_color(a: Color, b: Color, t: f64) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            Color::Rgb(
                lerp_u8(r1, r2, t),
                lerp_u8(g1, g2, t),
                lerp_u8(b1, b2, t),
            )
        }
        // If not RGB colors, return the target color
        _ => if t < 0.5 { a } else { b },
    }
}

/// Smooth damp - spring-like smoothing towards target
/// Returns the new current value and updates velocity
///
/// Parameters:
/// - current: current value
/// - target: target value to move towards
/// - velocity: current velocity (mutable, will be updated)
/// - smooth_time: approximate time to reach target (lower = faster)
/// - dt: delta time since last update
pub fn smooth_damp(current: f64, target: f64, velocity: &mut f64, smooth_time: f64, dt: f64) -> f64 {
    // Based on Game Programming Gems 4, critically damped spring
    let smooth_time = smooth_time.max(0.0001);
    let omega = 2.0 / smooth_time;

    let x = omega * dt;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);

    let change = current - target;
    let temp = (*velocity + omega * change) * dt;

    *velocity = (*velocity - omega * temp) * exp;
    let result = target + (change + temp) * exp;

    // Prevent overshooting
    if (target - current > 0.0) == (result > target) {
        *velocity = 0.0;
        target
    } else {
        result
    }
}

/// Smooth damp for angles (handles wrapping)
#[allow(dead_code)]
pub fn smooth_damp_angle(current: f64, target: f64, velocity: &mut f64, smooth_time: f64, dt: f64) -> f64 {
    let two_pi = 2.0 * PI;
    let mut delta = (target - current) % two_pi;
    if delta > PI {
        delta -= two_pi;
    } else if delta < -PI {
        delta += two_pi;
    }
    smooth_damp(current, current + delta, velocity, smooth_time, dt)
}

// ============================================================================
// PULSE AND WAVE FUNCTIONS
// ============================================================================

/// Sine wave pulse (0 to 1 range) based on time
#[allow(dead_code)]
pub fn pulse_sine(time: f64, frequency: f64) -> f64 {
    ((time * frequency * 2.0 * PI).sin() + 1.0) / 2.0
}

/// Triangle wave (0 to 1 range) based on time
#[allow(dead_code)]
pub fn pulse_triangle(time: f64, frequency: f64) -> f64 {
    let t = (time * frequency) % 1.0;
    if t < 0.5 {
        t * 2.0
    } else {
        2.0 - t * 2.0
    }
}

/// Breathing pulse - organic pulsing effect for visuals
pub fn pulse_breath(time: f64, base_freq: f64) -> f64 {
    // Combine multiple sine waves for organic feel
    let primary = (time * base_freq * 2.0 * PI).sin();
    let secondary = (time * base_freq * 0.5 * 2.0 * PI).sin() * 0.3;
    let tertiary = (time * base_freq * 2.3 * 2.0 * PI).sin() * 0.1;

    ((primary + secondary + tertiary) / 1.4 + 1.0) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_bounds() {
        // All easing functions should return 0 at t=0 and 1 at t=1
        assert!((ease_in_out_sine(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_sine(1.0) - 1.0).abs() < 0.001);

        assert!((ease_in_out_cubic(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_cubic(1.0) - 1.0).abs() < 0.001);

        assert!((ease_out_cubic(0.0) - 0.0).abs() < 0.001);
        assert!((ease_out_cubic(1.0) - 1.0).abs() < 0.001);

        assert!((ease_breath(0.0) - 0.0).abs() < 0.001);
        assert!((ease_breath(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < 0.001);
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 0.001);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_lerp_color() {
        let black = Color::Rgb(0, 0, 0);
        let white = Color::Rgb(255, 255, 255);

        let mid = lerp_color(black, white, 0.5);
        if let Color::Rgb(r, g, b) = mid {
            assert!((r as i32 - 128).abs() <= 1);
            assert!((g as i32 - 128).abs() <= 1);
            assert!((b as i32 - 128).abs() <= 1);
        }
    }
}
