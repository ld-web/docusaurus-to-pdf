use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use chromiumoxide::Browser;

use crate::browser;
use crate::docusaurus::Chapter;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

pub async fn generate_pdfs(
    chapters: &Vec<Chapter>,
    browser: &Browser,
    base_url: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();

    for chapter in chapters {
        if chapter.href().await? == "#" {
            continue;
        }

        let chapter_url = format!("{}{}", base_url, chapter.href().await?);
        let dest_file = format!(
            "{}/{} - {}.pdf",
            output_dir,
            chapter.position,
            chapter.label().await?
        );
        let page = browser::get_new_page(browser, true).await?;

        handles.push(async_std::task::spawn(async move {
            page.goto(&chapter_url).await.unwrap();
            thread::sleep(Duration::from_secs(1));
            let pdf = page.pdf(PrintToPdfParams::default()).await.unwrap();
            fs::write(dest_file, pdf).unwrap();
        }));
    }

    for thread_handle in handles {
        thread_handle.await;
    }

    Ok(())
}
