# breathe

Military-grade breathing techniques in your terminal.

![breathe demo](https://raw.githubusercontent.com/AtomicIntuition/breathe-cli/master/demo.gif)

## Features

- **6 Breathing Techniques**: Box Breathing, 4-7-8 Relaxation, Tactical Recovery, Wim Hof, Resonance, and Physiological Sigh
- **Award-Winning Visuals**: Stunning, full-screen animated breathing visualizer with particle effects
- **Smooth Animations**: 60 FPS rendering with organic breathing curves and phase transitions
- **Session Tracking**: Configurable cycles with progress tracking and completion celebration
- **Audio Cues**: Optional audio feedback for phase transitions
- **Keyboard Driven**: Full keyboard navigation with vim-style bindings

## Installation

```bash
cargo install breathe
```

## Usage

```bash
# Interactive mode - select a technique
breathe

# Start a specific technique
breathe box          # Box Breathing (4-4-4-4)
breathe relax        # 4-7-8 Relaxation
breathe tactical     # Tactical Recovery
breathe wim          # Wim Hof Method
breathe resonance    # Resonance Breathing
breathe sigh         # Physiological Sigh

# Specify number of cycles
breathe box -c 8
```

## Techniques

| Technique | Pattern | Best For |
|-----------|---------|----------|
| Box Breathing | 4-4-4-4 | Focus & calm under pressure |
| 4-7-8 Relaxation | 4-7-8 | Sleep & deep relaxation |
| Tactical Recovery | 4-4-6-2 | Quick stress reset |
| Wim Hof | 2-0-2-0 | Energy & cold exposure prep |
| Resonance | 5-0-5-0 | Heart rate variability |
| Physiological Sigh | 1.2-0.8-0-5 | Immediate calm |

## Keybindings

| Key | Action |
|-----|--------|
| `Space` | Start / Pause / Resume |
| `Arrow keys` | Navigate / Adjust cycles |
| `Enter` | Select technique |
| `g` | View technique guide |
| `r` | Restart session |
| `b` / `Esc` | Back to menu |
| `?` | Help |
| `q` | Quit |

## Requirements

- Terminal with true color support (most modern terminals)
- Minimum size: 80x24

## License

MIT License - see [LICENSE](LICENSE)
