mod fetch_poison;
mod gzip;
mod html_builder;
mod link_settings;
mod route;

pub use link_settings::LinkSettings;
use link_settings::LinkSettingsInner;

pub use route::serve_poison;
