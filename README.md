# Docusaurus to PDF

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

Small utility to crawl a Docusaurus v3 website and create a folder with each chapter as a PDF.

## Usage

```bash
# output dir defaults to "pdfs"
dcsrs-to-pdf <initial-docs-url> --output-dir <output-dir>
```

## Example

```bash
dcsrs-to-pdf https://your-website.com/docs --output-dir docs
```
