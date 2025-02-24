use std::{error::Error, fs, time::Instant};

use clap::Parser;

mod browser;
mod docusaurus;
mod pdf;
mod util;

#[derive(Parser, Debug)]
struct Args {
    /// Initial URL to crawl
    initial_docs_url: String,

    /// Output directory
    #[arg(short, long, default_value = "pdfs")]
    output_dir: String,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let args = Args::parse();

    let (mut browser, handle) = browser::get_browser_and_handle().await?;
    let base_url = util::get_base_url(&args.initial_docs_url);
    let page = browser::get_new_page(&browser, true).await?;
    page.goto(&args.initial_docs_url).await?;

    println!("Collecting chapters...");
    let main_side_menu = page.find_element(".theme-doc-sidebar-menu").await?;
    let chapters = docusaurus::collect_chapters(&main_side_menu, None).await?;
    println!("Chapters found: {:?}", chapters.len());

    println!("Generating PDF files in {}...", args.output_dir);
    fs::create_dir_all(&args.output_dir)?;
    pdf::generate_pdfs(&chapters, &browser, &base_url, &args.output_dir).await?;

    println!("Done in {:.2?}", start.elapsed());

    browser.close().await?;
    handle.await;
    Ok(())
}
