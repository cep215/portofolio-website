/// All editable site content in one place.
/// Change URLs, text, and metadata here — no need to touch components.

// ── Identity ───────────────────────────────────────────────
pub const NAME: &str = "Catalina Popescu";
pub const ROLE: &str = "Founder & CEO @ Lawren AI";
pub const HERO_IMAGE: &str = "images/hero_image.png";

// ── Links ──────────────────────────────────────────────────
pub const EMAIL: &str = "mailto:catalina@lawren.ai";
pub const LINKEDIN_URL: &str = "https://linkedin.com/in/popescu15";
pub const GITHUB_URL: &str = "https://github.com/cep215";
pub const LAWREN_URL: &str = "#";
pub const PDF_URL: &str = "#";

// ── About ──────────────────────────────────────────────────
pub const HERO_ONELINER: &str =
    "Lawren builds source-grounded legal intelligence for Romanian legislation: \
     chat + document workflows + RAG.";

pub const WHAT_I_DO: &[&str] = &[
    "I design and ship RAG-first, source-grounded workflows for regulated decision-making.",
    "I run product + GTM: roadmap, pilots, pricing, and enterprise adoption.",
    "I translate messy domains into reliable systems\u{2014}data, automation, and measurable outcomes.",
];

// ── Quick Facts (sidebar) ──────────────────────────────────
pub struct QuickFact {
    pub label: &'static str,
    pub value: &'static str,
}

pub const QUICK_FACTS: &[QuickFact] = &[
    QuickFact {
        label: "Location",
        value: "Bucharest, RO",
    },
    QuickFact {
        label: "Education",
        value: "Imperial College London",
    },
    QuickFact {
        label: "Focus",
        value: "Legal AI / NLP",
    },
];

// ── Skills ─────────────────────────────────────────────────
pub const SKILLS: &[&str] = &[
    "RAG",
    "NLP",
    "Python",
    "Rust",
    "LLMs",
    "Product Strategy",
    "GTM",
    "Enterprise Sales",
];

// ── Featured Work ──────────────────────────────────────────
pub struct WorkCard {
    pub title: &'static str,
    pub description: &'static str,
    pub bullets: &'static [&'static str],
    pub links: &'static [(&'static str, &'static str)],
    pub chips: &'static [&'static str],
    pub metrics: &'static [&'static str],
}

pub const FEATURED_WORK: &[WorkCard] = &[
    WorkCard {
        title: "Lawren AI",
        description: "Source-grounded legal intelligence for Romanian legislation.",
        bullets: &[
            "RAG-powered chat over the full Romanian legal corpus",
            "Document workflows: summarisation, clause extraction, compliance checks",
            "Enterprise-ready: SSO, audit logs, on-prem deployment options",
        ],
        links: &[("Site", "#"), ("Demo", "#"), ("Deck", "#")],
        chips: &["RAG", "NLP", "LLMs", "Python"],
        metrics: &["10K+ documents indexed", "3 enterprise pilots"],
    },
    WorkCard {
        title: "PSC Meta \u{2014} Waste-to-Energy / Biogas",
        description:
            "Founded and lead a Romanian waste management & biogas consultancy; \
             industrial automation + process optimization for waste-to-energy.",
        bullets: &[
            "End-to-end project management for biogas & waste-processing facilities",
            "Process automation reducing operational costs by 30%+",
        ],
        links: &[("Learn more", "#")],
        chips: &["Automation", "Process Optimization", "Biogas"],
        metrics: &["30%+ cost reduction"],
    },
    WorkCard {
        title: "Research / Builds",
        description: "Academic and independent technical work.",
        bullets: &[
            "Imperial College London dissertation on applied ML for financial markets",
        ],
        links: &[("Dissertation", "#")],
        chips: &["ML", "Finance", "Research"],
        metrics: &[],
    },
];

// ── Experience ─────────────────────────────────────────────
pub struct ExperienceEntry {
    pub company: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub note: &'static str,
    pub tags: &'static [&'static str],
    pub outcomes: &'static [&'static str],
}

pub const EXPERIENCE: &[ExperienceEntry] = &[
    ExperienceEntry {
        company: "Lawren AI",
        role: "Co-founder & CEO",
        period: "2024 \u{2013} Present",
        note: "",
        tags: &["AI", "Legal Tech", "NLP"],
        outcomes: &[
            "Launched RAG platform for Romanian legal corpus",
            "Secured enterprise pilots",
        ],
    },
    ExperienceEntry {
        company: "PSC Meta",
        role: "Founder / CEO",
        period: "2022 \u{2013} Present",
        note: "",
        tags: &["Waste-to-Energy", "Automation"],
        outcomes: &["30%+ operational cost reduction through automation"],
    },
    ExperienceEntry {
        company: "Two Sigma",
        role: "Quant Research Consultant",
        period: "2020 \u{2013} 2023",
        note: "",
        tags: &["Quant", "Research"],
        outcomes: &[],
    },
    ExperienceEntry {
        company: "WorldQuant",
        role: "Quant Researcher",
        period: "2019 \u{2013} 2020",
        note: "",
        tags: &["Quant", "Finance"],
        outcomes: &[],
    },
    ExperienceEntry {
        company: "Microsoft / Deutsche Bank",
        role: "Internships",
        period: "",
        note: "Software engineering & quantitative finance internships",
        tags: &["SWE", "Finance"],
        outcomes: &[],
    },
];

// ── Proof / Links Strip ────────────────────────────────────
pub struct ProofLink {
    pub label: &'static str,
    pub url: &'static str,
}

pub const PROOF_LINKS: &[ProofLink] = &[
    ProofLink {
        label: "Publications",
        url: "#",
    },
    ProofLink {
        label: "Press",
        url: "#",
    },
    ProofLink {
        label: "Talks & Demos",
        url: "#",
    },
];

// ── Contact ────────────────────────────────────────────────
pub const CONTACT_CTA: &str = "Open to pilots/partnerships + investor conversations.";
pub const CONTACT_AVAILABILITY: &str =
    "Available for calls \u{2014} Bucharest timezone (EET)";
