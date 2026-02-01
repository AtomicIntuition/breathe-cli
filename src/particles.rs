//! Enhanced particle system with trails and multiple emitter types

#![allow(dead_code)]

use crate::techniques::PhaseName;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

/// Maximum trail length for particles
const MAX_TRAIL_LENGTH: usize = 8;

/// Enhanced particle with trail support
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    pub size: f64,
    pub trail: Vec<(f64, f64)>,  // Position history for comet trails
    pub particle_type: ParticleType,
}

/// Different particle behaviors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    /// Standard floating particle
    Standard,
    /// Flows inward towards center (inhale)
    Inward,
    /// Disperses outward from center (exhale)
    Outward,
    /// Orbits around center (hold)
    Orbital,
    /// Ambient background particle
    Ambient,
    /// Celebration burst particle
    Celebration,
}

impl Particle {
    pub fn new(x: f64, y: f64, angle: f64, speed: f64, life: f64, particle_type: ParticleType) -> Self {
        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life,
            max_life: life,
            size: 1.0,
            trail: Vec::with_capacity(MAX_TRAIL_LENGTH),
            particle_type,
        }
    }

    pub fn new_with_size(x: f64, y: f64, angle: f64, speed: f64, life: f64, size: f64, particle_type: ParticleType) -> Self {
        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life,
            max_life: life,
            size,
            trail: Vec::with_capacity(MAX_TRAIL_LENGTH),
            particle_type,
        }
    }

    /// Update particle position and trail
    pub fn update(&mut self, dt: f64, center_x: f64, center_y: f64) {
        // Store current position in trail
        if self.trail.len() >= MAX_TRAIL_LENGTH {
            self.trail.remove(0);
        }
        self.trail.push((self.x, self.y));

        match self.particle_type {
            ParticleType::Inward => {
                // Accelerate towards center
                let dx = center_x - self.x;
                let dy = center_y - self.y;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                let accel = 15.0 / dist; // Stronger pull as we get further
                self.vx += dx / dist * accel * dt;
                self.vy += dy / dist * accel * dt;
            }
            ParticleType::Outward => {
                // Slight deceleration for mist effect
                self.vx *= 0.98;
                self.vy *= 0.98;
            }
            ParticleType::Orbital => {
                // Orbit around center
                let dx = self.x - center_x;
                let dy = self.y - center_y;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                // Tangential velocity
                let orbital_speed = 2.0;
                self.vx = -dy / dist * orbital_speed;
                self.vy = dx / dist * orbital_speed;
            }
            ParticleType::Celebration => {
                // Gravity effect
                self.vy -= 5.0 * dt;
                self.vx *= 0.99;
            }
            ParticleType::Ambient | ParticleType::Standard => {
                // Gentle drift with slight slowdown
                self.vx *= 0.995;
                self.vy *= 0.995;
            }
        }

        // Update position
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Update life
        self.life -= dt;
    }

    /// Get particle opacity based on remaining life
    pub fn opacity(&self) -> f64 {
        (self.life / self.max_life).clamp(0.0, 1.0)
    }

    /// Check if particle is still alive
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}

/// Emitter shape for spawning particles
#[derive(Debug, Clone, Copy)]
pub enum EmitterShape {
    /// Single point emitter
    Point { x: f64, y: f64 },
    /// Ring emitter (particles spawn on circle)
    Ring { cx: f64, cy: f64, radius: f64 },
    /// Cone emitter (particles spawn in a direction range)
    Cone { x: f64, y: f64, direction: f64, spread: f64 },
}

/// Particle emitter configuration
#[derive(Debug, Clone)]
pub struct Emitter {
    pub shape: EmitterShape,
    pub rate: f64,              // Particles per second
    pub speed_min: f64,
    pub speed_max: f64,
    pub life_min: f64,
    pub life_max: f64,
    pub size_min: f64,
    pub size_max: f64,
    pub particle_type: ParticleType,
    accumulator: f64,          // Time accumulator for emission
}

impl Emitter {
    pub fn new(shape: EmitterShape, rate: f64, particle_type: ParticleType) -> Self {
        Self {
            shape,
            rate,
            speed_min: 2.0,
            speed_max: 5.0,
            life_min: 1.0,
            life_max: 3.0,
            size_min: 0.5,
            size_max: 1.5,
            particle_type,
            accumulator: 0.0,
        }
    }

    pub fn with_speed(mut self, min: f64, max: f64) -> Self {
        self.speed_min = min;
        self.speed_max = max;
        self
    }

    pub fn with_life(mut self, min: f64, max: f64) -> Self {
        self.life_min = min;
        self.life_max = max;
        self
    }

    pub fn with_size(mut self, min: f64, max: f64) -> Self {
        self.size_min = min;
        self.size_max = max;
        self
    }

    /// Emit particles based on elapsed time
    pub fn emit(&mut self, dt: f64) -> Vec<Particle> {
        self.accumulator += dt;
        let emit_interval = 1.0 / self.rate;
        let mut particles = Vec::new();

        while self.accumulator >= emit_interval {
            self.accumulator -= emit_interval;
            if let Some(p) = self.spawn_particle() {
                particles.push(p);
            }
        }

        particles
    }

    fn spawn_particle(&self) -> Option<Particle> {
        let (x, y, angle) = match self.shape {
            EmitterShape::Point { x, y } => {
                let angle = rand_f64() * std::f64::consts::TAU;
                (x, y, angle)
            }
            EmitterShape::Ring { cx, cy, radius } => {
                let angle = rand_f64() * std::f64::consts::TAU;
                let x = cx + angle.cos() * radius;
                let y = cy + angle.sin() * radius;
                // Direction towards center for inward, away for outward
                let dir = match self.particle_type {
                    ParticleType::Inward => angle + std::f64::consts::PI,
                    _ => angle,
                };
                (x, y, dir)
            }
            EmitterShape::Cone { x, y, direction, spread } => {
                let angle = direction + (rand_f64() - 0.5) * spread;
                (x, y, angle)
            }
        };

        let speed = lerp_rand(self.speed_min, self.speed_max);
        let life = lerp_rand(self.life_min, self.life_max);
        let size = lerp_rand(self.size_min, self.size_max);

        Some(Particle::new_with_size(x, y, angle, speed, life, size, self.particle_type))
    }
}

/// Enhanced particle system manager
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
    emitters: Vec<Emitter>,
    center_x: f64,
    center_y: f64,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
            emitters: Vec::new(),
            center_x: 0.0,
            center_y: 0.0,
        }
    }

    /// Set the center point for particle behaviors
    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center_x = x;
        self.center_y = y;
    }

    /// Add an emitter
    pub fn add_emitter(&mut self, emitter: Emitter) {
        self.emitters.push(emitter);
    }

    /// Clear all emitters
    pub fn clear_emitters(&mut self) {
        self.emitters.clear();
    }

    /// Configure emitters for a specific breathing phase
    pub fn configure_for_phase(&mut self, phase: PhaseName, scale: f64) {
        self.clear_emitters();

        match phase {
            PhaseName::Inhale => {
                // Particles flow inward from outer ring
                let outer_radius = 25.0 + scale * 10.0;
                let emitter = Emitter::new(
                    EmitterShape::Ring {
                        cx: self.center_x,
                        cy: self.center_y,
                        radius: outer_radius,
                    },
                    30.0, // particles per second
                    ParticleType::Inward,
                )
                .with_speed(8.0, 15.0)
                .with_life(1.5, 2.5)
                .with_size(0.8, 1.2);
                self.add_emitter(emitter);
            }
            PhaseName::Exhale => {
                // Particles disperse outward from center
                let emitter = Emitter::new(
                    EmitterShape::Point {
                        x: self.center_x,
                        y: self.center_y,
                    },
                    25.0,
                    ParticleType::Outward,
                )
                .with_speed(5.0, 12.0)
                .with_life(2.0, 3.5)
                .with_size(0.6, 1.0);
                self.add_emitter(emitter);
            }
            PhaseName::Hold => {
                // Orbital particles around center
                let orbit_radius = 12.0 + scale * 5.0;
                let emitter = Emitter::new(
                    EmitterShape::Ring {
                        cx: self.center_x,
                        cy: self.center_y,
                        radius: orbit_radius,
                    },
                    15.0,
                    ParticleType::Orbital,
                )
                .with_speed(1.0, 2.0)
                .with_life(3.0, 5.0)
                .with_size(0.5, 0.8);
                self.add_emitter(emitter);
            }
            PhaseName::HoldAfterExhale => {
                // Very subtle ambient particles
                let emitter = Emitter::new(
                    EmitterShape::Ring {
                        cx: self.center_x,
                        cy: self.center_y,
                        radius: 15.0,
                    },
                    5.0,
                    ParticleType::Ambient,
                )
                .with_speed(0.5, 1.5)
                .with_life(2.0, 4.0)
                .with_size(0.3, 0.6);
                self.add_emitter(emitter);
            }
        }
    }

    /// Update all particles and emit new ones
    pub fn update(&mut self, dt: f64) {
        // Update existing particles
        self.particles.retain_mut(|p| {
            p.update(dt, self.center_x, self.center_y);
            p.is_alive()
        });

        // Emit new particles from emitters
        for emitter in &mut self.emitters {
            if self.particles.len() < self.max_particles {
                let new_particles = emitter.emit(dt);
                let remaining_capacity = self.max_particles - self.particles.len();
                self.particles.extend(new_particles.into_iter().take(remaining_capacity));
            }
        }
    }

    /// Spawn a burst of particles (for celebration, etc.)
    pub fn spawn_burst(&mut self, x: f64, y: f64, count: usize, particle_type: ParticleType) {
        for _ in 0..count {
            if self.particles.len() >= self.max_particles {
                break;
            }

            let angle = rand_f64() * std::f64::consts::TAU;
            let speed = 10.0 + rand_f64() * 20.0;
            let life = 1.5 + rand_f64() * 2.0;
            let size = 0.8 + rand_f64() * 0.8;

            self.particles.push(Particle::new_with_size(
                x, y, angle, speed, life, size, particle_type,
            ));
        }
    }

    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    /// Get iterator over particles
    pub fn iter(&self) -> impl Iterator<Item = &Particle> {
        self.particles.iter()
    }

    /// Number of active particles
    pub fn count(&self) -> usize {
        self.particles.len()
    }
}

// ============================================================================
// RANDOM NUMBER UTILITIES
// ============================================================================

/// Simple pseudo-random for particles (no external crate needed)
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

/// Random value between min and max
fn lerp_rand(min: f64, max: f64) -> f64 {
    min + rand_f64() * (max - min)
}
