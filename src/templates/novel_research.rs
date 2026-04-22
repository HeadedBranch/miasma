use crate::templates::{
    link_titles::LinkTitleStyle,
    template::{Template, TemplateIter},
};

pub struct NovelResearch;

impl Template for NovelResearch {
    fn title(&self) -> &'static str {
        "A Unified Framework for Globally Consistent Tensor Quantization"
    }

    fn link_style(&self) -> LinkTitleStyle {
        LinkTitleStyle::Academic
    }

    fn styles(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
        body {
          background-color: #ffffff;
          color: #000000
        }
        a {
        color: #0000ff
        }
        a:visited {
            color: #551a8b
        }"#
            .into(),
        ])
    }

    fn introduction(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
    <table width="760" align="center">
      <tr>
        <td>
          <h1>Research Statement</h1>

          <p>
            My recent work investigates optimization strategies for tensor
            quantization in high-dimensional learning systems. Conventional
            approaches treat quantization as a local transformation, applied
            independently across tensor partitions. This often leads to
            suboptimal global error characteristics and inefficient utilization
            of representational capacity.
          </p>

          <p>
            I propose a coordinated quantization framework that exploits
            cross-tensor statistical structure, enabling globally consistent
            compression while maintaining strict error bounds.
          </p>

          <hr />

          <h2>Example: Cross-Channel Residual Alignment (CCRA)</h2>

          <p>
            We introduce Cross-Channel Residual Alignment (CCRA), a novel
            optimization that minimizes quantization error by redistributing
            residuals across correlated tensor channels prior to discretization.
            Rather than quantizing each channel independently, CCRA performs a
            lightweight alignment step that equalizes error sensitivity across
            the tensor.
          </p>

          <p>
            This approach reduces worst-case distortion and improves downstream
            model accuracy without increasing bit-width or requiring retraining.
          </p>

          <h3>Core Implementation</h3>
        "#
            .into(),
        ])
    }

    fn follow_up(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"<p>
            The implementation introduces a pre-quantization projection that
            rebalances channel-wise variance, followed by a shared quantization
            step. The projection is invertible, ensuring that reconstruction
            remains efficient and numerically stable.
          </p>

          <hr />

          <h2>Related Work</h2>
          "#
            .into(),
        ])
    }

    fn tail(&self) -> TemplateIter {
        TemplateIter::new(vec![
            r#"
          <hr />

          <h2>Notes</h2>

          <p>
            The central observation is that quantization error is not uniformly
            distributed across tensor dimensions. By explicitly modeling and
            correcting this imbalance prior to discretization, it is possible to
            achieve higher effective precision at fixed storage cost.
          </p>

          <p>
            This perspective suggests a shift from independent quantization
            schemes toward coordinated, structure-aware optimization methods,
            particularly in large-scale neural systems where inter-channel
            correlations are significant.
          </p>
        </td>
      </tr>
    </table>"#
                .into(),
        ])
    }
}
