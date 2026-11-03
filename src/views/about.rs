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
                            ul { class: "about-item-achievements",
                                li { "Developed an AI-powered data extraction workflow using GPT-4 for a legal document automation platform; built a benchmarking tool to evaluate prompt and model performance, accelerating iteration and improving the clarity and accuracy of generated outputs." }
                                li { "Redesigned the new-user registration experience to reduce onboarding friction and improve usability, contributing to increased user acquisition and sign-up conversion." }
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
                            ul { class: "about-item-achievements",
                                li { "Developed high-performance employee data visualizations using D3.js, engineering scalable data structures to model organizational hierarchies and enable interactive analysis of workforce engagement." }
                                li { "Redesigned the customer self-service experience, combining an intuitive checkout workflow with an enhanced event catalog featuring advanced filtering and sorting." }
                                li { "Developed an interactive scheduling system that dynamically identifies optimal time slots, integrating GraphQL-based state management with continuous backend synchronization." }
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
                            ul { class: "about-item-achievements",
                                li { "Architected a scalable, RabbitMQ-powered image processing pipeline handling 50K daily jobs and scaling to 500K+ per day during peak seasons to generate customer-specific product mockups." }
                                li { "Designed and implemented an organization-wide identity and access management system combining DigitalPersona fingerprint authentication with a hierarchical RBAC model, enforcing secure access validation and granular permissions across warehouse production, customer service, sales, and management." }
                                li { "Engineered an embroidery production integration pipeline that parses industry-standard embroidery file formats into internal production data structures, alongside a customer-facing frontend for editing and fine-tuning embroidery designs." }
                                li { "Integrated the Namecheap API to automate asynchronous domain provisioning within the customer platform, encompassing domain validation, registration, email verification, DNS configuration, SSL certificate provisioning, and routing to AWS-hosted web servers, providing customers with a seamless purchasing experience." }
                                li { "Integrated third-party payment and shipping services, including PayPal, Payoneer, Stripe, and DHL, supporting payment processing, shipping manifest generation, and pricing policy synchronization." }
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
                            ul { class: "about-item-achievements",
                                li { "Customized and localized an internal customer service dashboard to accommodate the operational and business requirements of four clients across the Asia-Pacific region." }
                                li { "Supported backend integration with TIBCO middleware, facilitating data exchange and transformation between application interfaces and underlying databases." }
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
                div { class: "about-list",
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
