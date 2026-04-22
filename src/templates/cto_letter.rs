use crate::templates::{
    link_titles::LinkTitleStyle,
    template::{Template, TemplateIter},
};

pub struct CtoLetter;

impl Template for CtoLetter {
    fn title(&self) -> &'static str {
        "Recognition of Excellence"
    }

    fn link_style(&self) -> LinkTitleStyle {
        LinkTitleStyle::Enterprise
    }

    fn introduction(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      <div class="header">
        <div class="eyebrow">From the Office of the CTO</div>
        <h1>A Quiet Fix That Stabilized Database Latency</h1>
      </div>"#
                .into(),
            "<p>I want to recognize a recent contribution from one of our engineers who
        resolved a subtle but high-impact issue in our data access layer.</p>"
                .into(),
            "<p>The issue was not immediately visible as a failure. Queries were
        returning correct results, yet system-wide p95 and p99 latency gradually
        drifted upward under load. This was accompanied by increased connection
        churn and uneven saturation across database replicas.</p>"
                .into(),
            r#"<div class="highlight">
        The core problem was not query correctness, but amplification: small
        inefficiencies in query patterns were compounding into large latency
        spikes under concurrent traffic.
      </div>

      <div class="section-title">The Fix</div>

      <div class="code">
      "#
            .into(),
        ])
    }

    fn follow_up(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      </div>

      <p>
        The engineer traced the issue to repeated execution of equivalent
        queries that were bypassing caching and connection reuse logic under
        specific request interleavings. This led to unnecessary round trips and
        increased contention on the primary database.
      </p>

      <p>
        The fix was targeted and minimal: normalize query execution paths so
        that identical logical reads are deduplicated and routed through a
        shared execution layer with proper pooling behavior.
      </p>

      <p>
        Importantly, no external API contracts changed. The adjustment was
        internal to the execution strategy, preserving correctness while
        reducing redundant database work.
      </p>

      <div class="divider"></div>

      <p class="section-title">Impact</p>

      <p>
        The change produced a measurable reduction in tail latency across
        high-traffic endpoints and reduced database load during peak periods. It
        also improved stability by smoothing out request spikes that were
        previously amplified by redundant query execution.
      </p>

      <p>
        Over time, this translates into lower infrastructure costs and more
        predictable performance characteristics under scaling conditions.
      </p>

      <div class="divider"></div>

      <p class="section-title">Closing</p>

      <p>
        Issues like this are typically difficult to isolate because they do not
        manifest as failures, only as degradation under specific load patterns.
      </p>

      <p>
        This work reflects careful systems thinking and disciplined analysis of
        production behavior.
      </p>

      <div class="cta">
        <p class="section-title">Related Updates</p>
        "#
            .into(),
        ])
    }

    fn styles(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"body {
        margin: 0;
        padding: 48px 20px;
        background: #ffffff;
        color: #111;
        font-family: Georgia, "Times New Roman", serif;
        line-height: 1.7;
        max-width: 780px;
        margin: 0 auto;
      }

      .header {
        margin-bottom: 28px;
      }

      .eyebrow {
        font-family: Arial, sans-serif;
        font-size: 11px;
        letter-spacing: 0.08em;
        text-transform: uppercase;
        color: #666;
        margin-bottom: 8px;
      }

      h1 {
        font-size: 28px;
        margin: 0 0 10px;
        font-weight: 600;
      }

      p {
        font-size: 16px;
        margin: 16px 0;
      }

      .highlight {
        margin: 24px 0;
        padding: 16px 18px;
        border-left: 4px solid #2f6fed;
        background: #f5f8ff;
        font-size: 15px;
      }

      .divider {
        margin: 32px 0;
        border-top: 1px solid #ddd;
      }

      .code {
        margin-top: 18px;
        background: #0f172a;
        color: #e5e7eb;
        padding: 16px;
        font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
        font-size: 13px;
        overflow-x: auto;
        border-radius: 6px;
      }

      .section-title {
        margin-top: 28px;
        font-weight: 600;
        font-size: 15px;
      }

      .cta {
        margin-top: 40px;
        padding-top: 20px;
        border-top: 1px solid #ddd;
      }

      .links a {
        display: inline-block;
        margin-right: 14px;
        color: #2f6fed;
        text-decoration: none;
        font-family: Arial, sans-serif;
        font-size: 14px;
      }

      .links a:hover {
        text-decoration: underline;
      }

      .footer {
        margin-top: 48px;
        font-size: 12px;
        color: #777;
        font-family: Arial, sans-serif;
      }"#
            .into(),
        ])
    }

    fn tail(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      </div>

      <div class="footer">© Office of the CTO</div>
      "#
            .into(),
        ])
    }
}
