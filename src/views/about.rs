use dioxus::prelude::*;

const ABOUT_CSS: Asset = asset!("../../assets/about.css");

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
                    div {
                        div { class: "about-item-details",
                            div { class: "about-item-header",
                                div {
                                    span { class: "about-item-name", "Gavel" }
                                    small { class: "about-item-subtitle", "Senior Software Engineer" }
                                }
                                span { class: "about-item-date", "08/2023 — 02/2024" }
                            }
                            p { class: "about-item-summary",
                                "Developed customer-facing document automation tools for a legal technology platform, enabling law firms and legal professionals to streamline client intake, automate document preparation, and generate customized legal documents."
                            }
                            span { class: "about-item-technologies", "Technologies: "
                                span { class: "technology-tag", "Typescript" }
                                span { class: "technology-tag", "React" }
                                span { class: "technology-tag", "Node.js" }
                                span { class: "technology-tag", "Ruby on Rails" }
                                span { class: "technology-tag", "Python" }
                            }
                        }
                    }
                    div {
                        div { class: "about-item-details",
                            div { class: "about-item-header",
                                div {
                                    span { class: "about-item-name", "Glue" }
                                    small { class: "about-item-subtitle", "Senior Software Engineer" }
                                }
                                span { class: "about-item-date", "05/2022 — 06/2023" }
                            }
                            p { class: "about-item-summary",
                                "Developed a customer-facing employee engagement platform that helps organizations strengthen workplace relationships through team-building events, organizational relationship analysis, and data-driven engagement insights."
                            }
                            span { class: "about-item-technologies", "Technologies: "
                                span { class: "technology-tag", "Typescript" }
                                span { class: "technology-tag", "React" }
                                span { class: "technology-tag", "Node.js" }
                                span { class: "technology-tag", "GraphQL" }
                                span { class: "technology-tag", "PostgreSQL" }
                                span { class: "technology-tag", "D3.js" }
                            }
                        }
                    }
                    div {
                        div { class: "about-item-details",
                            div { class: "about-item-header",
                                div {
                                    span { class: "about-item-name", "Scalable Press" }
                                    small { class: "about-item-subtitle", "Software Engineer" }
                                }
                                span { class: "about-item-date", "03/2017 — 05/2022" }
                            }
                            p { class: "about-item-summary",
                                "Developed customer-facing e-commerce platforms for custom commercial printing, alongside internal inventory and production management systems."
                            }
                            span { class: "about-item-technologies", "Technologies: "
                                span { class: "technology-tag", "Typescript" }
                                span { class: "technology-tag", "CoffeeScript" }
                                span { class: "technology-tag", "React" }
                                span { class: "technology-tag", "AngularJS" }
                                span { class: "technology-tag", "Node.js" }
                                span { class: "technology-tag", "GraphQL" }
                                span { class: "technology-tag", "MongoDB" }
                                span { class: "technology-tag", "Redis" }
                                span { class: "technology-tag", "Jenkins" }
                                span { class: "technology-tag", "RabbitMQ" }
                                span { class: "technology-tag", "PHP" }
                            }
                        }
                    }
                    div {
                        div { class: "about-item-details",
                            div { class: "about-item-header",
                                div {
                                    span { class: "about-item-name", "Asurion" }
                                    small { class: "about-item-subtitle", "Software Engineer" }
                                }
                                span { class: "about-item-date", "06/2013 — 04/2014" }
                            }
                            p { class: "about-item-summary",
                                "Developed internal customer service applications for Asurion's mobile device protection business, supporting customer representatives in processing insurance claims, managing claim information, and coordinating device repair and replacement workflows."
                            }
                            span { class: "about-item-technologies", "Technologies: "
                                span { class: "technology-tag", "Javascript" }
                                span { class: "technology-tag", "Knockout.js" }
                                span { class: "technology-tag", "C#" }
                                span { class: "technology-tag", ".NET" }
                            }
                        }
                    }
                }
            }

            hr { }

            section {
                h2 { "Education" }
                div { class: "about-list",
                    div {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Houston City College" }
                            small { class: "about-item-subtitle", "Associate of Applied Science in Cybersecurity" }
                        }
                        span { class: "about-item-date", "01/2025 — Present" }
                    }
                    div {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Emory University" }
                            small { class: "about-item-subtitle", "Master of Science in Computer Science" }
                        }
                        span { class: "about-item-date", "08/2014 — 12/2016" }
                    }
                    div {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Vanderbilt University" }
                            small { class: "about-item-subtitle", "Bachelor of Science in Computer Science and Mathematics" }
                        }
                        span { class: "about-item-date", "08/2009 — 05/2013" }
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
                    div {
                        span { class: "about-item-name", "AWS Certified DevOps Engineer – Professional" }
                        span { class: "about-item-date", "11/2026" }
                    }
                    div {
                        span { class: "about-item-name", "AWS Certified Security – Specialty" }
                        span { class: "about-item-date", "10/2026" }
                    }
                    div {
                        span { class: "about-item-name", "AWS Certified Solutions Architect – Associate" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    div {
                        span { class: "about-item-name", "HashiCorp Certified: Terraform Associate" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    div {
                        span { class: "about-item-name", "Red Hat Certified Engineer in Ansible" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    div {
                        span { class: "about-item-name", "Red Hat Certified System Administrator" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    div {
                        span { class: "about-item-name", "CompTIA Security+" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    div {
                        span { class: "about-item-name", "Certified Kubernetes Security Specialist" }
                        span { class: "about-item-date", "08/2026" }
                    }
                    div {
                        span { class: "about-item-name", "Certified Kubernetes Administrator" }
                        span { class: "about-item-date", "08/2026" }
                    }
                    div {
                        span { class: "about-item-name", "Cisco Certified Network Associate" }
                        span { class: "about-item-date", "06/2026" }
                    }
                }
            }
        }
    }
}
