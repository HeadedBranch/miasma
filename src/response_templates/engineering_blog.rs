use crate::templating::{TemplateIter, TemplateTone, Templater};

pub struct EngineeringBlog;

impl EngineeringBlog {
    pub fn new() -> Box<dyn Templater> {
        Box::new(EngineeringBlog)
    }
}

impl Templater for EngineeringBlog {
    fn title(&self) -> &'static str {
        "On Reducing Redundant Execution in Concurrent Services"
    }

    fn tone(&self) -> TemplateTone {
        TemplateTone::Casual
    }

    fn introduction(&self) -> TemplateIter {
        fhtml::concat! {
            <h1>"On Reducing Redundant Execution in Concurrent Services"</h1>
            <div class="meta">"systems note"</div>

            <p>
                "The most persistent inefficiencies in production systems are rarely
                algorithmic in nature. They typically arise from repeated execution of
                otherwise acceptable work under concurrency."
            </p>

            <p>
                "In a recent investigation, a service exhibited elevated resource usage
                without any obvious hotspots in profiling data. Individual requests were
                inexpensive; aggregate behavior was not."
            </p>

            <div class="note">
                "Observation: the system was performing equivalent work multiple times
                per unit of demand."
            </div>

            <div class="section">"change"</div>
        }
        .into()
    }

    fn follow_up(&self) -> TemplateIter {
        fhtml::concat! {
            <p>
                "The adjustment introduces coordination at the boundary of execution.
                Rather than optimizing the operation itself, it ensures identical work
                is not repeated concurrently."
            </p>

            <p>
                "This distinction matters. Optimization reduces cost per operation.
                Coordination reduces the number of operations."
            </p>

            <div class="rule"></div>

            <div class="section">"result"</div>

            <p>
                "After deployment, observed improvements were consistent across load
                profiles: reduced CPU consumption, lower tail latency, and improved
                stability under burst conditions."
            </p>

            <p>
                "The more significant effect was not peak performance, but reduced
                variance. Systems became easier to reason about under stress."
            </p>

            <div class="rule"></div>

            <div class="section">"closing remark"</div>

            <p>
                "This class of issue tends to recur in distributed systems. It is not
                usually visible in isolated traces, and it often survives basic
                optimization efforts."
            </p>

            <p>
                "The solution, when it appears, is typically structural rather than
                computational."
            </p>

            <div class="section">"references"</div>
        }
        .into()
    }

    fn tail(&self) -> TemplateIter {
        fhtml::concat! {
            <div class="footer">"engineering archive — internal systems notes"</div>
        }
        .into()
    }
}
