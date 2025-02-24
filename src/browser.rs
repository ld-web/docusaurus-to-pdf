use chromiumoxide::{
    browser::BrowserConfigBuilder,
    cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams, Browser, Page,
};
use futures::StreamExt;
use std::error::Error;

/// Creates a new browser and a handle to it, returns both as a tuple.
pub async fn get_browser_and_handle(
) -> Result<(Browser, async_std::task::JoinHandle<()>), Box<dyn Error>> {
    let config = BrowserConfigBuilder::default()
        .window_size(1200, 600)
        .viewport(None)
        // .headless_mode(chromiumoxide::browser::HeadlessMode::False)
        .build()?;
    let (browser, mut handler) = Browser::launch(config).await?;

    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    Ok((browser, handle))
}

/// Creates a new page and directly scrolls to the bottom if needed.
///
/// This is useful to make sure that lazy content is loaded.
pub async fn get_new_page(
    browser: &Browser,
    with_scroll_to_bottom: bool,
) -> Result<Page, Box<dyn Error>> {
    let page = browser.new_page("about:blank").await?;

    if with_scroll_to_bottom {
        page.execute(
            AddScriptToEvaluateOnNewDocumentParams::builder()
                .source("window.scrollTo(0, document.body.scrollHeight);")
                .build()?,
        )
        .await?;
    }

    Ok(page)
}
