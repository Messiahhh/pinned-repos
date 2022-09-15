use clap::Parser;
use scraper::{Html, Selector};
use tabled::{Table, Tabled};

#[derive(Debug, Tabled)]
struct Repo<'a> {
    name: &'a str,
    desc: &'a str,
    language: &'a str,
    star: &'a str,
    fork: &'a str,
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, value_parser)]
    user: String,

    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let resp = reqwest::get(format!("https://github.com/{}", args.user)).await?;
    let html = resp.text().await?;
    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("li.mb-3").unwrap();
    let mut pinned_list = Vec::<Repo>::new();

    for element in fragment.select(&selector) {
        let name_selector = Selector::parse(".repo").unwrap();
        let desc_selector = Selector::parse(".pinned-item-desc").unwrap();
        let language_selector = Selector::parse("[itemprop=programmingLanguage]").unwrap();
        let star_selector = Selector::parse(".pinned-item-meta").unwrap();
        let name = element
            .select(&name_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let desc = element
            .select(&desc_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let language = element
            .select(&language_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let mut pinned_item_meta = element.select(&star_selector);
        let [star, fork] =
            [pinned_item_meta.next(), pinned_item_meta.next()].map(|item| match item {
                Some(element) => element
                    .text()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .filter(|s| !s.chars().all(|c| c.is_whitespace()))
                    .map(|s| s.trim())
                    .next()
                    .unwrap(),
                None => "0",
            });
        let repo = Repo {
            name,
            desc,
            language,
            star,
            fork,
        };

        pinned_list.push(repo);
    }
    let table = Table::new(pinned_list).to_string();
    println!("{:?}", table);
    Ok(())
}
