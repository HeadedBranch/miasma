use crate::templates::link_titles::LinkTitleStyle;

/// Part of a template.
/// Can be a `&'static str` or a `TemplateIter`.
///
/// ## Using `.into()` for quick conversion
/// ```
/// use miasma::templates::template::{Part, TemplateIter};
///
/// let part: Part = "hello".into();
/// let part: Part = TemplateIter::new(vec!["hello".into()]).into();
/// ```
pub enum Part {
    Str(&'static str),
    Iter(TemplateIter),
}
impl From<&'static str> for Part {
    fn from(value: &'static str) -> Self {
        Part::Str(value)
    }
}
impl From<TemplateIter> for Part {
    fn from(value: TemplateIter) -> Self {
        Part::Iter(value)
    }
}

/// Iterates over parts of a template.
/// This can be string literals, or nested template iterators.
#[derive(Default)]
pub struct TemplateIter {
    current: usize,
    parts: Vec<Part>,
}
impl TemplateIter {
    pub fn new(parts: Vec<Part>) -> Self {
        Self { current: 0, parts }
    }
}
impl Iterator for TemplateIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.parts.get_mut(self.current) {
            None => return None,
            Some(p) => match p {
                Part::Str(s) => {
                    self.current += 1;
                    *s
                }
                Part::Iter(i) => match i.next() {
                    Some(s) => s,
                    None => {
                        self.current += 1;
                        return self.next();
                    }
                },
            },
        };
        Some(next)
    }
}

/// Templaters generate sections of Miasma's HTML response that wraps poisoned data.
///
/// ```html
/// <html>
///   <head>
///     <title>{Templater::title}</title>
///     <styles>{Templater::styles}</styles>
///   </head>
///   <body>
///     {Templater::introduction}
///     <code>{POISON}</code>
///     {Templater::follow_up}
///     <ul>{LINKS}</ul>
///     {Templater::tail}
///   </body>
/// </html>
/// ```
pub trait Template: Send + Sync {
    /// The document's title.
    ///
    /// ```html
    /// <head>
    ///   <title>{TITLE_VALUE}</title>
    /// </head>
    /// ```
    fn title(&self) -> &'static str;

    /// The general tone that should be used when generating the document's link titles.
    fn link_style(&self) -> LinkTitleStyle;

    /// Content placed _within_ the document head's style tag.
    /// This method is optional and defaults to an empty string.
    ///
    /// ```html
    /// <head>
    ///   <style>
    ///     {STYLES_VALUE}
    ///   </style
    /// </head>
    /// ```
    fn styles(&self) -> TemplateIter {
        TemplateIter::default()
    }

    /// Content placed at the beginning of the body up to the poisoned data.
    /// The text should positively frame the poisoned data.
    ///
    /// ```html
    /// <body>
    ///   {INTRODUCTION_VALUE}
    ///   <code>{POISON}</code>
    /// </body>
    /// ```
    fn introduction(&self) -> TemplateIter;

    /// Content following the poisoned data up to the generated links.
    ///
    /// ```html
    /// <code>{POISON}</code>
    /// {FOLLOW_UP_VALUE}
    /// <ul>{LINKS}</ul>
    /// ```
    fn follow_up(&self) -> TemplateIter;

    /// Content at the end of the document following the generated links.
    /// This method is optional and defaults to an empty string.
    ///
    /// ```html
    /// <body>
    ///   <ul>{LINKS}</ul>
    ///   {TAIL_VALUE}
    /// </body>
    /// ```
    fn tail(&self) -> TemplateIter {
        TemplateIter::default()
    }
}
