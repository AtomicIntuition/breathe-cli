//! Award-winning breathing visualizer - FULL SCREEN, VIBRANT, UNMISTAKABLE
//!
//! Uses the entire terminal for an immersive breathing experience.
//! Colors are bright and saturated. Elements are large and clear.

use crate::app::App;
use crate::particles::ParticleType;
use crate::techniques::PhaseName;
use crate::theme::{blend_color, with_opacity};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{Canvas, Context, Line as CanvasLine, Points},
    Frame,
};
use std::f64::consts::{PI, TAU};

/// Get vibrant phase colors - much brighter than theme defaults
fn get_vibrant_colors(phase: PhaseName) -> (Color, Color, Color) {
    match phase {
        PhaseName::Inhale => (
            Color::Rgb(80, 180, 255),   // Bright sky blue - primary
            Color::Rgb(150, 220, 255),  // Light cyan - glow
            Color::Rgb(200, 240, 255),  // Near white blue - core
        ),
        PhaseName::Hold => (
            Color::Rgb(255, 200, 50),   // Bright gold - primary
            Color::Rgb(255, 230, 120),  // Light gold - glow
            Color::Rgb(255, 250, 200),  // Near white gold - core
        ),
        PhaseName::Exhale => (
            Color::Rgb(180, 100, 255),  // Vibrant purple - primary
            Color::Rgb(210, 160, 255),  // Light purple - glow
            Color::Rgb(240, 210, 255),  // Near white purple - core
        ),
        PhaseName::HoldAfterExhale => (
            Color::Rgb(120, 160, 200),  // Steel blue - primary (brighter)
            Color::Rgb(160, 195, 225),  // Light steel - glow (brighter)
            Color::Rgb(200, 220, 240),  // Near white steel - core (brighter)
        ),
    }
}

/// Blend colors between phases for smooth transitions
fn blend_vibrant_colors(
    from_phase: PhaseName,
    to_phase: PhaseName,
    t: f64,
) -> (Color, Color, Color) {
    let (from_primary, from_glow, from_core) = get_vibrant_colors(from_phase);
    let (to_primary, to_glow, to_core) = get_vibrant_colors(to_phase);

    (
        blend_color(from_primary, to_primary, t),
        blend_color(from_glow, to_glow, t),
        blend_color(from_core, to_core, t),
    )
}

/// FULL-SCREEN breathing visualizer
pub fn render_breath_visualizer(frame: &mut Frame, app: &App, area: Rect) {
    let scale = app.breath_scale();
    let phase = app.current_phase().name;
    let progress = app.phase_progress();
    let time = app.session_elapsed().as_secs_f64();

    // Get vibrant colors (with transition blending)
    let transition_t = app.phase_transition_progress;
    let (primary, glow, core) = if transition_t < 1.0 {
        if let Some(prev) = get_previous_phase(app) {
            blend_vibrant_colors(prev, phase, transition_t)
        } else {
            get_vibrant_colors(phase)
        }
    } else {
        get_vibrant_colors(phase)
    };

    // Calculate canvas bounds to fill the ENTIRE area
    let aspect = area.width as f64 / (area.height as f64 * 2.0);
    let y_range = 50.0; // Larger coordinate system
    let x_range = y_range * aspect;

    // Rich dark background for high contrast - near black with slight blue tint
    let bg_color = Color::Rgb(5, 8, 15);

    let canvas = Canvas::default()
        .x_bounds([-x_range, x_range])
        .y_bounds([-y_range, y_range])
        .marker(ratatui::symbols::Marker::Braille)
        .background_color(bg_color)
        .paint(move |ctx| {
            // ═══════════════════════════════════════════════════════════════
            // LAYER 1: BACKGROUND GRADIENT FIELD
            // ═══════════════════════════════════════════════════════════════
            draw_background_field(ctx, x_range, y_range, time, scale, primary);

            // ═══════════════════════════════════════════════════════════════
            // LAYER 2: MASSIVE PULSING RINGS (fills most of the screen)
            // ═══════════════════════════════════════════════════════════════
            draw_massive_rings(ctx, x_range, y_range, time, scale, phase, primary, glow);

            // ═══════════════════════════════════════════════════════════════
            // LAYER 3: BREATHING CIRCLE (the main visual)
            // ═══════════════════════════════════════════════════════════════
            draw_breathing_circle(ctx, y_range, time, scale, primary, glow, core);

            // ═══════════════════════════════════════════════════════════════
            // LAYER 4: PHASE-SPECIFIC EFFECTS
            // ═══════════════════════════════════════════════════════════════
            match phase {
                PhaseName::Inhale => draw_inhale_effect(ctx, y_range, progress, time, primary, glow),
                PhaseName::Exhale => draw_exhale_effect(ctx, y_range, progress, time, primary, glow),
                PhaseName::Hold => draw_hold_effect(ctx, y_range, time, primary, glow, core),
                PhaseName::HoldAfterExhale => draw_rest_effect(ctx, y_range, time, primary),
            }

            // ═══════════════════════════════════════════════════════════════
            // LAYER 5: PARTICLE STREAMS
            // ═══════════════════════════════════════════════════════════════
            draw_particle_streams(ctx, app, y_range, primary, glow);

            // ═══════════════════════════════════════════════════════════════
            // LAYER 6: BRIGHT CORE
            // ═══════════════════════════════════════════════════════════════
            draw_bright_core(ctx, y_range, scale, time, core);
        });

    frame.render_widget(canvas, area);
}

/// Get previous phase for transition blending
fn get_previous_phase(app: &App) -> Option<PhaseName> {
    let phases = &app.current_technique().phases;
    if app.current_phase_index == 0 {
        Some(phases.last()?.name)
    } else {
        Some(phases[app.current_phase_index - 1].name)
    }
}

/// Layer 1: Background gradient field with floating orbs
fn draw_background_field(ctx: &mut Context, x_range: f64, y_range: f64, time: f64, scale: f64, primary: Color) {

    // Floating orbs across the entire screen
    for i in 0..60 {
        let seed = i as f64 * 1.618033988749; // Golden ratio
        let base_angle = seed * TAU;
        let orbit_speed = 0.05 + (seed % 0.1);
        let angle = base_angle + time * orbit_speed;

        let radius_factor = 0.5 + (seed % 0.5);
        let radius = y_range * radius_factor;
        let drift = (time * 0.3 + seed).sin() * 5.0;

        let x = angle.cos() * radius * (x_range / y_range) + drift;
        let y = angle.sin() * radius;

        // Twinkle effect - brighter against dark background
        let twinkle = (time * 2.0 + seed * 5.0).sin() * 0.5 + 0.5;
        let orb_color = with_opacity(primary, 0.2 + twinkle * 0.25);

        ctx.draw(&Points {
            coords: &[(x, y)],
            color: orb_color,
        });
    }

    // Horizontal wave bands - more visible against dark background
    for band in 0..5 {
        let band_y = -y_range * 0.8 + band as f64 * y_range * 0.4;
        for i in 0..40 {
            let x = -x_range + i as f64 * (x_range * 2.0 / 40.0);
            let wave = (x * 0.1 + time + band as f64 * 0.5).sin() * 3.0 * scale;
            let opacity = 0.15 + (wave.abs() / 3.0) * 0.12;

            ctx.draw(&Points {
                coords: &[(x, band_y + wave)],
                color: with_opacity(primary, opacity),
            });
        }
    }
}

/// Layer 2: Massive pulsing rings that expand across the screen
fn draw_massive_rings(
    ctx: &mut Context,
    x_range: f64,
    y_range: f64,
    time: f64,
    scale: f64,
    _phase: PhaseName,
    primary: Color,
    glow: Color,
) {
    // Base radius scales with breath (30-70% of screen height)
    let base_radius = y_range * (0.3 + scale * 0.4);

    // Draw 8 expanding rings
    for ring in 0..8 {
        let ring_offset = ring as f64 * 0.4;
        let ring_time = (time * 0.6 + ring_offset) % 4.0;
        let ring_progress = ring_time / 4.0;

        // Ring expands from base radius to edge of screen
        let ring_radius = base_radius + ring_progress * (y_range * 0.7 - base_radius);

        // Fade out as it expands - brighter for dark background
        let opacity = (1.0 - ring_progress).powf(0.5) * 0.7;
        if opacity < 0.08 {
            continue;
        }

        let ring_color = with_opacity(glow, opacity);

        // Draw ring with many points for smooth appearance
        let points_count = 120;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU;
            let wobble = (angle * 6.0 + time * 2.0).sin() * 1.5 * ring_progress;
            let x = angle.cos() * (ring_radius + wobble) * (x_range / y_range);
            let y = angle.sin() * (ring_radius + wobble);

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: ring_color,
            });
        }
    }

    // Inner glow rings (stationary, pulsing) - brighter for dark background
    for ring in 0..4 {
        let pulse = (time * 2.0 + ring as f64 * 0.3).sin() * 0.15 + 0.85;
        let ring_radius = base_radius * (0.85 - ring as f64 * 0.1) * pulse;
        let opacity = 0.5 - ring as f64 * 0.08;

        let ring_color = with_opacity(primary, opacity);

        let points_count = 80;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU;
            let x = angle.cos() * ring_radius * (x_range / y_range);
            let y = angle.sin() * ring_radius;

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: ring_color,
            });
        }
    }
}

/// Layer 3: Main breathing circle with thick borders
fn draw_breathing_circle(
    ctx: &mut Context,
    y_range: f64,
    time: f64,
    scale: f64,
    primary: Color,
    glow: Color,
    core: Color,
) {
    let base_radius = y_range * (0.25 + scale * 0.35);
    let pulse = (time * 2.0).sin() * 0.03 + 1.0;
    let radius = base_radius * pulse;

    // Outer glow (thick, multiple layers) - brighter for dark background
    for layer in 0..6 {
        let layer_radius = radius + layer as f64 * 1.5;
        let opacity = 0.6 - layer as f64 * 0.08;
        let layer_color = with_opacity(glow, opacity);

        let points_count = 100;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU;
            let x = angle.cos() * layer_radius;
            let y = angle.sin() * layer_radius;

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: layer_color,
            });
        }
    }

    // Main circle (thick border) - full brightness for dark background
    for thickness in 0..4 {
        let t_radius = radius - thickness as f64 * 0.8;
        let opacity = 1.0 - thickness as f64 * 0.08;
        let circle_color = with_opacity(primary, opacity);

        let points_count = 100;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU;
            let x = angle.cos() * t_radius;
            let y = angle.sin() * t_radius;

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: circle_color,
            });
        }
    }

    // Inner fill gradient - brighter for dark background
    for layer in 0..8 {
        let fill_radius = radius * (0.7 - layer as f64 * 0.08);
        if fill_radius <= 0.0 {
            continue;
        }

        let opacity = 0.25 + layer as f64 * 0.05;
        let fill_color = with_opacity(core, opacity);

        let points_count = 60;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU;
            let x = angle.cos() * fill_radius;
            let y = angle.sin() * fill_radius;

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: fill_color,
            });
        }
    }
}

/// Layer 4a: Inhale effect - streams flowing inward
fn draw_inhale_effect(ctx: &mut Context, y_range: f64, progress: f64, time: f64, primary: Color, glow: Color) {
    let stream_count = 16;

    for stream in 0..stream_count {
        let base_angle = (stream as f64 / stream_count as f64) * TAU;
        let angle = base_angle + time * 0.2;

        // Multiple particles per stream
        for particle in 0..12 {
            let particle_offset = (particle as f64 * 0.15 + time * 3.0) % 2.0;
            let particle_progress = particle_offset / 2.0;

            // Start from outer edge, flow to center
            let start_dist = y_range * 0.95;
            let end_dist = y_range * 0.2;
            let dist = start_dist - (start_dist - end_dist) * particle_progress;

            if particle_progress > progress {
                continue;
            }

            let x = angle.cos() * dist;
            let y = angle.sin() * dist;

            // Brighter near center - high visibility
            let brightness = 1.0 - particle_progress * 0.3;
            let particle_color = with_opacity(glow, brightness * 0.9);

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: particle_color,
            });

            // Trail behind particle - brighter trails
            for trail in 1..4 {
                let trail_dist = dist + trail as f64 * 3.0;
                if trail_dist < start_dist {
                    let trail_x = angle.cos() * trail_dist;
                    let trail_y = angle.sin() * trail_dist;
                    let trail_opacity = brightness * 0.5 * (1.0 - trail as f64 * 0.2);

                    ctx.draw(&Points {
                        coords: &[(trail_x, trail_y)],
                        color: with_opacity(primary, trail_opacity),
                    });
                }
            }
        }
    }

    // Arrows pointing inward
    for arrow in 0..8 {
        let angle = (arrow as f64 / 8.0) * TAU + time * 0.3;
        let arrow_dist = y_range * (0.5 + progress * 0.2);

        let tip_x = angle.cos() * (arrow_dist - 3.0);
        let tip_y = angle.sin() * (arrow_dist - 3.0);
        let base_x = angle.cos() * (arrow_dist + 3.0);
        let base_y = angle.sin() * (arrow_dist + 3.0);

        ctx.draw(&CanvasLine {
            x1: base_x,
            y1: base_y,
            x2: tip_x,
            y2: tip_y,
            color: with_opacity(glow, 0.85),
        });
    }
}

/// Layer 4b: Exhale effect - mist dispersing outward
fn draw_exhale_effect(ctx: &mut Context, y_range: f64, progress: f64, time: f64, primary: Color, glow: Color) {
    let stream_count = 24;

    for stream in 0..stream_count {
        let base_angle = (stream as f64 / stream_count as f64) * TAU;
        let angle_spread = (time * 0.5 + stream as f64).sin() * 0.15;
        let angle = base_angle + angle_spread;

        for particle in 0..10 {
            let particle_offset = (particle as f64 * 0.18 + time * 2.0) % 2.5;
            let particle_progress = particle_offset / 2.5;

            if particle_progress > progress {
                continue;
            }

            // Start from center, disperse outward
            let start_dist = y_range * 0.25;
            let end_dist = y_range * 0.9;
            let dist = start_dist + (end_dist - start_dist) * particle_progress;

            // Add drift/wobble
            let drift = (time * 3.0 + particle as f64 * 0.5).sin() * 2.0 * particle_progress;
            let drift_angle = angle + drift * 0.05;

            let x = drift_angle.cos() * dist;
            let y = drift_angle.sin() * dist;

            // Fade out as it disperses - brighter for visibility
            let opacity = (1.0 - particle_progress) * 0.85;
            let particle_color = with_opacity(glow, opacity);

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: particle_color,
            });

            // Mist trail - brighter for visibility
            for trail in 1..3 {
                let trail_dist = dist - trail as f64 * 2.5;
                if trail_dist > start_dist {
                    let trail_x = drift_angle.cos() * trail_dist;
                    let trail_y = drift_angle.sin() * trail_dist;
                    let trail_opacity = opacity * 0.6 * (1.0 - trail as f64 * 0.25);

                    ctx.draw(&Points {
                        coords: &[(trail_x, trail_y)],
                        color: with_opacity(primary, trail_opacity),
                    });
                }
            }
        }
    }
}

/// Layer 4c: Hold effect - energy orbiting and pulsing
fn draw_hold_effect(ctx: &mut Context, y_range: f64, time: f64, _primary: Color, glow: Color, core: Color) {
    // Orbiting energy balls
    for orbit in 0..3 {
        let orbit_radius = y_range * (0.4 + orbit as f64 * 0.12);
        let orbit_speed = 1.5 - orbit as f64 * 0.3;
        let ball_count = 6 + orbit * 2;

        for ball in 0..ball_count {
            let angle = (ball as f64 / ball_count as f64) * TAU + time * orbit_speed;
            let x = angle.cos() * orbit_radius;
            let y = angle.sin() * orbit_radius;

            // Energy ball with glow
            ctx.draw(&Points {
                coords: &[(x, y)],
                color: core,
            });

            // Glow around ball - brighter
            for glow_layer in 1..3 {
                let glow_offset = glow_layer as f64 * 1.2;
                ctx.draw(&Points {
                    coords: &[
                        (x + glow_offset, y),
                        (x - glow_offset, y),
                        (x, y + glow_offset),
                        (x, y - glow_offset),
                    ],
                    color: with_opacity(glow, 0.6 - glow_layer as f64 * 0.15),
                });
            }
        }
    }

    // Pulsing energy waves - brighter
    let pulse_count = 3;
    for pulse in 0..pulse_count {
        let pulse_time = (time * 2.0 + pulse as f64 * 1.0) % 2.0;
        let pulse_radius = y_range * 0.3 + pulse_time * y_range * 0.25;
        let opacity = (1.0 - pulse_time / 2.0) * 0.6;

        let points_count = 60;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU;
            let wobble = (angle * 8.0 + time * 4.0).sin() * 2.0;
            let x = angle.cos() * (pulse_radius + wobble);
            let y = angle.sin() * (pulse_radius + wobble);

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: with_opacity(glow, opacity),
            });
        }
    }
}

/// Layer 4d: Rest effect - calm, subtle anticipation
fn draw_rest_effect(ctx: &mut Context, y_range: f64, time: f64, primary: Color) {
    // Gentle breathing dots
    let anticipation = (time * 1.2).sin() * 0.08 + 1.0;
    let radius = y_range * 0.2 * anticipation;

    // Subtle pulsing ring
    let points_count = 40;
    for i in 0..points_count {
        let angle = (i as f64 / points_count as f64) * TAU;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        ctx.draw(&Points {
            coords: &[(x, y)],
            color: with_opacity(primary, 0.6),
        });
    }

    // Waiting indicator dots
    for dot in 0..3 {
        let dot_x = (dot as f64 - 1.0) * 4.0;
        let dot_y = y_range * 0.5;
        let pulse = (time * 2.5 + dot as f64 * 0.3).sin() * 0.3 + 0.7;

        ctx.draw(&Points {
            coords: &[(dot_x, dot_y)],
            color: with_opacity(primary, pulse),
        });
    }
}

/// Layer 5: Particle streams from particle system
fn draw_particle_streams(ctx: &mut Context, app: &App, y_range: f64, primary: Color, glow: Color) {
    for particle in app.particle_system.iter() {
        let opacity = particle.opacity();
        if opacity < 0.1 {
            continue;
        }

        // Scale particles to fill more of the screen
        let scale_factor = y_range / 30.0;
        let px = particle.x * scale_factor;
        let py = particle.y * scale_factor * 0.6; // Aspect ratio adjustment

        // Draw trail - brighter
        for (i, (tx, ty)) in particle.trail.iter().enumerate() {
            let trail_opacity = opacity * (i as f64 / particle.trail.len().max(1) as f64) * 0.7;
            if trail_opacity > 0.08 {
                let trail_x = tx * scale_factor;
                let trail_y = ty * scale_factor * 0.6;
                ctx.draw(&Points {
                    coords: &[(trail_x, trail_y)],
                    color: with_opacity(primary, trail_opacity),
                });
            }
        }

        // Draw particle
        let particle_color = match particle.particle_type {
            ParticleType::Inward => glow,
            ParticleType::Outward => with_opacity(glow, opacity * 0.8),
            ParticleType::Orbital => primary,
            _ => with_opacity(glow, opacity),
        };

        ctx.draw(&Points {
            coords: &[(px, py)],
            color: with_opacity(particle_color, opacity),
        });
    }
}

/// Layer 6: Bright glowing core
fn draw_bright_core(ctx: &mut Context, y_range: f64, scale: f64, time: f64, core: Color) {
    let pulse = (time * 2.5).sin() * 0.1 + 1.0;
    let core_radius = y_range * (0.05 + scale * 0.08) * pulse;

    // Multiple core layers for glow effect
    for layer in 0..8 {
        let layer_radius = core_radius - layer as f64 * (core_radius / 8.0);
        if layer_radius <= 0.0 {
            continue;
        }

        let brightness = 0.3 + (layer as f64 / 8.0) * 0.7;
        let layer_color = with_opacity(core, brightness);

        let points_count = 30;
        for i in 0..points_count {
            let angle = (i as f64 / points_count as f64) * TAU + time * 0.5;
            let x = angle.cos() * layer_radius;
            let y = angle.sin() * layer_radius;

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: layer_color,
            });
        }
    }

    // Bright center point
    ctx.draw(&Points {
        coords: &[(0.0, 0.0)],
        color: Color::Rgb(255, 255, 255),
    });

    // Cross-shaped flare
    let flare_size = core_radius * 1.5;
    for i in 0..4 {
        let angle = i as f64 * PI / 2.0 + time * 0.3;
        for step in 1..6 {
            let dist = step as f64 * (flare_size / 5.0);
            let opacity = 1.0 - step as f64 * 0.18;
            let x = angle.cos() * dist;
            let y = angle.sin() * dist;

            ctx.draw(&Points {
                coords: &[(x, y)],
                color: with_opacity(core, opacity),
            });
        }
    }
}
