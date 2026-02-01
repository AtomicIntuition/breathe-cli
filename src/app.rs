//! Main application state and logic

#![allow(dead_code)]

use crate::animation::{ease_breath, smooth_damp};
use crate::particles::ParticleSystem;
use crate::techniques::{all_techniques, Phase, PhaseName, Technique};
use crate::theme::{blend_phase_colors, default_theme, PhaseColors};
use crate::ui::celebration::CelebrationAnimation;
use ratatui::widgets::ListState;
use std::time::{Duration, Instant};

/// Phase transition duration in seconds
const PHASE_TRANSITION_DURATION: f64 = 0.3;

/// Smooth damp time for transitions
const TRANSITION_SMOOTH_TIME: f64 = 0.15;

/// The main application state
pub struct App {
    pub techniques: Vec<Technique>,
    pub selected_index: usize,
    pub list_state: ListState,
    pub technique: Option<Technique>,
    pub state: AppState,
    pub cycles_target: u32,
    pub cycles_completed: u32,
    pub current_phase_index: usize,
    pub phase_start_time: Instant,
    pub session_start_time: Instant,

    // Enhanced particle system (replaces old particles Vec)
    pub particle_system: ParticleSystem,

    // Phase transition smoothing
    pub phase_transition_progress: f64,
    phase_transition_velocity: f64,
    previous_phase: Option<PhaseName>,

    // Celebration animation
    pub celebration: Option<CelebrationAnimation>,

    pub show_help: bool,
    pub show_guide: bool,
    pub audio_enabled: bool,

    // Pause tracking
    phase_elapsed_at_pause: f64,
    session_elapsed_at_pause: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Selecting,  // Choosing a technique
    Ready,      // Technique selected, waiting to start
    Breathing,  // Active session
    Paused,     // Session paused
    Complete,   // Session finished
}

// Legacy Particle struct kept for compatibility (but we use ParticleSystem now)
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    #[allow(dead_code)]
    pub size: u8,
}

impl Particle {
    pub fn new(x: f64, y: f64, angle: f64, speed: f64, life: f64) -> Self {
        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life,
            max_life: life,
            size: 1,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.life -= dt;
        self.vx *= 0.98;
        self.vy *= 0.98;
    }

    pub fn opacity(&self) -> f64 {
        (self.life / self.max_life).clamp(0.0, 1.0)
    }
}

impl App {
    /// Create app in interactive mode (technique selector)
    pub fn new_interactive() -> Self {
        let now = Instant::now();
        let techniques = all_techniques();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            techniques,
            selected_index: 0,
            list_state,
            technique: None,
            state: AppState::Selecting,
            cycles_target: 4,
            cycles_completed: 0,
            current_phase_index: 0,
            phase_start_time: now,
            session_start_time: now,
            particle_system: ParticleSystem::new(150), // 150 max particles (up from 50)
            phase_transition_progress: 1.0,
            phase_transition_velocity: 0.0,
            previous_phase: None,
            celebration: None,
            show_help: false,
            show_guide: false,
            audio_enabled: true,
            phase_elapsed_at_pause: 0.0,
            session_elapsed_at_pause: Duration::ZERO,
        }
    }

    /// Create app with a specific technique
    pub fn new_with_technique(technique: Technique, cycles: u32) -> Self {
        let now = Instant::now();
        let default_cycles = technique.default_cycles;
        let techniques = all_techniques();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            techniques,
            selected_index: 0,
            list_state,
            technique: Some(technique),
            state: AppState::Ready,
            cycles_target: if cycles > 0 { cycles } else { default_cycles },
            cycles_completed: 0,
            current_phase_index: 0,
            phase_start_time: now,
            session_start_time: now,
            particle_system: ParticleSystem::new(150),
            phase_transition_progress: 1.0,
            phase_transition_velocity: 0.0,
            previous_phase: None,
            celebration: None,
            show_help: false,
            show_guide: false,
            audio_enabled: true,
            phase_elapsed_at_pause: 0.0,
            session_elapsed_at_pause: Duration::ZERO,
        }
    }

    pub fn selected_technique(&self) -> &Technique {
        &self.techniques[self.selected_index]
    }

    pub fn current_technique(&self) -> &Technique {
        self.technique.as_ref().unwrap()
    }

    pub fn select_next(&mut self) {
        if self.state == AppState::Selecting {
            self.selected_index = (self.selected_index + 1) % self.techniques.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn select_prev(&mut self) {
        if self.state == AppState::Selecting {
            if self.selected_index == 0 {
                self.selected_index = self.techniques.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn confirm_selection(&mut self) {
        if self.state == AppState::Selecting {
            let technique = self.techniques[self.selected_index].clone();
            self.cycles_target = technique.default_cycles;
            self.technique = Some(technique);
            self.state = AppState::Ready;
        }
    }

    pub fn toggle_guide(&mut self) {
        self.show_guide = !self.show_guide;
    }

    pub fn toggle_audio(&mut self) {
        self.audio_enabled = !self.audio_enabled;
    }

    pub fn back_to_selection(&mut self) {
        self.state = AppState::Selecting;
        self.technique = None;
        self.current_phase_index = 0;
        self.cycles_completed = 0;
        self.particle_system.clear();
        self.celebration = None;
        self.phase_elapsed_at_pause = 0.0;
        self.session_elapsed_at_pause = Duration::ZERO;
        self.phase_transition_progress = 1.0;
        self.previous_phase = None;
    }

    pub fn adjust_cycles(&mut self, delta: i32) {
        if self.state == AppState::Ready {
            let new_cycles = (self.cycles_target as i32 + delta).max(1).min(99);
            self.cycles_target = new_cycles as u32;
        }
    }

    pub fn start(&mut self) {
        if self.technique.is_some() {
            self.state = AppState::Breathing;
            self.session_start_time = Instant::now();
            self.phase_start_time = Instant::now();
            self.current_phase_index = 0;
            self.cycles_completed = 0;
            self.phase_elapsed_at_pause = 0.0;
            self.session_elapsed_at_pause = Duration::ZERO;
            self.phase_transition_progress = 1.0;
            self.previous_phase = Some(self.current_phase().name);
            self.celebration = None;

            // Configure particle system for initial phase
            let scale = self.breath_scale();
            self.particle_system.configure_for_phase(self.current_phase().name, scale);
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            AppState::Breathing => {
                self.phase_elapsed_at_pause = self.phase_start_time.elapsed().as_secs_f64();
                self.session_elapsed_at_pause = self.session_start_time.elapsed();
                self.state = AppState::Paused;
            }
            AppState::Paused => {
                self.phase_start_time =
                    Instant::now() - Duration::from_secs_f64(self.phase_elapsed_at_pause);
                self.session_start_time = Instant::now() - self.session_elapsed_at_pause;
                self.state = AppState::Breathing;
            }
            _ => {}
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn reset(&mut self) {
        self.state = AppState::Ready;
        self.current_phase_index = 0;
        self.cycles_completed = 0;
        self.particle_system.clear();
        self.celebration = None;
        self.phase_elapsed_at_pause = 0.0;
        self.session_elapsed_at_pause = Duration::ZERO;
        self.phase_transition_progress = 1.0;
        self.previous_phase = None;
    }

    pub fn current_phase(&self) -> &Phase {
        &self.current_technique().phases[self.current_phase_index]
    }

    pub fn phase_elapsed(&self) -> f64 {
        if self.state == AppState::Paused {
            self.phase_elapsed_at_pause
        } else {
            self.phase_start_time.elapsed().as_secs_f64()
        }
    }

    pub fn phase_progress(&self) -> f64 {
        let elapsed = self.phase_elapsed();
        let duration = self.current_phase().duration_secs;
        (elapsed / duration).min(1.0)
    }

    pub fn session_elapsed(&self) -> Duration {
        if self.state == AppState::Paused || self.state == AppState::Complete {
            self.session_elapsed_at_pause
        } else {
            self.session_start_time.elapsed()
        }
    }

    /// Calculate the breathing circle scale (0.0 to 1.0) with organic easing
    pub fn breath_scale(&self) -> f64 {
        if self.technique.is_none() {
            return 0.5;
        }

        let progress = self.phase_progress();
        let phase = self.current_phase().name;

        // Use organic breathing easing curve
        let eased = ease_breath(progress);

        match phase {
            PhaseName::Inhale => eased,
            PhaseName::Hold => 1.0,
            PhaseName::Exhale => 1.0 - eased,
            PhaseName::HoldAfterExhale => 0.0,
        }
    }

    /// Get blended phase colors for smooth transitions between phases
    pub fn get_blended_phase_colors(&self) -> PhaseColors {
        let theme = default_theme();
        let current_colors = theme.get_phase_colors(self.current_phase().name);

        if let Some(prev_phase) = self.previous_phase {
            if self.phase_transition_progress < 1.0 {
                let prev_colors = theme.get_phase_colors(prev_phase);
                return blend_phase_colors(prev_colors, current_colors, self.phase_transition_progress);
            }
        }

        *current_colors
    }

    /// Update the app state (call this every frame)
    pub fn tick(&mut self, dt: f64) {
        // Update celebration animation if present
        if let Some(ref mut celebration) = self.celebration {
            celebration.tick(dt);
            if celebration.is_complete() {
                self.celebration = None;
            }
        }

        // Don't update breathing state if paused or complete
        if self.state != AppState::Breathing {
            return;
        }

        // Update phase transition progress
        if self.phase_transition_progress < 1.0 {
            self.phase_transition_progress = smooth_damp(
                self.phase_transition_progress,
                1.0,
                &mut self.phase_transition_velocity,
                TRANSITION_SMOOTH_TIME,
                dt,
            );
        }

        // Update particle system
        self.particle_system.update(dt);

        // Check for phase transition
        if self.phase_elapsed() >= self.current_phase().duration_secs {
            self.advance_phase();
        }
    }

    fn advance_phase(&mut self) {
        // Store previous phase for color blending
        self.previous_phase = Some(self.current_phase().name);

        self.current_phase_index += 1;

        // Check if cycle is complete
        if self.current_phase_index >= self.current_technique().phases.len() {
            self.current_phase_index = 0;
            self.cycles_completed += 1;

            // Check if session is complete
            if self.cycles_completed >= self.cycles_target {
                // Capture final duration before changing state
                self.session_elapsed_at_pause = self.session_start_time.elapsed();
                self.state = AppState::Complete;

                // Start celebration animation
                let mut celebration = CelebrationAnimation::new();
                celebration.set_center(0.0, 0.0);
                celebration.spawn_burst();
                self.celebration = Some(celebration);
                return;
            }
        }

        self.phase_start_time = Instant::now();

        // Reset transition progress for smooth color blending
        self.phase_transition_progress = 0.0;
        self.phase_transition_velocity = 0.0;

        // Reconfigure particle system for new phase
        let scale = self.breath_scale();
        self.particle_system.configure_for_phase(self.current_phase().name, scale);
    }

    pub fn format_time(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    // Legacy compatibility: provide access to particles as a vec slice
    // The new particle system stores particles internally
    pub fn particles(&self) -> &[crate::particles::Particle] {
        &self.particle_system.particles
    }
}
