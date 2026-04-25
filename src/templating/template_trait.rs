use crate::templating::{TemplateIter, TemplateTone};

/// Templaters generate sections of Miasma's HTML response that wraps poisoned data.
///
/// Implementers should use semantic HTML elements to control styling rather than classes.
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
pub trait Templater: Send + Sync {
    /// The document's title.
    ///
    /// ```html
    /// <head>
    ///   <title>{TITLE_VALUE}</title>
    /// </head>
    /// ```
    fn title(&self) -> &'static str;

    /// The general tone of the document.
    /// Tone is used to genreate the document's link titles and CSS styles unless overridden.
    fn tone(&self) -> TemplateTone;

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

    /// The template's CSS styles.
    /// Defaults to the `Tone`'s random style method.
    fn styles(&self) -> TemplateIter {
        self.tone().random_style().into()
    }

    /// Get a random link title.
    /// Defaults to the `Tone`'s random link title method.
    fn random_link_title(&self) -> &'static str {
        self.tone().random_link_title()
    }
}
