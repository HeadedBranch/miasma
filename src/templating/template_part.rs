use crate::templating::TemplateIter;

/// Part of a template.
/// Can be a `&'static str` or a `TemplateIter`.
///
/// ## Using `.into()` for quick conversion
/// ```
/// use miasma::templating::{TemplateIter, TemplatePart};
///
/// let part: TemplatePart = "hello".into();
/// let part: TemplatePart = TemplateIter::new(vec!["hello".into()]).into();
/// ```
pub enum TemplatePart {
    Str(&'static str),
    Iter(TemplateIter),
}

impl From<&'static str> for TemplatePart {
    fn from(value: &'static str) -> Self {
        TemplatePart::Str(value)
    }
}

impl From<TemplateIter> for TemplatePart {
    fn from(value: TemplateIter) -> Self {
        TemplatePart::Iter(value)
    }
}
