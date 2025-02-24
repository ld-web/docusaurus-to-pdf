use std::{
    error::Error,
    fs, thread,
    time::{Duration, Instant},
};

use chromiumoxide::{cdp::browser_protocol::page::PrintToPdfParams, Element};
use clap::Parser;

use url::Url;

mod browser;

#[derive(Parser, Debug)]
struct Args {
    /// Initial URL to crawl
    initial_docs_url: String,

    /// Output directory
    #[arg(short, long, default_value = "pdfs")]
    output_dir: String,
}

#[derive(Debug)]
struct Chapter {
    /// Chapter's title
    label: String,
    /// Chapter's relative URL
    relative_url: String,
    /// Chapter's position, like "1", "1.1", "1.1.1", etc.
    position: String,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let args = Args::parse();
    let (mut browser, handle) = browser::get_browser_and_handle().await?;

    let base_url = get_base_url(&args.initial_docs_url);
    let page = browser::get_new_page(&browser, true).await?;
    page.goto(&args.initial_docs_url).await?;

    println!("Collecting chapters...");
    let main_side_menu = page.find_element(".theme-doc-sidebar-menu").await?;
    let chapters = collect_chapters(&main_side_menu, None).await?;
    println!("Chapters found: {:?}", chapters.len());

    println!("Generating PDF files in {}...", args.output_dir);
    fs::create_dir_all(&args.output_dir)?;
    let mut handles = Vec::new();

    for chapter in &chapters {
        if chapter.relative_url == "#" {
            continue;
        }

        let chapter_url = format!("{}{}", base_url, chapter.relative_url);
        let dest_file = format!(
            "{}/{} - {}.pdf",
            args.output_dir, chapter.position, chapter.label
        );
        let page = browser::get_new_page(&browser, true).await?;

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

    println!("Done in {:.2?}", start.elapsed());

    browser.close().await?;
    handle.await;
    Ok(())
}

async fn collect_chapters(
    elt: &Element,
    parent_position: Option<String>,
) -> Result<Vec<Chapter>, Box<dyn Error>> {
    let mut chapters = Vec::new();

    let items = elt.find_elements(".menu__list-item").await?;

    for (idx, item) in items.iter().enumerate() {
        let menu_link = item.find_element(".menu__link").await?;
        let href = menu_link.attribute("href").await?.unwrap();
        let label = menu_link.inner_text().await?.unwrap();
        let position = get_chapter_position(idx, &parent_position);

        let chapter = Chapter {
            label,
            relative_url: href,
            position: position.clone(),
        };

        chapters.push(chapter);

        if is_category(item).await {
            let expander_element = item.find_element(".clean-btn").await.unwrap_or(menu_link);
            expander_element.click().await?;

            thread::sleep(Duration::from_millis(300));

            chapters.extend(Box::pin(collect_chapters(item, Some(position))).await?);
        }
    }

    Ok(chapters)
}

fn get_base_url(url: &str) -> String {
    let parsed_url = Url::parse(url).expect("Couldn't parse URL");
    parsed_url.scheme().to_owned() + "://" + parsed_url.host_str().unwrap()
}

fn get_chapter_position(idx: usize, parent_position: &Option<String>) -> String {
    match parent_position {
        Some(parent_position) => format!("{}.{}", parent_position, idx + 1),
        None => (idx + 1).to_string(),
    }
}

async fn is_category(elt: &Element) -> bool {
    match elt.attribute("class").await {
        Ok(Some(css_classes)) => css_classes.contains("theme-doc-sidebar-item-category"),
        _ => false,
    }
}
