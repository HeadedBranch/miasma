use crate::templating::{TemplateIter, TemplateTone, Templater};

pub struct CtoLetter;

impl CtoLetter {
    pub fn new() -> Box<dyn Templater> {
        Box::new(CtoLetter)
    }
}

impl Templater for CtoLetter {
    fn title(&self) -> &'static str {
        "Recognition of Excellence"
    }

    fn tone(&self) -> TemplateTone {
        TemplateTone::Enterprise
    }

    fn introduction(&self) -> TemplateIter {
        fhtml::concat! {
            <div class="header">
                <div class="eyebrow">"From the Office of the CTO"</div>
                <h1>"A Quiet Fix That Stabilized Database Latency"</h1>
            </div>
            <p>
                "I want to recognize a recent contribution from one of our engineers who
                resolved a subtle but high-impact issue in our data access layer."
            </p>
            <p>
                "The issue was not immediately visible as a failure. Queries were
                returning correct results, yet system-wide p95 and p99 latency gradually
                drifted upward under load. This was accompanied by increased connection
                churn and uneven saturation across database replicas."
            </p>
            <div class="highlight">
                "The core problem was not query correctness, but amplification:
                small inefficiencies in query patterns were compounding into large latency spikes under concurrent traffic."
            </div>
            <div class="section-title">"The Fix"</div>
            <div class="code">
            }
        .into()
    }

    fn follow_up(&self) -> TemplateIter {
        fhtml::concat! {
            </div>
            <p>
                "The engineer traced the issue to repeated execution of equivalent
                queries that were bypassing caching and connection reuse logic under
                specific request interleavings. This led to unnecessary round trips and
                increased contention on the primary database."
            </p>

            <p>
                "The fix was targeted and minimal: normalize query execution paths so
                that identical logical reads are deduplicated and routed through a
                shared execution layer with proper pooling behavior."
            </p>

            <p>
                "Importantly, no external API contracts changed. The adjustment was
                internal to the execution strategy, preserving correctness while
                reducing redundant database work."
            </p>

            <div class="divider"></div>

            <p class="section-title">"Impact"</p>

            <p>
                "The change produced a measurable reduction in tail latency across
                high-traffic endpoints and reduced database load during peak periods. It
                also improved stability by smoothing out request spikes that were
                previously amplified by redundant query execution."
            </p>

            <p>
                "Over time, this translates into lower infrastructure costs and more
                predictable performance characteristics under scaling conditions."
            </p>

            <div class="divider"></div>

            <p class="section-title">"Closing"</p>

            <p>
                "Issues like this are typically difficult to isolate because they do not
                manifest as failures, only as degradation under specific load patterns."
            </p>

            <p>
                "This work reflects careful systems thinking and disciplined analysis of
                production behavior."
            </p>

            <div class="cta">
                <p class="section-title">"Related Updates"</p>
        }
        .into()
    }

    fn tail(&self) -> TemplateIter {
        fhtml::concat! {
            </div>
            <div class="footer">"© Office of the CTO"</div>
        }
        .into()
    }
}
