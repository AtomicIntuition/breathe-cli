use rodio::{OutputStream, Sink, Source};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

/// Audio player for breathing cues
pub struct AudioPlayer {
    sender: Option<Sender<AudioCommand>>,
}

enum AudioCommand {
    PlayTone { frequency: f32, duration_ms: u64 },
    Stop,
}

impl AudioPlayer {
    /// Create a new audio player
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<AudioCommand>();

        // Spawn audio thread
        thread::spawn(move || {
            // Try to get audio output
            let output = match OutputStream::try_default() {
                Ok((stream, handle)) => Some((stream, handle)),
                Err(_) => None,
            };

            if let Some((_stream, handle)) = output {
                while let Ok(cmd) = receiver.recv() {
                    match cmd {
                        AudioCommand::PlayTone { frequency, duration_ms } => {
                            if let Ok(sink) = Sink::try_new(&handle) {
                                let source = SineWave::new(frequency)
                                    .take_duration(Duration::from_millis(duration_ms))
                                    .amplify(0.15)  // Quiet, subtle tone
                                    .fade_in(Duration::from_millis(20))
                                    .buffered();
                                sink.append(source);
                                sink.sleep_until_end();
                            }
                        }
                        AudioCommand::Stop => break,
                    }
                }
            }
        });

        Self {
            sender: Some(sender),
        }
    }

    /// Play a tone for phase transitions
    pub fn play_phase_tone(&self, phase: PhaseTone) {
        if let Some(ref sender) = self.sender {
            let (frequency, duration_ms) = match phase {
                PhaseTone::Inhale => (440.0, 150),      // A4 - start breathing in
                PhaseTone::Hold => (523.25, 100),      // C5 - hold
                PhaseTone::Exhale => (349.23, 150),    // F4 - breathe out
                PhaseTone::HoldEmpty => (293.66, 100), // D4 - hold empty
                PhaseTone::Start => (523.25, 200),     // C5 - session start
                PhaseTone::Complete => (659.25, 300),  // E5 - session complete
            };
            let _ = sender.send(AudioCommand::PlayTone { frequency, duration_ms });
        }
    }

    /// Check if audio is available
    #[allow(dead_code)]
    pub fn is_available(&self) -> bool {
        self.sender.is_some()
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(AudioCommand::Stop);
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of audio cues
#[derive(Debug, Clone, Copy)]
pub enum PhaseTone {
    Inhale,
    Hold,
    Exhale,
    HoldEmpty,
    Start,
    Complete,
}

/// Simple sine wave source
struct SineWave {
    frequency: f32,
    sample_rate: u32,
    sample_index: u64,
}

impl SineWave {
    fn new(frequency: f32) -> Self {
        Self {
            frequency,
            sample_rate: 44100,
            sample_index: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let t = self.sample_index as f32 / self.sample_rate as f32;
        let sample = (t * self.frequency * 2.0 * std::f32::consts::PI).sin();
        self.sample_index += 1;
        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
