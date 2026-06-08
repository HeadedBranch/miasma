use std::sync::Arc;

use crate::{MiasmaConfig, config::LinkPrefix};

/// Controls link generation for a given response.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum LinkSettings {
    NoLinks,
    Links(LinkSettingsInner),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct LinkSettingsInner {
    pub count: u8,
    pub prefix: LinkPrefix,
    pub next_depth: Option<u32>,
}

impl LinkSettings {
    /// Determine link generation settings based on config and current depth.
    pub fn next(config: Arc<MiasmaConfig>, current_depth: u32) -> Self {
        let at_max_depth = config.max_depth.is_some_and(|max| current_depth >= max);
        if at_max_depth {
            return Self::NoLinks;
        }

        let next_depth = if config.max_depth.is_some() {
            Some(current_depth + 1)
        } else {
            None
        };

        Self::Links(LinkSettingsInner {
            count: config.link_count,
            prefix: config.link_prefix.clone(),
            next_depth,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn links_if_no_max_depth() {
        let config = Arc::new(MiasmaConfig {
            max_depth: None,
            ..MiasmaConfig::default()
        });

        let link_settings = LinkSettings::next(config.clone(), 0);
        let expected = LinkSettings::Links(LinkSettingsInner {
            count: config.link_count,
            prefix: config.link_prefix.clone(),
            next_depth: None,
        });
        assert_eq!(link_settings, expected);
    }

    #[test]
    fn links_if_not_at_max_depth() {
        let config = Arc::new(MiasmaConfig {
            max_depth: Some(5),
            ..MiasmaConfig::default()
        });
        for current_depth in [0, 1, 2, 3, 4] {
            let link_settings = LinkSettings::next(config.clone(), current_depth);
            let expected = LinkSettings::Links(LinkSettingsInner {
                count: config.link_count,
                prefix: config.link_prefix.clone(),
                next_depth: Some(current_depth + 1),
            });
            assert_eq!(link_settings, expected);
        }
    }

    #[test]
    fn no_links_if_at_max_depth() {
        let config = Arc::new(MiasmaConfig {
            max_depth: Some(5),
            ..MiasmaConfig::default()
        });
        for current_depth in [5, 6, 10] {
            let link_settings = LinkSettings::next(config.clone(), current_depth);
            assert!(matches!(link_settings, LinkSettings::NoLinks))
        }
    }
}
