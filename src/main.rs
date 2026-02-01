mod animation;
mod app;
mod audio;
mod particles;
mod techniques;
mod theme;
mod ui;

use anyhow::Result;
use app::{App, AppState};
use audio::{AudioPlayer, PhaseTone};
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};
use techniques::{all_techniques, get_technique, PhaseName};

/// BREATHE - Military-grade breathing techniques in your terminal
#[derive(Parser)]
#[command(
    name = "breathe",
    author = "Atomic Intuition",
    version,
    about = "Military-grade breathing techniques in your terminal",
    long_about = "Practice scientifically-backed breathing techniques used by Navy SEALs, \
                  athletes, and peak performers. Right in your terminal.",
    after_help = "EXAMPLES:\n    \
                  breathe                  Interactive technique selector\n    \
                  breathe box              Start box breathing (4-4-4-4)\n    \
                  breathe 478 -c 6         4-7-8 breathing for 6 cycles\n    \
                  breathe list             Show all techniques\n    \
                  breathe --help           Show this help"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // === FOCUS & PERFORMANCE ===
    /// Box breathing - Navy SEAL technique (4-4-4-4)
    #[command(visible_alias = "b")]
    Box {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Gateway Process - CIA declassified technique
    Gateway {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Operative Protocol - Field agent standard
    Operative {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// SERE Breathing - Survival training technique
    Sere {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    // === STRESS & CALM ===
    /// Combat breathing - Rapid calm-down
    Combat {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Physiological sigh - Instant calm reset
    #[command(visible_alias = "sigh")]
    PhysiologicalSigh {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Coherent breathing - Heart-brain sync
    Coherent {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Resonant breathing - Vagal tone builder
    Resonant {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    // === SLEEP & RELAXATION ===
    /// Military sleep method - 2-minute sleep technique
    #[command(name = "military-sleep", visible_alias = "sleep")]
    MilitarySleep {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// 4-7-8 breathing - Natural tranquilizer
    #[command(name = "478")]
    FourSevenEight {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Sleep exhale - Extended exhale for sleep
    #[command(name = "sleep-exhale")]
    SleepExhale {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    // === ENERGY & ACTIVATION ===
    /// Energizing breath - Natural energy surge
    Energize {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Power breathing - Pre-mission activation
    Power {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// Wim Hof Method - The Iceman protocol
    #[command(name = "wim-hof", visible_alias = "wh")]
    WimHof {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    // === RECOVERY & HEALING ===
    /// Recovery breathing - Post-stress recovery
    Recovery {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// NSDR breathing - Non-sleep deep rest
    Nsdr {
        #[arg(short, long)]
        cycles: Option<u32>,
    },

    /// List all available breathing techniques
    #[command(visible_alias = "ls")]
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List) => {
            print_techniques_list();
            Ok(())
        }
        Some(cmd) => {
            let (technique_id, cycles) = match cmd {
                // Focus & Performance
                Commands::Box { cycles } => ("box", cycles),
                Commands::Gateway { cycles } => ("gateway", cycles),
                Commands::Operative { cycles } => ("operative", cycles),
                Commands::Sere { cycles } => ("sere", cycles),
                // Stress & Calm
                Commands::Combat { cycles } => ("combat", cycles),
                Commands::PhysiologicalSigh { cycles } => ("physiological-sigh", cycles),
                Commands::Coherent { cycles } => ("coherent", cycles),
                Commands::Resonant { cycles } => ("resonant", cycles),
                // Sleep & Relaxation
                Commands::MilitarySleep { cycles } => ("military-sleep", cycles),
                Commands::FourSevenEight { cycles } => ("478", cycles),
                Commands::SleepExhale { cycles } => ("sleep-exhale", cycles),
                // Energy & Activation
                Commands::Energize { cycles } => ("energize", cycles),
                Commands::Power { cycles } => ("power", cycles),
                Commands::WimHof { cycles } => ("wim-hof", cycles),
                // Recovery & Healing
                Commands::Recovery { cycles } => ("recovery", cycles),
                Commands::Nsdr { cycles } => ("nsdr", cycles),
                Commands::List => unreachable!(),
            };

            let technique = get_technique(technique_id)
                .expect("Unknown technique");
            let cycle_count = cycles.unwrap_or(technique.default_cycles);

            run_with_technique(technique, cycle_count)
        }
        None => {
            // Interactive mode - show technique selector
            run_interactive()
        }
    }
}

fn print_techniques_list() {
    println!();
    println!("  \x1b[1;38;5;75m◉ BREATHE\x1b[0m - Available Techniques");
    println!("  \x1b[38;5;240m─────────────────────────────────────────\x1b[0m");
    println!();

    for technique in all_techniques() {
        let tc = technique.color;
        println!(
            "  \x1b[38;2;{};{};{}m●\x1b[0m \x1b[1m{:<20}\x1b[0m \x1b[38;5;245m{}\x1b[0m",
            tc.r, tc.g, tc.b,
            technique.name,
            technique.pattern
        );
        println!(
            "    \x1b[38;5;240m{}\x1b[0m",
            technique.description
        );
        println!(
            "    \x1b[38;5;75mbreathe {}\x1b[0m",
            technique.id
        );
        println!();
    }

    println!("  \x1b[38;5;240m─────────────────────────────────────────\x1b[0m");
    println!("  \x1b[38;5;245mUsage:\x1b[0m breathe <technique> [-c cycles]");
    println!("  \x1b[38;5;245mOr just:\x1b[0m breathe \x1b[38;5;240m(for interactive selector)\x1b[0m");
    println!();
}

fn run_interactive() -> Result<()> {
    // Initialize audio
    let audio = AudioPlayer::new();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app in interactive mode
    let mut app = App::new_interactive();

    // Run the main loop
    let result = run_loop(&mut terminal, &mut app, &audio);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print session summary if completed
    if app.state == AppState::Complete {
        print_session_summary(&app);
    }

    result
}

fn run_with_technique(technique: techniques::Technique, cycles: u32) -> Result<()> {
    // Initialize audio
    let audio = AudioPlayer::new();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app with specific technique
    let mut app = App::new_with_technique(technique, cycles);

    // Run the main loop
    let result = run_loop(&mut terminal, &mut app, &audio);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print session summary if completed
    if app.state == AppState::Complete {
        print_session_summary(&app);
    }

    result
}

fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    audio: &AudioPlayer,
) -> Result<()> {
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        // Render
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle input with timeout
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // If help or guide is showing, any key closes it
                    if app.show_help {
                        app.show_help = false;
                        continue;
                    }
                    if app.show_guide {
                        app.show_guide = false;
                        continue;
                    }

                    match app.state {
                        AppState::Selecting => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
                            KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                            KeyCode::Enter | KeyCode::Char(' ') => app.confirm_selection(),
                            KeyCode::Char('g') => app.toggle_guide(),
                            KeyCode::Char('?') => app.toggle_help(),
                            _ => {}
                        },
                        AppState::Ready => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Esc | KeyCode::Char('b') => app.back_to_selection(),
                            KeyCode::Char(' ') | KeyCode::Enter => {
                                app.start();
                                if app.audio_enabled {
                                    audio.play_phase_tone(PhaseTone::Start);
                                }
                            },
                            KeyCode::Left => app.adjust_cycles(-1),
                            KeyCode::Right => app.adjust_cycles(1),
                            KeyCode::Char('g') => app.toggle_guide(),
                            KeyCode::Char('a') => app.toggle_audio(),
                            KeyCode::Char('?') => app.toggle_help(),
                            _ => {}
                        },
                        AppState::Breathing => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char(' ') => app.toggle_pause(),
                            KeyCode::Char('a') => app.toggle_audio(),
                            KeyCode::Char('?') => app.toggle_help(),
                            _ => {}
                        },
                        AppState::Paused => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Esc | KeyCode::Char('b') => app.back_to_selection(),
                            KeyCode::Char(' ') => app.toggle_pause(),
                            KeyCode::Char('r') => app.reset(),
                            KeyCode::Char('?') => app.toggle_help(),
                            _ => {}
                        },
                        AppState::Complete => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('r') => app.reset(),
                            KeyCode::Char('b') => app.back_to_selection(),
                            KeyCode::Char('?') => app.toggle_help(),
                            _ => {}
                        },
                    }
                }
            }
        }

        // Update app state and check for phase changes
        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            let prev_phase = app.current_phase_index;
            let prev_state = app.state;
            app.tick(dt);

            // Play sound on phase change
            if app.audio_enabled && app.state == AppState::Breathing && app.current_phase_index != prev_phase {
                let tone = match app.current_phase().name {
                    PhaseName::Inhale => PhaseTone::Inhale,
                    PhaseName::Hold => PhaseTone::Hold,
                    PhaseName::Exhale => PhaseTone::Exhale,
                    PhaseName::HoldAfterExhale => PhaseTone::HoldEmpty,
                };
                audio.play_phase_tone(tone);
            }

            // Play completion sound
            if app.audio_enabled && prev_state == AppState::Breathing && app.state == AppState::Complete {
                audio.play_phase_tone(PhaseTone::Complete);
            }

            last_tick = Instant::now();
        }
    }
}

fn print_session_summary(app: &App) {
    let technique = app.current_technique();
    let tc = technique.color;
    let elapsed = App::format_time(app.session_elapsed());

    println!();
    println!("  \x1b[1;38;5;82m✓ Session Complete\x1b[0m");
    println!();
    println!(
        "  \x1b[38;2;{};{};{}m●\x1b[0m {} · {} cycles · {}",
        tc.r, tc.g, tc.b,
        technique.name,
        app.cycles_completed,
        elapsed
    );
    println!();
    println!("  \x1b[38;5;245mTake a moment to notice how you feel.\x1b[0m");
    println!();
}
