//! The main module that fetches html code and scrapes the more information href link and displays
//! it on stdout.
//! Reference: https://dev.to/neon_mmd/how-to-wrap-your-errors-with-enums-when-using-error-stack-49p0

use error_stack::{Context, Result, Report, IntoReport, ResultExt};
use reqwest::header::{HeaderMap, CONTENT_TYPE, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use std::{println, time::Duration};
use core::fmt;

#[derive(Debug)]
enum ScraperError {
    InvalidHeaderMapValue,
    RequestError,
    SelectorError,
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScraperError::InvalidHeaderMapValue => {
                write!(f, "Invalid header map value provided")
            }
            ScraperError::RequestError => {
                write!(f, "Error occurred while requesting data from the webpage")
            }
            ScraperError::SelectorError => {
                write!(f, "An error occured while initializing new Selector")
            }
        }
    }
}

impl Context for ScraperError {
}

fn main() -> Result<(), ScraperError> {
    // A map that holds various request headers
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:105.0) Gecko/20100101 Firefox/105.0"
            .parse()
            .into_report()
            .change_context(ScraperError::InvalidHeaderMapValue)?, // --> (1)
    );
    headers.insert(REFERER, "https://google.com/"
            .parse()
            .into_report()
            .change_context(ScraperError::InvalidHeaderMapValue)?
    ); // --> (1)
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded"
            .parse()
            .into_report()
            .change_context(ScraperError::InvalidHeaderMapValue)?
    ); // --> (1)

    // A blocking request call that fetches the html text from the example.com site.
    let html_results = reqwest::blocking::Client::new()
        .get("https://example.com/")
        .timeout(Duration::from_secs(30))
        .headers(headers)
        .send()
        .into_report()
        .change_context(ScraperError::RequestError)? // --> (2)
        .text()
        .into_report()
        .change_context(ScraperError::RequestError)?; // --> (2) // --> (2)

    // Parse the recieved html text to html for scraping.
    let document = Html::parse_document(&html_results);

    // Initialize a new selector for scraping more information href link.
    let more_info_href_link_selector = Selector::parse("div>p>a$")
        .map_err(|_| Report::new(ScraperError::SelectorError))
        .attach_printable_lazy(|| "invalid CSS selector provided")?; // --> (3)

    // Scrape the more information href link.
    let more_info_href_link = document
        .select(&more_info_href_link_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    // Print the more information link.
    println!("More information link: {}", more_info_href_link);

    Ok(())
}