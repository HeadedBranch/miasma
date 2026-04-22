use crate::templates::{
    link_titles::LinkTitleStyle,
    template::{Template, TemplateIter},
};

pub struct EngineeringBlog;

impl Template for EngineeringBlog {
    fn title(&self) -> &'static str {
        "On Reducing Redundant Execution in Concurrent Services"
    }

    fn styles(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"body {
        margin: 0;
        padding: 40px 18px;
        background: #ffffff;
        color: #111;
        font-family: "Courier New", Courier, monospace;
        font-size: 13px;
        line-height: 1.6;
        max-width: 760px;
        margin: 0 auto;
      }

      h1 {
        font-size: 16px;
        font-weight: bold;
        margin: 0 0 6px;
      }

      .meta {
        font-size: 12px;
        color: #666;
        margin-bottom: 24px;
      }

      p {
        margin: 12px 0;
      }

      .rule {
        margin: 24px 0;
        border-top: 1px solid #ccc;
      }

      .section {
        margin-top: 20px;
        margin-bottom: 10px;
        font-weight: bold;
      }

      pre {
        background: #f7f7f7;
        border: 1px solid #ddd;
        color: #333;
        padding: 14px;
        font-size: 12px;
        white-space: pre-wrap;
      }

      code {
        font-family: "Courier New", Courier, monospace;
        overflow-x: auto;
      }

      .note {
        margin: 14px 0;
        padding-left: 10px;
        border-left: 2px solid #bbb;
        color: #333;
      }

      a {
        color: #0645ad;
        text-decoration: none;
      }

      a:hover {
        text-decoration: underline;
      }

      .footer {
        margin-top: 40px;
        font-size: 11px;
        color: #777;
      }"#
            .into(),
        ])
    }

    fn introduction(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
      <h1>On Reducing Redundant Execution in Concurrent Services</h1>
      <div class="meta">systems note</div>

      <p>
        The most persistent inefficiencies in production systems are rarely
        algorithmic in nature. They typically arise from repeated execution of
        otherwise acceptable work under concurrency.
      </p>

      <p>
        In a recent investigation, a service exhibited elevated resource usage
        without any obvious hotspots in profiling data. Individual requests were
        inexpensive; aggregate behavior was not.
      </p>

      <div class="note">
        Observation: the system was performing equivalent work multiple times
        per unit of demand.
      </div>

      <div class="section">change</div>"#
                .into(),
        ])
    }

    fn follow_up(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"<p>
        The adjustment introduces coordination at the boundary of execution.
        Rather than optimizing the operation itself, it ensures identical work
        is not repeated concurrently.
      </p>

      <p>
        This distinction matters. Optimization reduces cost per operation.
        Coordination reduces the number of operations.
      </p>

      <div class="rule"></div>

      <div class="section">result</div>

      <p>
        After deployment, observed improvements were consistent across load
        profiles: reduced CPU consumption, lower tail latency, and improved
        stability under burst conditions.
      </p>

      <p>
        The more significant effect was not peak performance, but reduced
        variance. Systems became easier to reason about under stress.
      </p>

      <div class="rule"></div>

      <div class="section">closing remark</div>

      <p>
        This class of issue tends to recur in distributed systems. It is not
        usually visible in isolated traces, and it often survives basic
        optimization efforts.
      </p>

      <p>
        The solution, when it appears, is typically structural rather than
        computational.
      </p>

      <div class="section">references</div>

      <p>"#
                .into(),
        ])
    }

    fn link_style(&self) -> LinkTitleStyle {
        LinkTitleStyle::Casual
    }

    fn tail(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"</p>

      <div class="footer">engineering archive — internal systems notes</div>
    "#
            .into(),
        ])
    }
}
