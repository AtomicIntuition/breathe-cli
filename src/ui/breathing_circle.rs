use crate::app::App;
use crate::techniques::PhaseName;
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{Canvas, Circle, Line as CanvasLine, Points},
    Frame,
};

/// Render the breathing circle with particles and glow effects
#[allow(dead_code)]
pub fn render_breathing_circle(frame: &mut Frame, app: &App, area: Rect) {
    let tc = app.current_technique().color;
    let base_color = Color::Rgb(tc.r, tc.g, tc.b);
    let scale = app.breath_scale();

    // Calculate center and radius based on available space
    let center_x = 0.0;
    let center_y = 0.0;
    let max_radius = (area.width.min(area.height * 2) as f64 / 4.0).min(25.0);
    let min_radius = max_radius * 0.5;
    let current_radius = min_radius + (max_radius - min_radius) * scale;

    // Get phase-specific colors
    let phase_color = match app.current_phase().name {
        PhaseName::Inhale => Color::Rgb(
            (tc.r as f64 * 0.8 + 50.0) as u8,
            (tc.g as f64 * 0.8 + 50.0) as u8,
            (tc.b as f64 * 0.8 + 50.0) as u8,
        ),
        PhaseName::Hold => Color::Rgb(201, 162, 39),
        PhaseName::Exhale => Color::Rgb(
            (tc.r as f64 * 0.7) as u8,
            (tc.g as f64 * 0.7 + 30.0) as u8,
            (tc.b as f64 * 0.9) as u8,
        ),
        PhaseName::HoldAfterExhale => Color::Rgb(100, 116, 139),
    };

    let canvas = Canvas::default()
        .x_bounds([-50.0, 50.0])
        .y_bounds([-25.0, 25.0])
        .marker(ratatui::symbols::Marker::Braille)
        .paint(move |ctx| {
            // Outer glow rings (3 layers)
            for i in 0..3 {
                let glow_radius = current_radius + (i as f64 + 1.0) * 2.5;
                let alpha = 0.3 - (i as f64 * 0.1);
                let glow_color = Color::Rgb(
                    (tc.r as f64 * alpha + 10.0 * (1.0 - alpha)) as u8,
                    (tc.g as f64 * alpha + 22.0 * (1.0 - alpha)) as u8,
                    (tc.b as f64 * alpha + 40.0 * (1.0 - alpha)) as u8,
                );
                ctx.draw(&Circle {
                    x: center_x,
                    y: center_y,
                    radius: glow_radius,
                    color: glow_color,
                });
            }

            // Main breathing circle
            ctx.draw(&Circle {
                x: center_x,
                y: center_y,
                radius: current_radius,
                color: base_color,
            });

            // Inner circle
            let inner_radius = current_radius * 0.6;
            ctx.draw(&Circle {
                x: center_x,
                y: center_y,
                radius: inner_radius,
                color: phase_color,
            });

            // Core dot
            let core_radius = current_radius * 0.2;
            ctx.draw(&Circle {
                x: center_x,
                y: center_y,
                radius: core_radius,
                color: Color::Rgb(
                    ((tc.r as f64 * 0.5) + 127.0) as u8,
                    ((tc.g as f64 * 0.5) + 127.0) as u8,
                    ((tc.b as f64 * 0.5) + 127.0) as u8,
                ),
            });

            // Orbital rings (rotating visual effect based on time)
            let time = app.session_elapsed().as_secs_f64();
            for i in 0..2 {
                let ring_radius = current_radius * (1.1 + i as f64 * 0.15);
                let rotation = time * (0.3 + i as f64 * 0.1) * if i % 2 == 0 { 1.0 } else { -1.0 };

                // Draw dotted orbital ring
                for j in 0..12 {
                    let angle = rotation + (j as f64 * std::f64::consts::TAU / 12.0);
                    let x = center_x + ring_radius * angle.cos();
                    let y = center_y + ring_radius * 0.5 * angle.sin(); // Squish for terminal aspect ratio
                    ctx.draw(&Points {
                        coords: &[(x, y)],
                        color: Color::Rgb(
                            (tc.r as f64 * 0.4) as u8,
                            (tc.g as f64 * 0.4) as u8,
                            (tc.b as f64 * 0.4) as u8,
                        ),
                    });
                }
            }

            // Particle effects (using new particle system)
            for particle in app.particle_system.iter() {
                let opacity = particle.opacity();
                if opacity > 0.1 {
                    let px = center_x + particle.x * scale;
                    let py = center_y + particle.y * 0.5 * scale; // Squish for aspect ratio

                    ctx.draw(&Points {
                        coords: &[(px, py)],
                        color: Color::Rgb(
                            ((tc.r as f64 * opacity) + (255.0 * (1.0 - opacity) * 0.3)) as u8,
                            ((tc.g as f64 * opacity) + (255.0 * (1.0 - opacity) * 0.3)) as u8,
                            ((tc.b as f64 * opacity) + (255.0 * (1.0 - opacity) * 0.3)) as u8,
                        ),
                    });
                }
            }

            // Breathing direction indicator lines
            if matches!(app.current_phase().name, PhaseName::Inhale | PhaseName::Exhale) {
                let is_inhale = app.current_phase().name == PhaseName::Inhale;
                let progress = app.phase_progress();

                for i in 0..8 {
                    let base_angle = (i as f64 * std::f64::consts::TAU / 8.0) + time * 0.5;
                    let start_r = current_radius * if is_inhale { 1.5 } else { 0.3 };
                    let end_r = current_radius * if is_inhale { 1.0 } else { 1.3 };

                    let lerp_r = start_r + (end_r - start_r) * progress;

                    let x1 = center_x + start_r * base_angle.cos();
                    let y1 = center_y + start_r * 0.5 * base_angle.sin();
                    let x2 = center_x + lerp_r * base_angle.cos();
                    let y2 = center_y + lerp_r * 0.5 * base_angle.sin();

                    ctx.draw(&CanvasLine {
                        x1,
                        y1,
                        x2,
                        y2,
                        color: Color::Rgb(
                            (tc.r as f64 * 0.3) as u8,
                            (tc.g as f64 * 0.3) as u8,
                            (tc.b as f64 * 0.3) as u8,
                        ),
                    });
                }
            }
        });

    frame.render_widget(canvas, area);
}
