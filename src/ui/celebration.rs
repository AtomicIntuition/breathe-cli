//! Celebration animation for session completion

use crate::animation::ease_out_cubic;
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{Canvas, Context, Points},
    Frame,
};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::f64::consts::TAU;

/// A celebration particle for the completion animation
#[derive(Debug, Clone)]
pub struct CelebrationParticle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    pub color: Color,
    #[allow(dead_code)]
    pub size: f64,
    pub trail: Vec<(f64, f64)>,
}

impl CelebrationParticle {
    pub fn new(x: f64, y: f64, angle: f64, speed: f64, color: Color) -> Self {
        let life = 2.0 + rand_f64() * 1.5;
        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life,
            max_life: life,
            color,
            size: 1.0 + rand_f64() * 0.5,
            trail: Vec::with_capacity(6),
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Store trail position
        if self.trail.len() >= 6 {
            self.trail.remove(0);
        }
        self.trail.push((self.x, self.y));

        // Apply gravity
        self.vy -= 15.0 * dt;

        // Apply air resistance
        self.vx *= 0.98;
        self.vy *= 0.99;

        // Update position
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Update life
        self.life -= dt;
    }

    pub fn opacity(&self) -> f64 {
        ease_out_cubic((self.life / self.max_life).clamp(0.0, 1.0))
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}

/// The celebration animation state
pub struct CelebrationAnimation {
    pub particles: Vec<CelebrationParticle>,
    pub progress: f64,
    pub duration: f64,
    center_x: f64,
    center_y: f64,
    burst_complete: bool,
}

impl CelebrationAnimation {
    /// Create a new celebration animation with an initial burst of particles
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(100),
            progress: 0.0,
            duration: 4.0, // 4 second animation
            center_x: 0.0,
            center_y: 0.0,
            burst_complete: false,
        }
    }

    /// Set the center point for the animation
    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center_x = x;
        self.center_y = y;
    }

    /// Spawn the initial burst of particles
    pub fn spawn_burst(&mut self) {
        if self.burst_complete {
            return;
        }

        // Celebration colors - rainbow spectrum plus gold
        let colors = [
            Color::Rgb(255, 215, 0),   // Gold
            Color::Rgb(34, 197, 94),   // Green (success)
            Color::Rgb(74, 144, 217),  // Blue
            Color::Rgb(139, 92, 246),  // Purple
            Color::Rgb(244, 63, 94),   // Rose
            Color::Rgb(251, 146, 60),  // Orange
            Color::Rgb(255, 255, 255), // White sparkle
        ];

        // Spawn 80 particles in a burst pattern
        for i in 0..80 {
            // Distribute evenly around the circle with some randomness
            let base_angle = (i as f64 / 80.0) * TAU;
            let angle = base_angle + (rand_f64() - 0.5) * 0.3;

            // Vary speed for natural feel
            let speed = 15.0 + rand_f64() * 25.0;

            // Pick a celebration color
            let color_idx = i % colors.len();
            let color = colors[color_idx];

            self.particles.push(CelebrationParticle::new(
                self.center_x,
                self.center_y,
                angle,
                speed,
                color,
            ));
        }

        // Add extra "sparkle" particles
        for _ in 0..20 {
            let angle = rand_f64() * TAU;
            let speed = 20.0 + rand_f64() * 15.0;
            self.particles.push(CelebrationParticle::new(
                self.center_x,
                self.center_y,
                angle,
                speed,
                Color::Rgb(255, 255, 255),
            ));
        }

        self.burst_complete = true;
    }

    /// Update the animation
    pub fn tick(&mut self, dt: f64) {
        self.progress += dt;

        // Spawn burst on first tick
        if !self.burst_complete {
            self.spawn_burst();
        }

        // Update all particles
        self.particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= self.duration || (self.burst_complete && self.particles.is_empty())
    }

    /// Render the celebration animation
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Calculate canvas bounds based on area
        let aspect = area.width as f64 / (area.height as f64 * 2.0);
        let y_range = 30.0;
        let x_range = y_range * aspect;

        // Rich dark background matching the visualizer
        let bg_color = Color::Rgb(5, 8, 15);

        let canvas = Canvas::default()
            .x_bounds([-x_range, x_range])
            .y_bounds([-y_range, y_range])
            .marker(ratatui::symbols::Marker::Braille)
            .background_color(bg_color)
            .paint(|ctx| {
                self.render_particles(ctx);
            });

        frame.render_widget(canvas, area);
    }

    fn render_particles(&self, ctx: &mut Context) {
        for particle in &self.particles {
            let opacity = particle.opacity();
            if opacity < 0.05 {
                continue;
            }

            // Render trail first (fading)
            for (i, (tx, ty)) in particle.trail.iter().enumerate() {
                let trail_opacity = opacity * (i as f64 / particle.trail.len() as f64) * 0.5;
                if trail_opacity > 0.05 {
                    let trail_color = apply_opacity(particle.color, trail_opacity);
                    ctx.draw(&Points {
                        coords: &[(*tx, *ty)],
                        color: trail_color,
                    });
                }
            }

            // Render main particle
            let particle_color = apply_opacity(particle.color, opacity);
            ctx.draw(&Points {
                coords: &[(particle.x, particle.y)],
                color: particle_color,
            });
        }
    }
}

impl Default for CelebrationAnimation {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply opacity to a color
fn apply_opacity(color: Color, opacity: f64) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            Color::Rgb(
                (r as f64 * opacity) as u8,
                (g as f64 * opacity) as u8,
                (b as f64 * opacity) as u8,
            )
        }
        _ => color,
    }
}

/// Simple random number generator
fn rand_f64() -> f64 {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    );
    (hasher.finish() as f64) / (u64::MAX as f64)
}
