//! UI module - rendering and layout

mod breath_visualizer;
mod breathing_circle;
pub mod celebration;
mod overlays;
mod widgets;

use crate::app::{App, AppState};
use crate::techniques::PhaseName;
use crate::theme::default_theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph},
    Frame,
};

pub use breath_visualizer::render_breath_visualizer;
#[allow(unused_imports)]
pub use breathing_circle::render_breathing_circle;

/// Main render function
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let theme = default_theme();

    // Dark background
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().style(Style::default().bg(theme.background)),
        area,
    );

    match app.state {
        AppState::Selecting => render_selector_screen(frame, app, area),
        AppState::Ready => render_ready_screen(frame, app, area),
        AppState::Breathing | AppState::Paused => render_session(frame, app, area),
        AppState::Complete => render_complete_screen(frame, app, area),
    }

    // Overlays
    if app.show_guide {
        render_guide_overlay(frame, app, area);
    }
    if app.show_help {
        render_help_overlay(frame, app, area);
    }
}

fn render_selector_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let theme = default_theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(8),     // Technique list
            Constraint::Length(6),  // Description
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Header
    render_selector_header(frame, chunks[0]);

    // Technique list with margins
    let list_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(chunks[1])[1];

    // Build technique list items
    let items: Vec<ListItem> = app.techniques
        .iter()
        .enumerate()
        .map(|(i, technique)| {
            let tc = technique.color;
            let is_selected = i == app.selected_index;

            let prefix = if is_selected { " ▸ " } else { "   " };
            let style = if is_selected {
                Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.ui.text_secondary)
            };

            let content = Line::from(vec![
                Span::styled(prefix, Style::default().fg(theme.ui.accent)),
                Span::styled("● ", Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b))),
                Span::styled(format!("{:<20}", technique.name), style),
                Span::styled(
                    format!(" {}", technique.pattern),
                    Style::default().fg(theme.ui.text_muted),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .highlight_style(Style::default().bg(theme.ui.border));

    // Use stateful rendering for scrolling
    frame.render_stateful_widget(list, list_area, &mut app.list_state);

    // Selected technique description panel
    let selected = app.selected_technique();

    let desc_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(chunks[2])[1];

    let desc_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.ui.border))
        .padding(Padding::new(1, 1, 1, 0));

    frame.render_widget(desc_block.clone(), desc_area);

    let inner = desc_block.inner(desc_area);

    // Wrap description text
    let wrapped = wrap_text(selected.description, inner.width.saturating_sub(2) as usize);
    let desc_lines: Vec<Line> = wrapped.into_iter()
        .take(3)  // Max 3 lines
        .map(|s| Line::from(Span::styled(s, Style::default().fg(theme.ui.text_secondary))))
        .collect();

    let desc_text = Paragraph::new(desc_lines);
    frame.render_widget(desc_text, inner);

    // Footer
    render_selector_footer(frame, chunks[3]);
}

fn render_selector_header(frame: &mut Frame, area: Rect) {
    let theme = default_theme();

    let header = Paragraph::new(Line::from(vec![
        Span::styled("◉ ", Style::default().fg(theme.ui.accent)),
        Span::styled("BREATHE", Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD)),
        Span::styled("  ·  ", Style::default().fg(theme.ui.border)),
        Span::styled("Select a technique", Style::default().fg(theme.ui.text_secondary)),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));

    frame.render_widget(header, area);
}

fn render_selector_footer(frame: &mut Frame, area: Rect) {
    let theme = default_theme();

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑↓", Style::default().fg(theme.ui.accent)),
        Span::styled(" navigate  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("g", Style::default().fg(theme.ui.accent)),
        Span::styled(" guide  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("ENTER", Style::default().fg(theme.ui.accent)),
        Span::styled(" select  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("q", Style::default().fg(theme.ui.accent)),
        Span::styled(" quit", Style::default().fg(theme.ui.text_muted)),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));

    frame.render_widget(footer, area);
}

fn render_ready_screen(frame: &mut Frame, app: &App, area: Rect) {
    let technique = app.current_technique();
    let tc = technique.color;
    let theme = default_theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Center content
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Header
    render_header(frame, app, chunks[0]);

    // Center content
    let center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Length(14),
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    // Technique info card
    let technique_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b)))
        .padding(Padding::horizontal(2))
        .style(Style::default().bg(Color::Rgb(15, 30, 50)));

    let technique_area = centered_rect(60, 100, center_chunks[1]);
    frame.render_widget(technique_block.clone(), technique_area);

    let inner = technique_block.inner(technique_area);
    let technique_text = vec![
        Line::from(""),
        Line::from(
            Span::styled(
                technique.name,
                Style::default()
                    .fg(theme.ui.text_primary)
                    .add_modifier(Modifier::BOLD),
            )
        ).centered(),
        Line::from(""),
        Line::from(
            Span::styled(
                technique.description,
                Style::default().fg(theme.ui.text_secondary),
            )
        ).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("Pattern: ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(
                technique.pattern,
                Style::default()
                    .fg(Color::Rgb(tc.r, tc.g, tc.b))
                    .add_modifier(Modifier::BOLD),
            ),
        ]).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("← ", Style::default().fg(theme.ui.text_muted)),
            Span::styled("Cycles: ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(
                format!("{}", app.cycles_target),
                Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" →", Style::default().fg(theme.ui.text_muted)),
        ]).centered(),
        Line::from(""),
    ];

    frame.render_widget(
        Paragraph::new(technique_text),
        inner,
    );

    // Start instruction
    let start_text = Line::from(vec![
        Span::styled("Press ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("SPACE", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
        Span::styled(" to begin", Style::default().fg(theme.ui.text_muted)),
    ]).centered();

    frame.render_widget(
        Paragraph::new(start_text),
        center_chunks[2],
    );

    // Footer
    render_ready_footer(frame, app, chunks[2]);
}

fn render_ready_footer(frame: &mut Frame, app: &App, area: Rect) {
    let theme = default_theme();
    let audio_icon = if app.audio_enabled { "♪" } else { "♪̸" };

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("←→", Style::default().fg(theme.ui.accent)),
        Span::styled(" cycles  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("g", Style::default().fg(theme.ui.accent)),
        Span::styled(" guide  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("a", Style::default().fg(theme.ui.accent)),
        Span::styled(format!(" {}  ", audio_icon), Style::default().fg(theme.ui.text_muted)),
        Span::styled("ESC", Style::default().fg(theme.ui.accent)),
        Span::styled(" back  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("q", Style::default().fg(theme.ui.accent)),
        Span::styled(" quit", Style::default().fg(theme.ui.text_muted)),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));

    frame.render_widget(footer, area);
}

fn render_session(frame: &mut Frame, app: &App, area: Rect) {
    // Responsive layout - larger visualizer area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(12),    // Breathing visualizer (expanded)
            Constraint::Length(6),  // Phase info with overlays
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Header with timer
    render_session_header(frame, app, chunks[0]);

    // New anatomical breath visualizer (centered with responsive bounds)
    let viz_area = chunks[1];
    render_breath_visualizer(frame, app, viz_area);

    // Enhanced phase indicator with progress bar and countdown
    render_enhanced_phase_info(frame, app, chunks[2]);

    // Footer
    render_session_footer(frame, chunks[3]);

    // Pause overlay
    if app.state == AppState::Paused {
        render_pause_overlay(frame, area);
    }
}

/// Enhanced phase info with giant indicator, progress bar, and countdown
fn render_enhanced_phase_info(frame: &mut Frame, app: &App, area: Rect) {
    let theme = default_theme();
    let phase = app.current_phase();
    let progress = app.phase_progress();
    let remaining = phase.duration_secs * (1.0 - progress);
    let time = app.session_elapsed().as_secs_f64();

    // Get blended phase colors
    let phase_colors = app.get_blended_phase_colors();

    let info_area = centered_rect(70, 100, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Phase name
            Constraint::Length(1),  // Progress bar
            Constraint::Length(1),  // Instruction + countdown
            Constraint::Length(1),  // Cycle dots
        ])
        .split(info_area);

    // Phase name with glow effect
    let phase_display = match phase.name {
        PhaseName::Inhale => "▲ INHALE ▲",
        PhaseName::Hold => "● HOLD ●",
        PhaseName::Exhale => "▼ EXHALE ▼",
        PhaseName::HoldAfterExhale => "○ REST ○",
    };

    let phase_color = phase_colors.text;
    let phase_text = Paragraph::new(Line::from(vec![
        Span::styled(
            phase_display,
            Style::default()
                .fg(phase_color)
                .add_modifier(Modifier::BOLD),
        )
    ]))
    .alignment(Alignment::Center);

    frame.render_widget(phase_text, chunks[0]);

    // Animated progress bar
    let bar_width = chunks[1].width.saturating_sub(4) as usize;
    let filled = ((bar_width as f64 * progress) as usize).min(bar_width);
    let empty = bar_width.saturating_sub(filled);

    let bar_char = match phase.name {
        PhaseName::Inhale => "▓",
        PhaseName::Exhale => "▒",
        _ => "█",
    };

    let bar_line = Line::from(vec![
        Span::styled("│", Style::default().fg(theme.ui.border)),
        Span::styled(bar_char.repeat(filled), Style::default().fg(phase_colors.primary)),
        Span::styled("░".repeat(empty), Style::default().fg(theme.ui.border)),
        Span::styled("│", Style::default().fg(theme.ui.border)),
    ]);

    frame.render_widget(Paragraph::new(bar_line).alignment(Alignment::Center), chunks[1]);

    // Instruction and countdown
    let instruction_line = Line::from(vec![
        Span::styled(phase.instruction, Style::default().fg(theme.ui.text_secondary)),
        Span::styled("  ·  ", Style::default().fg(theme.ui.border)),
        Span::styled(format!("{:.1}s", remaining.max(0.0)), Style::default().fg(theme.ui.text_muted)),
    ]);

    frame.render_widget(Paragraph::new(instruction_line).alignment(Alignment::Center), chunks[2]);

    // Cycle dots
    let completed = app.cycles_completed as usize;
    let target = app.cycles_target as usize;

    let max_display = 12;
    let (display_completed, display_target) = if target > max_display {
        let ratio = completed as f64 / target as f64;
        ((ratio * max_display as f64).round() as usize, max_display)
    } else {
        (completed, target)
    };

    let mut dots = Vec::new();
    for i in 0..display_target {
        if i < display_completed {
            dots.push(Span::styled("●", Style::default().fg(theme.ui.success)));
        } else if i == display_completed {
            let pulse_val = (time * 3.0).sin() * 0.3 + 0.7;
            let pulse_color = if let Color::Rgb(r, g, b) = theme.ui.accent {
                Color::Rgb(
                    (r as f64 * pulse_val) as u8,
                    (g as f64 * pulse_val) as u8,
                    (b as f64 * pulse_val) as u8,
                )
            } else {
                theme.ui.accent
            };
            dots.push(Span::styled("◉", Style::default().fg(pulse_color)));
        } else {
            dots.push(Span::styled("○", Style::default().fg(theme.ui.text_muted)));
        }
        if i < display_target - 1 {
            dots.push(Span::raw(" "));
        }
    }

    if target > max_display {
        dots.push(Span::styled(format!(" ({}/{})", completed, target), Style::default().fg(theme.ui.text_muted)));
    }

    frame.render_widget(Paragraph::new(Line::from(dots)).alignment(Alignment::Center), chunks[3]);
}

fn render_complete_screen(frame: &mut Frame, app: &App, area: Rect) {
    let technique = app.current_technique();
    let tc = technique.color;
    let theme = default_theme();

    // Render celebration animation if active
    if let Some(ref celebration) = app.celebration {
        celebration.render(frame, area);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, app, chunks[0]);

    let center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(14),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    // Completion card
    let complete_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b)))
        .padding(Padding::horizontal(2))
        .style(Style::default().bg(Color::Rgb(15, 30, 50)));

    let complete_area = centered_rect(50, 100, center_chunks[1]);
    frame.render_widget(complete_block.clone(), complete_area);

    let inner = complete_block.inner(complete_area);
    let elapsed = App::format_time(app.session_elapsed());

    let complete_text = vec![
        Line::from(""),
        Line::from(
            Span::styled(
                "✓ Session Complete",
                Style::default()
                    .fg(theme.ui.success)
                    .add_modifier(Modifier::BOLD),
            )
        ).centered(),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Technique  ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(technique.name, Style::default().fg(theme.ui.text_primary)),
        ]).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("Cycles     ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(format!("{}", app.cycles_completed), Style::default().fg(theme.ui.text_primary)),
        ]).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("Duration   ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(elapsed, Style::default().fg(theme.ui.text_primary)),
        ]).centered(),
        Line::from(""),
    ];

    frame.render_widget(Paragraph::new(complete_text), inner);

    // Restart instruction
    let restart_text = Line::from(vec![
        Span::styled("Press ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("R", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
        Span::styled(" to restart  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("B", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
        Span::styled(" techniques  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("Q", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
        Span::styled(" quit", Style::default().fg(theme.ui.text_muted)),
    ]).centered();

    frame.render_widget(Paragraph::new(restart_text), center_chunks[2]);

    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let tc = app.current_technique().color;
    let theme = default_theme();

    let header = Paragraph::new(Line::from(vec![
        Span::styled("◉ ", Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b))),
        Span::styled("BREATHE", Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD)),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));

    frame.render_widget(header, area);
}

fn render_session_header(frame: &mut Frame, app: &App, area: Rect) {
    let technique = app.current_technique();
    let elapsed = App::format_time(app.session_elapsed());
    let tc = technique.color;
    let theme = default_theme();

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    // Left: technique name
    let left = Paragraph::new(Line::from(vec![
        Span::styled("◉ ", Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b))),
        Span::styled(technique.name, Style::default().fg(theme.ui.text_secondary)),
    ]))
    .block(Block::default().padding(Padding::new(2, 0, 1, 0)));
    frame.render_widget(left, header_chunks[0]);

    // Center: cycle count
    let center = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("{}", app.cycles_completed + 1),
            Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" / {}", app.cycles_target),
            Style::default().fg(theme.ui.text_muted),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));
    frame.render_widget(center, header_chunks[1]);

    // Right: timer
    let right = Paragraph::new(Line::from(
        Span::styled(elapsed, Style::default().fg(theme.ui.text_secondary)),
    ))
    .alignment(Alignment::Right)
    .block(Block::default().padding(Padding::new(0, 2, 1, 0)));
    frame.render_widget(right, header_chunks[2]);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let theme = default_theme();

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("?", Style::default().fg(theme.ui.accent)),
        Span::styled(" help  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("q", Style::default().fg(theme.ui.accent)),
        Span::styled(" quit", Style::default().fg(theme.ui.text_muted)),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));

    frame.render_widget(footer, area);
}

fn render_session_footer(frame: &mut Frame, area: Rect) {
    let theme = default_theme();

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("SPACE", Style::default().fg(theme.ui.accent)),
        Span::styled(" pause  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("?", Style::default().fg(theme.ui.accent)),
        Span::styled(" help  ", Style::default().fg(theme.ui.text_muted)),
        Span::styled("q", Style::default().fg(theme.ui.accent)),
        Span::styled(" quit", Style::default().fg(theme.ui.text_muted)),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().padding(Padding::vertical(1)));

    frame.render_widget(footer, area);
}

fn render_pause_overlay(frame: &mut Frame, area: Rect) {
    let theme = default_theme();

    // Darken background
    let dim_block = Block::default()
        .style(Style::default().bg(theme.background_dark));
    frame.render_widget(dim_block, area);

    let overlay_area = centered_rect(40, 30, area);

    frame.render_widget(Clear, overlay_area);

    let pause_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.ui.warning))
        .style(Style::default().bg(Color::Rgb(15, 30, 50)));

    frame.render_widget(pause_block.clone(), overlay_area);

    let inner = pause_block.inner(overlay_area);
    let pause_text = Paragraph::new(vec![
        Line::from(""),
        Line::from(
            Span::styled("⏸  PAUSED", Style::default().fg(theme.ui.warning).add_modifier(Modifier::BOLD))
        ).centered(),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("SPACE", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
            Span::styled("  resume", Style::default().fg(theme.ui.text_secondary)),
        ]).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("R", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
            Span::styled("      restart", Style::default().fg(theme.ui.text_secondary)),
        ]).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("B", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
            Span::styled("      back to menu", Style::default().fg(theme.ui.text_secondary)),
        ]).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("Q", Style::default().fg(theme.ui.accent).add_modifier(Modifier::BOLD)),
            Span::styled("      quit", Style::default().fg(theme.ui.text_secondary)),
        ]).centered(),
    ]);

    frame.render_widget(pause_text, inner);
}

fn render_help_overlay(frame: &mut Frame, app: &App, area: Rect) {
    let theme = default_theme();
    let overlay_area = centered_rect(55, 65, area);

    frame.render_widget(Clear, overlay_area);

    let help_block = Block::default()
        .title(" Keyboard Shortcuts ")
        .title_style(Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.ui.accent))
        .padding(Padding::uniform(1))
        .style(Style::default().bg(Color::Rgb(15, 30, 50)));

    frame.render_widget(help_block.clone(), overlay_area);

    let inner = help_block.inner(overlay_area);

    let help_lines = match app.state {
        AppState::Selecting => vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ↑ / k       ", Style::default().fg(theme.ui.accent)),
                Span::styled("Previous technique", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ↓ / j       ", Style::default().fg(theme.ui.accent)),
                Span::styled("Next technique", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ENTER       ", Style::default().fg(theme.ui.accent)),
                Span::styled("Select technique", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Q / ESC     ", Style::default().fg(theme.ui.accent)),
                Span::styled("Quit", Style::default().fg(theme.ui.text_secondary)),
            ]),
        ],
        _ => vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  SPACE       ", Style::default().fg(theme.ui.accent)),
                Span::styled("Start / Pause / Resume", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ← / →       ", Style::default().fg(theme.ui.accent)),
                Span::styled("Adjust cycles", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  R           ", Style::default().fg(theme.ui.accent)),
                Span::styled("Restart session", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  B / ESC     ", Style::default().fg(theme.ui.accent)),
                Span::styled("Back to techniques", Style::default().fg(theme.ui.text_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Q           ", Style::default().fg(theme.ui.accent)),
                Span::styled("Quit", Style::default().fg(theme.ui.text_secondary)),
            ]),
        ],
    };

    let mut lines = help_lines;
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(
        Line::from(
            Span::styled("Press any key to close", Style::default().fg(theme.ui.text_muted))
        ).centered()
    );

    frame.render_widget(Paragraph::new(lines), inner);
}

fn render_guide_overlay(frame: &mut Frame, app: &App, area: Rect) {
    let theme = default_theme();
    let technique = if app.technique.is_some() {
        app.current_technique()
    } else {
        app.selected_technique()
    };
    let tc = technique.color;

    let overlay_area = centered_rect(75, 85, area);

    frame.render_widget(Clear, overlay_area);

    let guide_block = Block::default()
        .title(format!(" {} ", technique.name))
        .title_style(Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b)))
        .padding(Padding::uniform(1))
        .style(Style::default().bg(Color::Rgb(15, 30, 50)));

    frame.render_widget(guide_block.clone(), overlay_area);

    let inner = guide_block.inner(overlay_area);

    // Build guide content
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(technique.tagline, Style::default().fg(Color::Rgb(tc.r, tc.g, tc.b)).add_modifier(Modifier::ITALIC)),
        ]).centered(),
        Line::from(""),
        Line::from(""),
        // Description
        Line::from(vec![
            Span::styled("About", Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];

    // Word-wrap description
    for line in wrap_text(technique.description, 60) {
        lines.push(Line::from(Span::styled(line, Style::default().fg(theme.ui.text_secondary))));
    }

    lines.extend(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Pattern  ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(technique.pattern, Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Purpose  ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(technique.purpose, Style::default().fg(theme.ui.text_secondary)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Best For ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(technique.use_case, Style::default().fg(theme.ui.text_secondary)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Source   ", Style::default().fg(theme.ui.text_muted)),
            Span::styled(technique.source, Style::default().fg(theme.ui.text_muted).add_modifier(Modifier::ITALIC)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Phases", Style::default().fg(theme.ui.text_primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ]);

    // Add phase breakdown
    for (i, phase) in technique.phases.iter().enumerate() {
        let phase_color = match phase.name {
            PhaseName::Inhale => Color::Rgb(74, 144, 217),
            PhaseName::Hold => Color::Rgb(201, 162, 39),
            PhaseName::Exhale => Color::Rgb(139, 92, 246),
            PhaseName::HoldAfterExhale => Color::Rgb(100, 116, 139),
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {}. ", i + 1), Style::default().fg(theme.ui.text_muted)),
            Span::styled(format!("{:<8}", phase.name.display()), Style::default().fg(phase_color)),
            Span::styled(format!("{:>4}s  ", phase.duration_secs as u32), Style::default().fg(theme.ui.text_primary)),
            Span::styled(phase.instruction, Style::default().fg(theme.ui.text_secondary)),
        ]));
    }

    lines.extend(vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled("Press any key to close", Style::default().fg(theme.ui.text_muted))).centered(),
    ]);

    frame.render_widget(Paragraph::new(lines), inner);
}

/// Simple text wrapper
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
