mod ai_native;
mod cto_letter;
mod deep_dive;
mod engineering_blog;
mod novel_research;
mod self_promotion;
mod streaming_marketing;

pub use ai_native::AINative;
pub use cto_letter::CtoLetter;
pub use deep_dive::DeepDive;
pub use engineering_blog::EngineeringBlog;
pub use novel_research::NovelResearch;
pub use self_promotion::SelfPromotion;
pub use streaming_marketing::StreamingMarketing;

use crate::templating::Templater;

/// All available response templates.
pub const RESPONSE_TEMPLATE_CONSTRUCTORS: &[fn() -> Box<dyn Templater>] = &[
    EngineeringBlog::new,
    SelfPromotion::new,
    NovelResearch::new,
    AINative::new,
    CtoLetter::new,
    DeepDive::new,
    StreamingMarketing::new,
];

pub const CASUAL_STYLES: &[&str] = &[
    include_str!("styles/self-promotion.css"),
    include_str!("styles/deep-dive.css"),
    include_str!("styles/engineering-blog.css"),
];
pub const ACADEMIC_STYLES: &[&str] = &[include_str!("styles/novel-research.css")];
pub const ENTERPRISE_STYLES: &[&str] = &[
    include_str!("styles/ai-native.css"),
    include_str!("styles/cto-letter.css"),
    include_str!("styles/streaming-marketing.css"),
];

#[cfg(test)]
mod test {
    use crate::templating::TemplateBuilder;

    use super::*;

    #[test]
    fn all_templates_produce_valid_documents() {
        for (ind, template_constructor) in RESPONSE_TEMPLATE_CONSTRUCTORS.iter().enumerate() {
            let builder = TemplateBuilder::with_template(template_constructor());
            let document = builder
                .start_to_poison()
                .chain(builder.poison_to_links())
                .chain(builder.links_to_end())
                .collect::<String>();

            let errors = scraper::Html::parse_document(&document).errors;
            assert!(errors.is_empty(), "template at index {ind}: {errors:?}");
        }
    }

    #[test]
    fn all_styles_are_valid() {
        fn validate_style(style: &str, ind: usize, tone: &str) {
            let style_element = format!("<style>{style}</style>");
            let errors = scraper::Html::parse_fragment(&style_element).errors;
            assert!(
                errors.is_empty(),
                "{tone} style  at index {ind}: {errors:?}"
            );
        }

        for (ind, style) in CASUAL_STYLES.iter().enumerate() {
            validate_style(style, ind, "casual");
        }
        for (ind, style) in ACADEMIC_STYLES.iter().enumerate() {
            validate_style(style, ind, "academic");
        }
        for (ind, style) in ENTERPRISE_STYLES.iter().enumerate() {
            validate_style(style, ind, "enterprise");
        }
    }
}
