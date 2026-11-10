use dioxus::prelude::*;

const ABOUT_CSS: Asset = asset!("../../assets/about.css");

struct Experience {
    company: &'static str,
    role: &'static str,
    dates: &'static str,
    summary: &'static str,
    technologies: &'static [&'static str],
}

const EXPERIENCE: &[Experience] = &[
    Experience {
        company: "Gavel",
        role: "Senior Software Engineer",
        dates: "08/2023 — 02/2024",
        summary: "Developed customer-facing document automation tools for a legal technology platform, enabling law firms and legal professionals to streamline client intake, automate document preparation, and generate customized legal documents.",
        technologies: &["Typescript", "React", "Node.js", "Ruby on Rails", "Python"],
    },
    Experience {
        company: "Glue",
        role: "Senior Software Engineer",
        dates: "05/2022 — 06/2023",
        summary: "Developed a customer-facing employee engagement platform that helps organizations strengthen workplace relationships through team-building events, organizational relationship analysis, and data-driven engagement insights.",
        technologies: &[
            "Typescript",
            "React",
            "Node.js",
            "GraphQL",
            "PostgreSQL",
            "D3.js",
        ],
    },
    Experience {
        company: "Scalable Press",
        role: "Software Engineer",
        dates: "03/2017 — 05/2022",
        summary: "Developed customer-facing e-commerce platforms for custom commercial printing, alongside internal inventory and production management systems.",
        technologies: &[
            "Typescript",
            "CoffeeScript",
            "React",
            "AngularJS",
            "Node.js",
            "GraphQL",
            "MongoDB",
            "Redis",
            "Jenkins",
            "RabbitMQ",
            "PHP",
        ],
    },
    Experience {
        company: "Asurion",
        role: "Software Engineer",
        dates: "06/2013 — 04/2014",
        summary: "Developed internal customer service applications for Asurion's mobile device protection business, supporting customer representatives in processing insurance claims, managing claim information, and coordinating device repair and replacement workflows.",
        technologies: &["Javascript", "Knockout.js", "C#", ".NET"],
    },
];

struct Education {
    institution: &'static str,
    degree: &'static str,
    dates: &'static str,
}

const EDUCATION: &[Education] = &[
    Education {
        institution: "Houston City College",
        degree: "Associate of Applied Science in Cybersecurity",
        dates: "01/2025 — Present",
    },
    Education {
        institution: "Emory University",
        degree: "Master of Science in Computer Science",
        dates: "08/2014 — 12/2016",
    },
    Education {
        institution: "Vanderbilt University",
        degree: "Bachelor of Science in Computer Science and Mathematics",
        dates: "08/2009 — 05/2013",
    },
];

struct Certification {
    name: &'static str,
    date: &'static str,
}

const CERTIFICATIONS: &[Certification] = &[
    Certification {
        name: "AWS Certified DevOps Engineer – Professional",
        date: "11/2026",
    },
    Certification {
        name: "AWS Certified Security – Specialty",
        date: "10/2026",
    },
    Certification {
        name: "AWS Certified Solutions Architect – Associate",
        date: "09/2026",
    },
    Certification {
        name: "HashiCorp Certified: Terraform Associate",
        date: "09/2026",
    },
    Certification {
        name: "Red Hat Certified Engineer in Ansible",
        date: "09/2026",
    },
    Certification {
        name: "Red Hat Certified System Administrator",
        date: "09/2026",
    },
    Certification {
        name: "CompTIA Security+",
        date: "09/2026",
    },
    Certification {
        name: "Certified Kubernetes Security Specialist",
        date: "08/2026",
    },
    Certification {
        name: "Certified Kubernetes Administrator",
        date: "08/2026",
    },
    Certification {
        name: "Cisco Certified Network Associate",
        date: "06/2026",
    },
];

#[component]
pub fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ABOUT_CSS }
        document::Title { "About Rui" }

        div {
            class: "about-page",

            h1 { "About Me" }

            hr { }

            section {
                h2 { "Experience" }
                div { class: "about-list about-experience",
                    for experience in EXPERIENCE {
                        div { key: "{experience.company}",
                            div { class: "about-item-details",
                                div { class: "about-item-header",
                                    div {
                                        span { class: "about-item-name", "{experience.company}" }
                                        small { class: "about-item-subtitle", "{experience.role}" }
                                    }
                                    span { class: "about-item-date", "{experience.dates}" }
                                }
                                p { class: "about-item-summary", "{experience.summary}" }
                                span { class: "technology-tag-list",
                                    for technology in experience.technologies {
                                        span { key: "{technology}", class: "technology-tag", "{technology}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            hr { }

            section {
                h2 { "Education" }
                div { class: "about-list",
                    for education in EDUCATION {
                        div { key: "{education.institution}",
                            span { class: "about-item-details",
                                span { class: "about-item-name", "{education.institution}" }
                                small { class: "about-item-subtitle", "{education.degree}" }
                            }
                            span { class: "about-item-date", "{education.dates}" }
                        }
                    }
                }
            }

            hr { }

            section {
                h2 { "Certifications" }
                div {
                  a {
                    href: "https://www.credly.com/users/rui-jiang.5598bd5c/badges/credly",
                    target: "_blank",
                    "aria-label": "link to credly account",
                    "Credly Account ↗"
                  }
                }
                br { }
                div { class: "about-list",
                    for certification in CERTIFICATIONS {
                        div { key: "{certification.name}",
                            span { class: "about-item-name", "{certification.name}" }
                            span { class: "about-item-date", "{certification.date}" }
                        }
                    }
                }
            }
        }
    }
}
