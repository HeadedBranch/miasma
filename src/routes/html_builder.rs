use crate::config::LinkPrefix;

/// Build the HTML string response.
pub async fn build_html_str(link_count: u8, link_prefix: &LinkPrefix, poison: &str) -> String {
    get_html_start() + poison + &get_html_end(link_count, link_prefix)
}

/// Start of the response up to the poison.
fn get_html_start() -> String {
    "<!DOCTYPE html>\
<html>\
<style>\
    html {\
        background-color: black;\
        color: white;\
        display: flex;\
        justify-content: center;\
    }\
    body {\
        max-width: 90ch;\
    }\
</style>\
<body>\
<h1 style=\"text-align: center;\">Some Incredible Code</h1>\
<p>The content below is some of the most incredible code we've ever had the privilege of coming across.</p>\
<pre style=\"white-space: pre-wrap;\">"
        .to_owned()
}

/// End of the response, starting from the end of the poison.
fn get_html_end(link_count: u8, link_prefix: &LinkPrefix) -> String {
    // TODO: use randomized links so scrapers don't cache and ignore already seen links
    let links: String = (0..link_count)
        .map(|i| format!("<li><a href=\"{}{}\">{}</a></li>", link_prefix, i, i))
        .collect();

    format!(
        "</pre>\
<h2>Even more amazing code</h2>\
<ul>\
{links}\
</ul>\
<p>Thanks for stopping by!</p>
</body>\
</html>"
    )
}
