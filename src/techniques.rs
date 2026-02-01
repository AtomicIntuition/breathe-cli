use serde::{Deserialize, Serialize};

/// A breathing phase with duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    pub name: PhaseName,
    pub duration_secs: f64,
    pub instruction: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseName {
    Inhale,
    Hold,
    Exhale,
    HoldAfterExhale,
}

impl PhaseName {
    #[allow(dead_code)]
    pub fn display(&self) -> &'static str {
        match self {
            PhaseName::Inhale => "INHALE",
            PhaseName::Hold => "HOLD",
            PhaseName::Exhale => "EXHALE",
            PhaseName::HoldAfterExhale => "REST",
        }
    }

    #[allow(dead_code)]
    pub fn default_instruction(&self) -> &'static str {
        match self {
            PhaseName::Inhale => "Breathe in slowly through your nose",
            PhaseName::Hold => "Hold your breath gently",
            PhaseName::Exhale => "Release slowly through your mouth",
            PhaseName::HoldAfterExhale => "Rest in the stillness",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Focus,
    Calm,
    Sleep,
    Energy,
    Recovery,
}

impl Category {
    #[allow(dead_code)]
    pub fn display(&self) -> &'static str {
        match self {
            Category::Focus => "Focus & Performance",
            Category::Calm => "Stress & Calm",
            Category::Sleep => "Sleep & Relaxation",
            Category::Energy => "Energy & Activation",
            Category::Recovery => "Recovery & Healing",
        }
    }

    #[allow(dead_code)]
    pub fn icon(&self) -> &'static str {
        match self {
            Category::Focus => "◎",
            Category::Calm => "○",
            Category::Sleep => "◐",
            Category::Energy => "◈",
            Category::Recovery => "◇",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    #[allow(dead_code)]
    pub fn display(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
        }
    }
}

/// A complete breathing technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technique {
    pub id: &'static str,
    pub name: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub pattern: &'static str,
    pub phases: Vec<Phase>,
    pub purpose: &'static str,
    pub use_case: &'static str,
    pub source: &'static str,
    pub color: TechniqueColor,
    pub default_cycles: u32,
    pub category: Category,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TechniqueColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TechniqueColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // Named colors matching the web app
    pub const fn arctic() -> Self { Self::new(74, 144, 217) }
    pub const fn gold() -> Self { Self::new(201, 162, 39) }
    pub const fn slate() -> Self { Self::new(100, 116, 139) }
    pub const fn purple() -> Self { Self::new(139, 92, 246) }
    pub const fn orange() -> Self { Self::new(251, 146, 60) }
    pub const fn emerald() -> Self { Self::new(34, 197, 94) }
    pub const fn rose() -> Self { Self::new(244, 63, 94) }
}

impl Technique {
    #[allow(dead_code)]
    pub fn cycle_duration(&self) -> f64 {
        self.phases.iter().map(|p| p.duration_secs).sum()
    }
}

/// All available breathing techniques
pub fn all_techniques() -> Vec<Technique> {
    vec![
        // ==========================================
        // FOCUS & PERFORMANCE
        // ==========================================
        Technique {
            id: "box",
            name: "Box Breathing",
            tagline: "Navy SEAL Standard",
            description: "The gold standard of tactical breathing. Equal parts inhale, hold, exhale, and hold create a \"box\" pattern that brings you to a state of alert calm.",
            pattern: "4-4-4-4",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Breathe In" },
                Phase { name: PhaseName::Hold, duration_secs: 4.0, instruction: "Hold" },
                Phase { name: PhaseName::Exhale, duration_secs: 4.0, instruction: "Breathe Out" },
                Phase { name: PhaseName::HoldAfterExhale, duration_secs: 4.0, instruction: "Hold Empty" },
            ],
            purpose: "Alert calm, mental clarity, stress inoculation",
            use_case: "Pre-performance, daily practice, high-pressure situations",
            source: "Navy SEAL standard, Mark Divine (SEALFIT)",
            color: TechniqueColor::arctic(),
            default_cycles: 5,
            category: Category::Focus,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "gateway",
            name: "Gateway Process",
            tagline: "CIA Declassified",
            description: "From declassified CIA documents. Developed at the Monroe Institute for intelligence applications. Achieves \"Focus 10\" state—mind awake, body asleep.",
            pattern: "4-4-8",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Deep Breath In" },
                Phase { name: PhaseName::Hold, duration_secs: 4.0, instruction: "Hold & Hum" },
                Phase { name: PhaseName::Exhale, duration_secs: 8.0, instruction: "Resonant Exhale" },
            ],
            purpose: "Enhanced focus, expanded awareness, mental clarity",
            use_case: "Deep concentration, meditation, problem-solving",
            source: "CIA/Monroe Institute, declassified 2003",
            color: TechniqueColor::slate(),
            default_cycles: 7,
            category: Category::Focus,
            difficulty: Difficulty::Intermediate,
        },
        Technique {
            id: "operative",
            name: "Operative Protocol",
            tagline: "Field Agent Standard",
            description: "Three-phase technique from declassified CIA training. Emphasizes exhale and post-exhale hold where best mental concentration is achieved.",
            pattern: "3-6-3",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 3.0, instruction: "Effortless Inhale" },
                Phase { name: PhaseName::Exhale, duration_secs: 6.0, instruction: "Controlled Exhale" },
                Phase { name: PhaseName::HoldAfterExhale, duration_secs: 3.0, instruction: "Focus Point" },
            ],
            purpose: "Tactical calmness, mental concentration under pressure",
            use_case: "High-stakes situations, crisis management",
            source: "CIA declassified training documents",
            color: TechniqueColor::slate(),
            default_cycles: 8,
            category: Category::Focus,
            difficulty: Difficulty::Intermediate,
        },
        Technique {
            id: "sere",
            name: "SERE Breathing",
            tagline: "Survival Training",
            description: "Core technique from Survival, Evasion, Resistance, and Escape training. Builds stress tolerance through controlled discomfort.",
            pattern: "4-7-8-4",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Controlled Inhale" },
                Phase { name: PhaseName::Hold, duration_secs: 7.0, instruction: "Stress Inoculation" },
                Phase { name: PhaseName::Exhale, duration_secs: 8.0, instruction: "Complete Release" },
                Phase { name: PhaseName::HoldAfterExhale, duration_secs: 4.0, instruction: "Empty Resilience" },
            ],
            purpose: "Stress inoculation, psychological resilience",
            use_case: "Extreme stress preparation, building mental toughness",
            source: "SERE Training Program, U.S. Military",
            color: TechniqueColor::gold(),
            default_cycles: 6,
            category: Category::Focus,
            difficulty: Difficulty::Advanced,
        },

        // ==========================================
        // STRESS & CALM
        // ==========================================
        Technique {
            id: "combat",
            name: "Combat Breathing",
            tagline: "Rapid Calm-Down",
            description: "Designed for rapid calm-down in high-stress situations. Extended exhale activates parasympathetic nervous system, dropping heart rate within seconds.",
            pattern: "4-1-8",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Breathe In" },
                Phase { name: PhaseName::Hold, duration_secs: 1.0, instruction: "Brief Pause" },
                Phase { name: PhaseName::Exhale, duration_secs: 8.0, instruction: "Slow Exhale" },
            ],
            purpose: "Rapid heart rate reduction, combat stress control",
            use_case: "Acute stress, panic moments, before confrontation",
            source: "U.S. Military Combat Stress Control",
            color: TechniqueColor::gold(),
            default_cycles: 6,
            category: Category::Calm,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "sigh",
            name: "Physiological Sigh",
            tagline: "Instant Calm Reset",
            description: "The fastest scientifically-proven way to reduce stress in real-time. Double inhale reinflates lung sacs, long exhale offloads CO2, triggering immediate calm.",
            pattern: "2-1-6",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 2.0, instruction: "Inhale (Nose)" },
                Phase { name: PhaseName::Inhale, duration_secs: 1.0, instruction: "Sip More Air" },
                Phase { name: PhaseName::Exhale, duration_secs: 6.0, instruction: "Long Exhale (Mouth)" },
            ],
            purpose: "Fastest real-time stress reduction",
            use_case: "Panic attacks, immediate relief, emotional reset",
            source: "Dr. Andrew Huberman, Stanford Neuroscience",
            color: TechniqueColor::arctic(),
            default_cycles: 3,
            category: Category::Calm,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "coherent",
            name: "Coherent Breathing",
            tagline: "Heart-Brain Sync",
            description: "Breathing at 5 breaths per minute synchronizes heart rate variability, creating \"coherence\" between heart and brain. Used by elite athletes.",
            pattern: "6-6",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 6.0, instruction: "Slow Inhale" },
                Phase { name: PhaseName::Exhale, duration_secs: 6.0, instruction: "Slow Exhale" },
            ],
            purpose: "Heart-brain coherence, HRV optimization",
            use_case: "Daily practice, emotional regulation, peak performance",
            source: "HeartMath Institute, Stephen Elliott",
            color: TechniqueColor::rose(),
            default_cycles: 10,
            category: Category::Calm,
            difficulty: Difficulty::Intermediate,
        },
        Technique {
            id: "resonant",
            name: "Resonant Breathing",
            tagline: "Vagal Tone Builder",
            description: "Optimizes vagal tone—the strength of your relaxation response. At 5-6 breaths per minute, cardiovascular system enters resonance.",
            pattern: "5-5",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 5.0, instruction: "Smooth Inhale" },
                Phase { name: PhaseName::Exhale, duration_secs: 5.0, instruction: "Smooth Exhale" },
            ],
            purpose: "Build long-term stress resilience",
            use_case: "Daily practice, vagal toning, PTSD recovery",
            source: "Dr. Richard Brown, Columbia University",
            color: TechniqueColor::emerald(),
            default_cycles: 12,
            category: Category::Calm,
            difficulty: Difficulty::Beginner,
        },

        // ==========================================
        // SLEEP & RELAXATION
        // ==========================================
        Technique {
            id: "military-sleep",
            name: "Military Sleep",
            tagline: "2-Minute Sleep Technique",
            description: "Developed for fighter pilots to fall asleep in 2 minutes under any conditions. Used by 96% of pilots after 6 weeks of practice.",
            pattern: "4-7-8",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Deep Breath In" },
                Phase { name: PhaseName::Hold, duration_secs: 7.0, instruction: "Hold & Relax Face" },
                Phase { name: PhaseName::Exhale, duration_secs: 8.0, instruction: "Release Everything" },
            ],
            purpose: "Fall asleep in under 2 minutes",
            use_case: "Insomnia, sleeping in difficult conditions, jet lag",
            source: "U.S. Navy Pre-Flight School, Bud Winter",
            color: TechniqueColor::purple(),
            default_cycles: 6,
            category: Category::Sleep,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "478",
            name: "4-7-8 Breathing",
            tagline: "Natural Tranquilizer",
            description: "A powerful relaxation technique that acts as a natural tranquilizer for the nervous system. Long hold and exhale shift body into deep rest mode.",
            pattern: "4-7-8",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Breathe In" },
                Phase { name: PhaseName::Hold, duration_secs: 7.0, instruction: "Hold" },
                Phase { name: PhaseName::Exhale, duration_secs: 8.0, instruction: "Breathe Out" },
            ],
            purpose: "Deep relaxation, nervous system reset",
            use_case: "Pre-sleep routine, anxiety relief, wind-down",
            source: "Dr. Andrew Weil (based on yogic pranayama)",
            color: TechniqueColor::purple(),
            default_cycles: 4,
            category: Category::Sleep,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "sleep-exhale",
            name: "Sleep Exhale",
            tagline: "Extended Exhale Sleep",
            description: "Emphasizes very long exhale to maximally activate parasympathetic \"rest and digest\" response. 2:1 exhale-to-inhale ratio signals deep safety.",
            pattern: "4-2-8-2",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Gentle Inhale" },
                Phase { name: PhaseName::Hold, duration_secs: 2.0, instruction: "Soft Hold" },
                Phase { name: PhaseName::Exhale, duration_secs: 8.0, instruction: "Long Slow Exhale" },
                Phase { name: PhaseName::HoldAfterExhale, duration_secs: 2.0, instruction: "Rest Empty" },
            ],
            purpose: "Maximum relaxation, parasympathetic activation",
            use_case: "Deep insomnia, racing thoughts, nighttime anxiety",
            source: "Clinical sleep research",
            color: TechniqueColor::purple(),
            default_cycles: 8,
            category: Category::Sleep,
            difficulty: Difficulty::Beginner,
        },

        // ==========================================
        // ENERGY & ACTIVATION
        // ==========================================
        Technique {
            id: "energize",
            name: "Energizing Breath",
            tagline: "Natural Energy Surge",
            description: "Controlled hyperventilation that boosts oxygen levels and triggers adrenaline release. Creates natural energy surge without caffeine.",
            pattern: "1-1",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 1.0, instruction: "Quick Inhale" },
                Phase { name: PhaseName::Exhale, duration_secs: 1.0, instruction: "Quick Exhale" },
            ],
            purpose: "Alertness, energy boost, wake-up",
            use_case: "Morning activation, pre-workout, afternoon slump",
            source: "Modified from Wim Hof & Kapalabhati",
            color: TechniqueColor::orange(),
            default_cycles: 30,
            category: Category::Energy,
            difficulty: Difficulty::Intermediate,
        },
        Technique {
            id: "power",
            name: "Power Breathing",
            tagline: "Pre-Mission Activation",
            description: "Used by special operators before missions. Builds energy through breath holds that trigger adrenaline, then channels it with controlled exhales.",
            pattern: "4-4-4",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Power Inhale" },
                Phase { name: PhaseName::Hold, duration_secs: 4.0, instruction: "Build Energy" },
                Phase { name: PhaseName::Exhale, duration_secs: 4.0, instruction: "Channel Power" },
            ],
            purpose: "Peak activation, mental intensity, pre-performance",
            use_case: "Before competition, presentations, physical challenges",
            source: "Special Operations performance protocols",
            color: TechniqueColor::orange(),
            default_cycles: 6,
            category: Category::Energy,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "wim-hof",
            name: "Wim Hof Method",
            tagline: "The Iceman Protocol",
            description: "Famous technique from \"The Iceman.\" 30 power breaths create massive oxygen saturation and controlled stress exposure, building mental resilience.",
            pattern: "2-1",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 2.0, instruction: "Full Breath In" },
                Phase { name: PhaseName::Exhale, duration_secs: 1.0, instruction: "Let Go" },
            ],
            purpose: "Immune boost, cold tolerance, mental fortitude",
            use_case: "Morning practice, cold exposure prep, stress inoculation",
            source: "Wim Hof, validated by Radboud University",
            color: TechniqueColor::arctic(),
            default_cycles: 30,
            category: Category::Energy,
            difficulty: Difficulty::Advanced,
        },

        // ==========================================
        // RECOVERY & HEALING
        // ==========================================
        Technique {
            id: "recovery",
            name: "Recovery Breathing",
            tagline: "Post-Stress Recovery",
            description: "Designed for recovery after intense physical or mental stress. Longer exhales and holds maximize parasympathetic recovery and reduce cortisol.",
            pattern: "4-2-6-4",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Recovery Breath" },
                Phase { name: PhaseName::Hold, duration_secs: 2.0, instruction: "Brief Hold" },
                Phase { name: PhaseName::Exhale, duration_secs: 6.0, instruction: "Release Tension" },
                Phase { name: PhaseName::HoldAfterExhale, duration_secs: 4.0, instruction: "Deep Rest" },
            ],
            purpose: "Cortisol reduction, nervous system recovery",
            use_case: "Post-workout, after stressful events, evening wind-down",
            source: "Sports science recovery protocols",
            color: TechniqueColor::emerald(),
            default_cycles: 8,
            category: Category::Recovery,
            difficulty: Difficulty::Beginner,
        },
        Technique {
            id: "nsdr",
            name: "NSDR Breathing",
            tagline: "Non-Sleep Deep Rest",
            description: "Breathing pattern for Non-Sleep Deep Rest, providing recovery benefits similar to sleep. Achieves deep relaxation while maintaining awareness.",
            pattern: "4-6-6",
            phases: vec![
                Phase { name: PhaseName::Inhale, duration_secs: 4.0, instruction: "Gentle Inhale" },
                Phase { name: PhaseName::Hold, duration_secs: 6.0, instruction: "Restful Hold" },
                Phase { name: PhaseName::Exhale, duration_secs: 6.0, instruction: "Melting Exhale" },
            ],
            purpose: "Deep rest without sleep, recovery, focus restoration",
            use_case: "Afternoon recharge, sleep debt recovery, mental reset",
            source: "Dr. Andrew Huberman, Stanford protocols",
            color: TechniqueColor::purple(),
            default_cycles: 10,
            category: Category::Recovery,
            difficulty: Difficulty::Beginner,
        },
    ]
}

pub fn get_technique(id: &str) -> Option<Technique> {
    all_techniques().into_iter().find(|t| t.id == id)
}

#[allow(dead_code)]
pub fn get_techniques_by_category(category: Category) -> Vec<Technique> {
    all_techniques().into_iter().filter(|t| t.category == category).collect()
}

#[allow(dead_code)]
pub fn all_categories() -> Vec<Category> {
    vec![
        Category::Focus,
        Category::Calm,
        Category::Sleep,
        Category::Energy,
        Category::Recovery,
    ]
}
