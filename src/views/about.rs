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
            p { "A brief timeline of my professional background and learning." }

            section {
                h2 { "Experience" }
                ul { class: "about-list",
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Gavel" }
                            small { class: "about-item-subtitle", "Senior Software Engineer" }
                            span { class: "about-item-technologies", "aria-label": "Technologies",
                                span { class: "about-technology", "Typescript" }
                                span { class: "about-technology", "React" }
                                span { class: "about-technology", "Node.js" }
                                span { class: "about-technology", "Ruby on Rails" }
                                span { class: "about-technology", "Python" }
                            }
                        }
                        span { class: "about-item-date", "08/2023 — 02/2024" }
                    }
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Glue" }
                            small { class: "about-item-subtitle", "Senior Software Engineer" }
                            span { class: "about-item-technologies", "aria-label": "Technologies",
                                span { class: "about-technology", "Typescript" }
                                span { class: "about-technology", "React" }
                                span { class: "about-technology", "Node.js" }
                                span { class: "about-technology", "GraphQL" }
                                span { class: "about-technology", "PostgreSQL" }
                                span { class: "about-technology", "D3.js" }
                            }
                        }
                        span { class: "about-item-date", "05/2022 — 06/2023" }
                    }
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Scalable Press" }
                            small { class: "about-item-subtitle", "Software Engineer" }
                            span { class: "about-item-technologies", "aria-label": "Technologies",
                                span { class: "about-technology", "Typescript" }
                                span { class: "about-technology", "CoffeeScript" }
                                span { class: "about-technology", "React" }
                                span { class: "about-technology", "AngularJS" }
                                span { class: "about-technology", "Node.js" }
                                span { class: "about-technology", "GraphQL" }
                                span { class: "about-technology", "MongoDB" }
                                span { class: "about-technology", "Redis" }
                                span { class: "about-technology", "RabbitMQ" }
                                span { class: "about-technology", "php" }
                            }
                        }
                        span { class: "about-item-date", "01/2017 — 05/2022" }
                    }
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Asurion" }
                            small { class: "about-item-subtitle", "Software Engineer" }
                            span { class: "about-item-technologies", "aria-label": "Technologies",
                                span { class: "about-technology", "Javascript" }
                                span { class: "about-technology", "Knockout.js" }
                                span { class: "about-technology", "C#" }
                                span { class: "about-technology", ".NET" }
                            }
                        }
                        span { class: "about-item-date", "06/2013 — 04/2014" }
                    }
                }
            }

            section {
                h2 { "Certifications" }
                ul { class: "about-list",
                    li {
                        span { class: "about-item-name", "HashiCorp Certified Terraform Associate" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    li {
                        span { class: "about-item-name", "Red Hat Certified Engineer in Ansible" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    li {
                        span { class: "about-item-name", "Red Hat Certified System Administrator" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    li {
                        span { class: "about-item-name", "CompTIA Security+" }
                        span { class: "about-item-date", "09/2026" }
                    }
                    li {
                        span { class: "about-item-name", "Certified Kubernetes Security Specialist" }
                        span { class: "about-item-date", "08/2026" }
                    }
                    li {
                        span { class: "about-item-name", "Certified Kubernetes Administrator" }
                        span { class: "about-item-date", "08/2026" }
                    }
                    li {
                        span { class: "about-item-name", "Cisco Certified Network Associate" }
                        span { class: "about-item-date", "06/2026" }
                    }
                }
            }

            section {
                h2 { "Education" }
                ul { class: "about-list",
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Houston City College" }
                            small { class: "about-item-subtitle", "Associate of Applied Science in Cyber Security" }
                        }
                        span { class: "about-item-date", "01/2025 — Present" }
                    }
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Emory University" }
                            small { class: "about-item-subtitle", "Master of Science in Computer Science" }
                        }
                        span { class: "about-item-date", "08/2014 — 12/2016" }
                    }
                    li {
                        span { class: "about-item-details",
                            span { class: "about-item-name", "Vanderbilt University" }
                            small { class: "about-item-subtitle", "Bachelor of Science in Mathematics and Computer Science" }
                        }
                        span { class: "about-item-date", "08/2009 — 05/2013" }
                    }
                }
            }
        }
    }
}
