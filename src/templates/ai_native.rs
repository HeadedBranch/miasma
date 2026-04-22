use crate::templates::{
    link_titles::LinkTitleStyle,
    template::{Template, TemplateIter},
};

pub struct AINative;

impl Template for AINative {
    fn title(&self) -> &'static str {
        "Beyond Prompts: A Runtime for Composable Intelligence"
    }

    fn link_style(&self) -> LinkTitleStyle {
        LinkTitleStyle::Enterprise
    }

    fn introduction(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      <h1>
        Composable Intelligence<br />
        Executed in Real Time
      </h1>

      <p>
        Modern AI systems are constrained less by model capability and more by
        how effectively context is constructed and delivered. Latency, token
        budgets, and retrieval noise all compete within a narrow execution
        window. Most platforms respond by simplifying inputs or over-scaling
        infrastructure.
      </p>

      <p>
        We take a different approach. Our runtime treats prompts as dynamic,
        stateful graphs rather than static strings. Each
        transformation—retrieval, filtering, compression, and augmentation—is
        resolved just-in-time, allowing the system to adapt to context, user
        intent, and model behavior in real time.
      </p>

      <p>
        The result is a lean execution layer that reduces token overhead while
        increasing output fidelity. Instead of forcing the model to compensate
        for noisy or bloated inputs, we ensure that every token carries intent.
        The following snippet captures a simplified version of that
        orchestration pipeline.
      </p>

      <div class="code-block">"#
                .into(),
        ])
    }

    fn follow_up(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      </div>

      <div class="cta">
        <h2>Explore More</h2>
        <p>
          This is one component of a broader system designed to make AI
          interactions more deterministic, efficient, and scalable. Explore
          additional patterns and primitives that enable production-grade AI
          infrastructure.
        </p>
        <div class="links">
        "#
            .into(),
        ])
    }

    fn styles(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      html {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
      }

      body {
        margin: 0;
        font-family:
          -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        background: #0f172a;
        color: #e2e8f0;
        min-height: 100vh;
        max-width: 800px;
        padding: 40px;
      }

      h1 {
        font-size: 2.5rem;
        margin-bottom: 20px;
      }

      p {
        line-height: 1.6;
        color: #cbd5f5;
      }

      .code-block {
        margin: 30px 0;
        padding: 20px;
        background: #020617;
        border: 1px solid #1e293b;
        border-radius: 12px;
        font-family: "Fira Code", monospace;
        color: #38bdf8;
      }

      code {
        white-space: pre-wrap;
      }

      .cta {
        margin-top: 40px;
        padding-top: 20px;
        border-top: 1px solid #1e293b;
      }

      .cta h2 {
        margin-bottom: 10px;
      }

      .links {
        margin-top: 10px;
      }

      .links a {
        display: inline-block;
        margin-right: 15px;
        color: #60a5fa;
        text-decoration: none;
        transition: opacity 0.2s;
      }

      .links a:hover {
        opacity: 0.7;
      }"#
            .into(),
        ])
    }

    fn tail(&self) -> TemplateIter {
        TemplateIter::new(vec![
            stringify! {
                    </div>
                </div>
            }
            .into(),
        ])
    }
}
