//! Enhanced UI overlay components - giant phase indicator, progress bar, countdown, cycle dots

use crate::animation::{ease_breath, pulse_breath};
use crate::app::App;
use crate::techniques::PhaseName;
use crate::theme::{default_theme, with_opacity};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render a giant, pulsing phase indicator that scales with breathing
#[allow(dead_code)]
pub fn render_giant_phase_indicator(frame: &mut Frame, app: &App, area: Rect) {
    let phase = app.current_phase();
    let time = app.session_elapsed().as_secs_f64();
    let theme = default_theme();

    let phase_colors = theme.get_phase_colors(phase.name);

    // Calculate pulse effect
    let pulse = pulse_breath(time, 0.5);
    let breath_scale = app.breath_scale();

    // Determine text size based on area (simulate "giant" with padding)
    let vertical_padding = (area.height.saturating_sub(5)) / 2;

    let phase_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_padding),
            Constraint::Length(3),  // Phase name
            Constraint::Length(2),  // Instruction
            Constraint::Min(0),
        ])
        .split(area);

    // Phase name with glow effect (brighter during active breathing)
    let glow_intensity = 0.7 + pulse * 0.3 + breath_scale * 0.2;
    let phase_color = if let Color::Rgb(r, g, b) = phase_colors.text {
        Color::Rgb(
            ((r as f64 * glow_intensity).min(255.0)) as u8,
            ((g as f64 * glow_intensity).min(255.0)) as u8,
            ((b as f64 * glow_intensity).min(255.0)) as u8,
        )
    } else {
        phase_colors.text
    };

    // Create the phase name display with visual emphasis
    let phase_display = match phase.name {
        PhaseName::Inhale => "▲ INHALE ▲",
        PhaseName::Hold => "● HOLD ●",
        PhaseName::Exhale => "▼ EXHALE ▼",
        PhaseName::HoldAfterExhale => "○ REST ○",
    };

    let phase_text = Paragraph::new(Line::from(vec![Span::styled(
        phase_display,
        Style::default()
            .fg(phase_color)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);

    frame.render_widget(phase_text, phase_area[1]);

    // Instruction text (pulsing opacity)
    let instruction_opacity = 0.5 + pulse * 0.3;
    let instruction_color = with_opacity(theme.ui.text_secondary, instruction_opacity);

    let instruction_text = Paragraph::new(Line::from(Span::styled(
        phase.instruction,
        Style::default().fg(instruction_color),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(instruction_text, phase_area[2]);
}

/// Render a breathing progress bar that pulses with the breath
#[allow(dead_code)]
pub fn render_breathing_progress_bar(frame: &mut Frame, app: &App, area: Rect) {
    let progress = app.phase_progress();
    let phase = app.current_phase();
    let time = app.session_elapsed().as_secs_f64();
    let theme = default_theme();

    let phase_colors = theme.get_phase_colors(phase.name);

    // Calculate the visual progress with easing
    let eased_progress = ease_breath(progress);

    // Bar dimensions
    let bar_width = area.width.saturating_sub(4) as usize;
    let filled_width = ((bar_width as f64 * eased_progress) as usize).min(bar_width);
    let empty_width = bar_width.saturating_sub(filled_width);

    // Breathing pulse effect on the bar
    let pulse = pulse_breath(time, 1.0);
    let bar_brightness = 0.8 + pulse * 0.2;

    let bar_color = if let Color::Rgb(r, g, b) = phase_colors.primary {
        Color::Rgb(
            ((r as f64 * bar_brightness).min(255.0)) as u8,
            ((g as f64 * bar_brightness).min(255.0)) as u8,
            ((b as f64 * bar_brightness).min(255.0)) as u8,
        )
    } else {
        phase_colors.primary
    };

    // Build the progress bar
    let filled_char = match phase.name {
        PhaseName::Inhale => "▓",
        PhaseName::Exhale => "▒",
        _ => "█",
    };

    let bar_line = Line::from(vec![
        Span::styled("│", Style::default().fg(theme.ui.border)),
        Span::styled(
            filled_char.repeat(filled_width),
            Style::default().fg(bar_color),
        ),
        Span::styled(
            "░".repeat(empty_width),
            Style::default().fg(theme.ui.border),
        ),
        Span::styled("│", Style::default().fg(theme.ui.border)),
    ]);

    let bar_widget = Paragraph::new(bar_line).alignment(Alignment::Center);

    frame.render_widget(bar_widget, area);
}

/// Render a countdown timer showing time remaining in current phase
#[allow(dead_code)]
pub fn render_countdown_timer(frame: &mut Frame, app: &App, area: Rect) {
    let phase = app.current_phase();
    let progress = app.phase_progress();
    let time = app.session_elapsed().as_secs_f64();
    let theme = default_theme();

    let phase_colors = theme.get_phase_colors(phase.name);

    // Calculate remaining time
    let remaining = phase.duration_secs * (1.0 - progress);
    let remaining_display = format!("{:.1}s", remaining.max(0.0));

    // Pulse effect near end of phase
    let urgency_pulse = if remaining < 2.0 {
        ((time * 4.0).sin() * 0.3 + 0.7).max(0.4)
    } else {
        1.0
    };

    let timer_color = if remaining < 2.0 {
        // Flash between phase color and white near end
        if let Color::Rgb(r, g, b) = phase_colors.primary {
            Color::Rgb(
                ((r as f64 * urgency_pulse + 255.0 * (1.0 - urgency_pulse)).min(255.0)) as u8,
                ((g as f64 * urgency_pulse + 255.0 * (1.0 - urgency_pulse)).min(255.0)) as u8,
                ((b as f64 * urgency_pulse + 255.0 * (1.0 - urgency_pulse)).min(255.0)) as u8,
            )
        } else {
            phase_colors.primary
        }
    } else {
        theme.ui.text_muted
    };

    let timer_text = Paragraph::new(Line::from(Span::styled(
        remaining_display,
        Style::default().fg(timer_color),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(timer_text, area);
}

/// Render cycle progress dots showing completed and remaining cycles
#[allow(dead_code)]
pub fn render_cycle_dots(frame: &mut Frame, app: &App, area: Rect) {
    let completed = app.cycles_completed as usize;
    let target = app.cycles_target as usize;
    let time = app.session_elapsed().as_secs_f64();
    let theme = default_theme();

    // Limit displayed dots for very long sessions
    let max_display = 12;
    let (display_completed, display_target) = if target > max_display {
        // Show proportional representation
        let ratio = completed as f64 / target as f64;
        let scaled_completed = (ratio * max_display as f64).round() as usize;
        (scaled_completed.min(max_display), max_display)
    } else {
        (completed, target)
    };

    // Build the dots display
    let mut spans = Vec::new();

    // Current cycle indicator (pulsing)
    let current_pulse = (time * 2.0).sin() * 0.3 + 0.7;

    for i in 0..display_target {
        if i < display_completed {
            // Completed cycle - filled dot
            spans.push(Span::styled(
                "●",
                Style::default().fg(theme.ui.success),
            ));
        } else if i == display_completed {
            // Current cycle - pulsing dot with accent color
            let pulse_color = if let Color::Rgb(r, g, b) = theme.ui.accent {
                Color::Rgb(
                    ((r as f64 * current_pulse).min(255.0)) as u8,
                    ((g as f64 * current_pulse).min(255.0)) as u8,
                    ((b as f64 * current_pulse).min(255.0)) as u8,
                )
            } else {
                theme.ui.accent
            };
            spans.push(Span::styled(
                "◉",
                Style::default().fg(pulse_color),
            ));
        } else {
            // Future cycle - empty dot
            spans.push(Span::styled(
                "○",
                Style::default().fg(theme.ui.text_muted),
            ));
        }

        // Add spacing between dots
        if i < display_target - 1 {
            spans.push(Span::raw(" "));
        }
    }

    // Add numeric display if cycles were compressed
    if target > max_display {
        spans.push(Span::styled(
            format!(" ({}/{})", completed, target),
            Style::default().fg(theme.ui.text_muted),
        ));
    }

    let dots_widget = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);

    frame.render_widget(dots_widget, area);
}

/// Render a combined phase info panel with all overlays
#[allow(dead_code)]
pub fn render_phase_info_panel(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Giant phase indicator
            Constraint::Length(1),  // Progress bar
            Constraint::Length(1),  // Countdown
            Constraint::Length(1),  // Cycle dots
        ])
        .split(area);

    render_giant_phase_indicator(frame, app, chunks[0]);
    render_breathing_progress_bar(frame, app, chunks[1]);
    render_countdown_timer(frame, app, chunks[2]);
    render_cycle_dots(frame, app, chunks[3]);
}

/// Render session stats in a compact format
#[allow(dead_code)]
pub fn render_session_stats(frame: &mut Frame, app: &App, area: Rect) {
    let theme = default_theme();
    let elapsed = crate::app::App::format_time(app.session_elapsed());

    let stats_line = Line::from(vec![
        Span::styled("Cycle ", Style::default().fg(theme.ui.text_muted)),
        Span::styled(
            format!("{}", app.cycles_completed + 1),
            Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("/{}", app.cycles_target),
            Style::default().fg(theme.ui.text_muted),
        ),
        Span::styled("  ·  ", Style::default().fg(theme.ui.border)),
        Span::styled(elapsed, Style::default().fg(theme.ui.text_secondary)),
    ]);

    let stats_widget = Paragraph::new(stats_line).alignment(Alignment::Center);

    frame.render_widget(stats_widget, area);
}
