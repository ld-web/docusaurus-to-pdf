use chromiumoxide::Element;
use std::error::Error;
use std::thread;
use std::time::Duration;

/// Chapter corresponds to a list item in the sidebar menu
#[derive(Debug)]
pub struct Chapter {
    /// The menu link contained in the list item
    menu_link: Element,

    /// Chapter's position, like "1", "1.1", "1.1.1", etc.
    pub position: String,
}

impl Chapter {
    /// Chapter's relative URL
    pub async fn href(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.menu_link.attribute("href").await?.unwrap())
    }

    /// Chapter's label
    pub async fn label(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.menu_link.inner_text().await?.unwrap())
    }
}

/// Recursively collects chapters starting from the given element
pub async fn collect_chapters(
    elt: &Element,
    parent_position: Option<String>,
) -> Result<Vec<Chapter>, Box<dyn Error>> {
    let mut chapters = Vec::new();

    let items = elt.find_elements(".menu__list-item").await?;

    for (idx, item) in items.iter().enumerate() {
        let chapter = create_chapter_from_item(item, idx, &parent_position).await?;
        chapters.push(chapter);

        if is_category(item).await {
            expand_category(item).await?;
            let position = get_chapter_position(idx, &parent_position);

            chapters.extend(Box::pin(collect_chapters(item, Some(position))).await?);
        }
    }

    Ok(chapters)
}

/// Returns the position of the chapter, like "1", "1.1", "1.1.1", etc.
pub fn get_chapter_position(idx: usize, parent_position: &Option<String>) -> String {
    match parent_position {
        Some(parent_position) => format!("{}.{}", parent_position, idx + 1),
        None => (idx + 1).to_string(),
    }
}

/// Checks the element's class attribute to see if it's a category
pub async fn is_category(elt: &Element) -> bool {
    match elt.attribute("class").await {
        Ok(Some(css_classes)) => css_classes.contains("theme-doc-sidebar-item-category"),
        _ => false,
    }
}

/// Creates a chapter from a given list item
pub async fn create_chapter_from_item(
    item: &Element,
    idx: usize,
    parent_position: &Option<String>,
) -> Result<Chapter, Box<dyn Error>> {
    let menu_link = item.find_element(".menu__link").await?;
    let position = get_chapter_position(idx, parent_position);

    Ok(Chapter {
        menu_link,
        position,
    })
}

/// Expands a category by clicking on the expander element
///
/// The expander element is either the menu link or a button :
/// - button if the category is itself a page that will be exported as a PDF
/// - menu link if the category is not an actual page (href is "#")
pub async fn expand_category(item: &Element) -> Result<(), Box<dyn Error>> {
    const DOCUSAURUS_TRANSITION_DURATION: u64 = 300;

    let menu_link = item.find_element(".menu__link").await?;
    let expander_element = item.find_element(".clean-btn").await.unwrap_or(menu_link);
    expander_element.click().await?;

    thread::sleep(Duration::from_millis(DOCUSAURUS_TRANSITION_DURATION));

    Ok(())
}
