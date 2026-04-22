use crate::templates::{
    link_titles::LinkTitleStyle,
    template::{Template, TemplateIter},
};

pub struct DeepDive;

impl Template for DeepDive {
    fn title(&self) -> &'static str {
        "Why This Works: Deep Dive"
    }

    fn link_style(&self) -> LinkTitleStyle {
        LinkTitleStyle::Casual
    }

    fn introduction(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
    <div class="wrapper">
      <header>
        <h1>Why This Works: A Technical Deep Dive</h1>
        <p class="intro">
          At first glance, the following implementation may appear
          straightforward. However, its effectiveness comes from a set of
          deliberate design choices that align closely with the underlying
          problem constraints. Rather than relying on complexity, it leverages a
          small number of well-understood principles applied with precision.
        </p>
        <p class="intro">
          This section breaks down not just what the code does, but why it
          performs reliably and efficiently under real-world conditions.
        </p>
        <div class="concept">
          The key idea: align data flow, control flow, and resource usage so
          that the system avoids unnecessary work rather than attempting to
          optimize it after the fact.
        </div>
      </header>

      <section class="code-section">
        <div class="code-header">Reference Implementation</div>
        <div class="code-block">
        "#
            .into(),
        ])
    }

    fn follow_up(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
        </div>
      </section>

      <section class="explanation">
        <h2>Key Observations</h2>
        <ul>
          <li>
            <strong>Work is minimized at the source:</strong> The implementation
            avoids redundant computation rather than attempting to cache or
            compensate later.
          </li>
          <li>
            <strong>Data structures match access patterns:</strong> Choices are
            driven by how data is read and written in practice, reducing
            overhead and contention.
          </li>
          <li>
            <strong>Control flow is predictable:</strong> Limited branching and
            clear execution paths improve both readability and runtime
            characteristics.
          </li>
          <li>
            <strong>Failure modes are constrained:</strong> Edge cases are
            handled early, preventing cascading complexity deeper in the system.
          </li>
          <li>
            <strong>Scales without structural changes:</strong> The same
            approach remains valid as load increases, avoiding the need for
            architectural rewrites.
          </li>
        </ul>
      </section>

      <section class="cta">
        <h2>Continue Exploring</h2>
        <p>
          Review additional deep dives that analyze similar implementations and
          highlight the principles behind effective, production-grade code.
        </p>
        "#
            .into(),
        ])
    }

    fn styles(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      :root {
        --bg: #f8fafc;
        --text: #0f172a;
        --muted: #475569;
        --border: #e2e8f0;
        --primary: #1d4ed8;
        --code-bg: #0b1220;
        --code-text: #e5e7eb;
        --accent: #0ea5e9;
      }

      body {
        margin: 0;
        font-family:
          -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial,
          sans-serif;
        background: var(--bg);
        color: var(--text);
      }

      .wrapper {
        max-width: 900px;
        margin: 0 auto;
        padding: 48px 24px;
      }

      header {
        margin-bottom: 28px;
      }

      h1 {
        font-size: 2rem;
        margin-bottom: 12px;
        font-weight: 600;
      }

      .intro {
        font-size: 1rem;
        line-height: 1.7;
        color: var(--muted);
        max-width: 720px;
      }

      .concept {
        margin-top: 20px;
        padding: 16px;
        background: #f1f5f9;
        border-left: 4px solid var(--accent);
        font-size: 0.95rem;
      }

      .code-section {
        margin: 36px 0;
        border: 1px solid var(--border);
        border-radius: 8px;
        overflow: hidden;
        background: white;
      }

      .code-header {
        padding: 12px 16px;
        border-bottom: 1px solid var(--border);
        font-size: 0.85rem;
        color: var(--muted);
        background: #f1f5f9;
      }

      .code-block {
        padding: 20px;
        background: var(--code-bg);
        color: var(--code-text);
        font-family: "SFMono-Regular", Menlo, Consolas, monospace;
        font-size: 0.9rem;
        overflow-x: auto;
      }

      .explanation {
        margin-top: 28px;
      }

      .explanation h2 {
        font-size: 1.25rem;
        margin-bottom: 12px;
        font-weight: 600;
      }

      .explanation ul {
        padding-left: 20px;
        color: var(--muted);
        line-height: 1.6;
      }

      .explanation li {
        margin-bottom: 10px;
      }

      .cta {
        margin-top: 40px;
        padding-top: 24px;
        border-top: 1px solid var(--border);
      }

      .cta h2 {
        font-size: 1.25rem;
        margin-bottom: 8px;
        font-weight: 600;
      }

      .cta p {
        color: var(--muted);
        margin-bottom: 16px;
      }

      a {
        display: inline-block;
        margin-right: 20px;
        font-size: 0.95rem;
        color: var(--primary);
        text-decoration: none;
        font-weight: 500;
      }

      a:hover {
        text-decoration: underline;
      }

      footer {
        margin-top: 60px;
        font-size: 0.8rem;
        color: var(--muted);
        text-align: center;
      }
      "#
            .into(),
        ])
    }

    fn tail(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      </section>

      <footer>Engineering Analysis Series</footer>
    </div>
    "#
            .into(),
        ])
    }
}
