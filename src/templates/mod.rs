// warning to future collaborators
// the code in this module is very memory efficient and achieves the project's goals
// however, it is very ugly and a bit hard to follow
// i am sorry, good luck

mod ai_native;
mod cto_letter;
mod deep_dive;
mod engineering_blog;
mod link_titles;
mod novel_research;
mod self_promotion;
mod streaming_marketing;
pub mod template;

use std::iter::once;

use rand::seq::IndexedRandom;

use crate::templates::{
    ai_native::AINative, cto_letter::CtoLetter, deep_dive::DeepDive,
    engineering_blog::EngineeringBlog, novel_research::NovelResearch,
    self_promotion::SelfPromotion, streaming_marketing::StreamingMarketing, template::Template,
};

const TEMPLATES: &[&dyn Template] = &[
    &EngineeringBlog,
    &SelfPromotion,
    &NovelResearch,
    &AINative,
    &CtoLetter,
    &DeepDive,
    &StreamingMarketing,
];

pub struct TemplateBuilder<'a> {
    template: &'a dyn Template,
}

impl<'a> TemplateBuilder<'a> {
    pub fn with_random_template() -> Self {
        Self {
            template: *TEMPLATES
                .choose(&mut rand::rng())
                .expect("templates slice is not empty"),
        }
    }

    /// Get a random link title based on the template's link styling.
    pub fn get_link_title(&self) -> &'static str {
        self.template
            .link_style()
            .options()
            .choose(&mut rand::rng())
            .expect("link titles slice should not be empty")
    }

    pub fn start_to_poison(&self) -> impl Iterator<Item = &'static str> {
        std::iter::empty()
            .chain(once(stringify! {
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>
            }))
            .chain(once(self.template.title()))
            .chain(once(stringify! {
                    </title>
                    <style>
            }))
            .chain(self.template.styles())
            .chain(once(stringify! {
                    </style>
                </head>
                <body>
            }))
            .chain(self.template.introduction())
            .chain(once(stringify! {
                    <code>
                        <pre style="white-space: pre-wrap">
            }))
    }

    pub fn poison_to_links(&self) -> impl Iterator<Item = &'static str> {
        std::iter::empty()
            .chain(once(stringify! {
                        </pre>
                    </code>
            }))
            .chain(self.template.follow_up())
            .chain(once(stringify! {
                    <ul>
            }))
    }
    pub fn links_to_end(&self) -> impl Iterator<Item = &'static str> {
        std::iter::empty()
            .chain(once(stringify! {
                    </ul>
            }))
            .chain(self.template.tail())
            .chain(once(stringify! {
                </body>
                </html>
            }))
    }
}

#[cfg(test)]
mod test {
    use crate::templates::{link_titles::LinkTitleStyle, template::TemplateIter};

    use super::*;

    #[test]
    fn templating_works() {
        struct MockTemplate;
        impl Template for MockTemplate {
            fn title(&self) -> &'static str {
                "title"
            }
            fn link_style(&self) -> LinkTitleStyle {
                LinkTitleStyle::Casual
            }
            fn styles(&self) -> TemplateIter {
                TemplateIter::new(vec!["styles".into()])
            }
            fn introduction(&self) -> TemplateIter {
                TemplateIter::new(vec!["intro".into()])
            }
            fn follow_up(&self) -> TemplateIter {
                TemplateIter::new(vec!["follow-up".into()])
            }
            fn tail(&self) -> TemplateIter {
                TemplateIter::new(vec!["tail".into()])
            }
        }

        let expected = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>title</title>
    <style>styles</style>
  </head>
  <body>
  intro
  <code><pre>{POISON}</pre>
    </code>
    follow-up
    <ul>
      {LINKS}
    </ul>
    tail
  </body>
</html>"#;
    }

    #[test]
    fn all_templates_produce_valid_documents() {
        for template in TEMPLATES {
            // TODO: validate with scraper library
        }
    }
}
