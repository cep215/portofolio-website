mod content;

use leptos::prelude::*;
use thaw::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <ConfigProvider>
                <App />
            </ConfigProvider>
        }
    });
}

// ── Root ───────────────────────────────────────────────────

#[component]
fn App() -> impl IntoView {
    view! {
        <SiteHeader />
        <div class="page-layout">
            <aside class="sidebar">
                <ProfileCard />
            </aside>
            <main class="content">
                <AboutSection />
                <FeaturedWorkSection />
                <ExperienceSection />
                <ProofStripSection />
                <ContactSection />
            </main>
        </div>
        <SiteFooter />
    }
}

// ── Header ─────────────────────────────────────────────────

#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="header-inner">
                <a href="#" class="logo">{content::NAME}</a>
                <nav class="main-nav">
                    <a href="#about">"About"</a>
                    <a href="#work">"Work"</a>
                    <a href="#experience">"Experience"</a>
                    <a href="#contact">"Contact"</a>
                </nav>
                <div class="header-actions">
                    <a
                        href={content::PDF_URL}
                        class="pdf-link"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "Download PDF"
                    </a>
                    <a href={content::EMAIL} title="Email" class="icon-btn">
                        "@"
                    </a>
                    <a
                        href={content::LINKEDIN_URL}
                        target="_blank"
                        rel="noopener noreferrer"
                        title="LinkedIn"
                        class="icon-btn"
                    >
                        "in"
                    </a>
                    <a
                        href={content::GITHUB_URL}
                        target="_blank"
                        rel="noopener noreferrer"
                        title="GitHub"
                        class="icon-btn"
                    >
                        "GH"
                    </a>
                </div>
            </div>
        </header>
    }
}

// ── Profile Card (sidebar) ─────────────────────────────────

#[component]
fn ProfileCard() -> impl IntoView {
    view! {
        <div class="profile-card">
            <img
                src={content::HERO_IMAGE}
                alt={content::NAME}
                class="profile-avatar"
            />
            <h1 class="profile-name">{content::NAME}</h1>
            <p class="profile-role">{content::ROLE}</p>

            <div class="profile-ctas">
                <a href={content::EMAIL} class="cta-primary">
                    "Get in touch"
                </a>
                <a
                    href={content::LAWREN_URL}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="cta-secondary"
                >
                    "See Lawren"
                </a>
            </div>

            <Divider />

            <div class="quick-facts">
                {content::QUICK_FACTS
                    .iter()
                    .map(|fact| {
                        view! {
                            <div class="fact-row">
                                <span class="fact-label">{fact.label}</span>
                                <span class="fact-value">{fact.value}</span>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <Divider />

            <div class="skill-chips">
                {content::SKILLS
                    .iter()
                    .map(|skill| {
                        view! { <span class="chip">{*skill}</span> }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

// ── About ──────────────────────────────────────────────────

#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <section class="section" id="about">
            <h2 class="section-title">"About"</h2>
            <p class="about-oneliner">{content::HERO_ONELINER}</p>
            <div class="about-items">
                {content::WHAT_I_DO
                    .iter()
                    .map(|item| {
                        view! { <p class="about-item">{*item}</p> }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

// ── Featured Work ──────────────────────────────────────────

#[component]
fn FeaturedWorkSection() -> impl IntoView {
    view! {
        <section class="section" id="work">
            <h2 class="section-title">"Featured Work"</h2>
            <div class="work-cards">
                {content::FEATURED_WORK
                    .iter()
                    .map(|card| {
                        view! {
                            <div class="work-card">
                                <div class="work-card-header">
                                    <h3 class="work-card-title">{card.title}</h3>
                                    <div class="work-chips">
                                        {card
                                            .chips
                                            .iter()
                                            .map(|c| {
                                                view! { <span class="chip chip-sm">{*c}</span> }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </div>
                                <p class="work-desc">{card.description}</p>
                                {(!card.metrics.is_empty())
                                    .then(|| {
                                        view! {
                                            <div class="work-metrics">
                                                {card
                                                    .metrics
                                                    .iter()
                                                    .map(|m| {
                                                        view! { <span class="metric">{*m}</span> }
                                                    })
                                                    .collect::<Vec<_>>()}
                                            </div>
                                        }
                                    })}
                                <ul class="work-bullets">
                                    {card
                                        .bullets
                                        .iter()
                                        .map(|b| view! { <li>{*b}</li> })
                                        .collect::<Vec<_>>()}
                                </ul>
                                <div class="work-links">
                                    {card
                                        .links
                                        .iter()
                                        .map(|(label, url)| {
                                            view! {
                                                <a
                                                    href={*url}
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="work-link"
                                                >
                                                    {*label}
                                                </a>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

// ── Experience ─────────────────────────────────────────────

#[component]
fn ExperienceSection() -> impl IntoView {
    view! {
        <section class="section" id="experience">
            <h2 class="section-title">"Experience"</h2>
            <div class="exp-list">
                {content::EXPERIENCE
                    .iter()
                    .map(|entry| {
                        view! {
                            <div class="exp-entry">
                                <div class="exp-main">
                                    <div class="exp-left">
                                        <h3 class="exp-company">{entry.company}</h3>
                                        <span class="exp-role">{entry.role}</span>
                                        {(!entry.tags.is_empty())
                                            .then(|| {
                                                view! {
                                                    <div class="exp-tags">
                                                        {entry
                                                            .tags
                                                            .iter()
                                                            .map(|t| {
                                                                view! { <span class="tag">{*t}</span> }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </div>
                                                }
                                            })}
                                    </div>
                                    <div class="exp-right">
                                        {(!entry.period.is_empty())
                                            .then(|| {
                                                view! {
                                                    <span class="exp-period">{entry.period}</span>
                                                }
                                            })}
                                    </div>
                                </div>
                                {(!entry.note.is_empty())
                                    .then(|| {
                                        view! { <p class="exp-note">{entry.note}</p> }
                                    })}
                                {(!entry.outcomes.is_empty())
                                    .then(|| {
                                        view! {
                                            <ul class="exp-outcomes">
                                                {entry
                                                    .outcomes
                                                    .iter()
                                                    .map(|o| view! { <li>{*o}</li> })
                                                    .collect::<Vec<_>>()}
                                            </ul>
                                        }
                                    })}
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

// ── Proof Strip ────────────────────────────────────────────

#[component]
fn ProofStripSection() -> impl IntoView {
    view! {
        <section class="section proof-section">
            <div class="proof-pills">
                {content::PROOF_LINKS
                    .iter()
                    .map(|link| {
                        view! {
                            <a
                                href={link.url}
                                class="proof-pill"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {link.label}
                            </a>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

// ── Contact ────────────────────────────────────────────────

#[component]
fn ContactSection() -> impl IntoView {
    view! {
        <section class="section" id="contact">
            <h2 class="section-title">"Contact"</h2>
            <p class="contact-cta">{content::CONTACT_CTA}</p>
            <p class="contact-availability">{content::CONTACT_AVAILABILITY}</p>
            <div class="contact-actions">
                <a href={content::EMAIL} class="cta-primary">
                    "Email me"
                </a>
                <a
                    href={content::LINKEDIN_URL}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="cta-secondary"
                >
                    "LinkedIn"
                </a>
            </div>
        </section>
    }
}

// ── Footer ─────────────────────────────────────────────────

#[component]
fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <p>{format!("\u{00a9} {} {}", 2025, content::NAME)}</p>
        </footer>
    }
}
