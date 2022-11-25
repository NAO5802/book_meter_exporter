use std::thread;
use std::time::Duration;
use chrono::NaiveDate;
use regex::Regex;
use scraper::{Html, Selector};
use crate::{Asin, ReadBook, ReadDay};

pub async fn get_html_body(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

pub fn get_target_elements(html: &String, selector_name: String) -> Vec<String> {
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse(selector_name.as_str()).expect("failed to parse selector");

    let mut result = vec![];
    for element in document.select(&selector) {
        result.push(element.html())
    }
    result
}

pub async fn get_read_books() -> Vec<ReadBook> {
    let read_days = distinct_read_day_by_book_id(get_read_days(2).await);
    let asins = distinct_asin_by_book_id(get_asins(&read_days).await);

    generate_read_books(&read_days, &asins)
}

fn distinct_read_day_by_book_id(mut read_days: Vec<ReadDay>) -> Vec<ReadDay>{
    read_days.dedup_by(|a,b| a.book_id == b.book_id);
    read_days

}

fn distinct_asin_by_book_id(mut asins: Vec<Asin>)-> Vec<Asin> {
    asins.dedup_by(|a,b| a.book_id == b. book_id);
    asins
}


fn generate_read_books(read_days: &Vec<ReadDay>, asins: &Vec<Asin>) -> Vec<ReadBook> {
    let mut read_books: Vec<ReadBook> = vec![];
    for asin in asins {
        for read_day in read_days {
            if asin.book_id == read_day.book_id {
                read_books.push(ReadBook {
                    read_day: String::from(&read_day.read_day),
                    asin: asin.asin,
                })
            }
        }
    }
    read_books
}

async fn get_read_days(pages: i32) -> Vec<ReadDay> {
    let mut read_days: Vec<ReadDay> = vec![];

    for page in 1..(pages + 1) {
        let url = format!("https://bookmeter.com/users/390266/books/read?display_type=list&page={}", page);
        let html = get_html_body(url.as_str()).await.expect("failed to get html body");
        let read_day_elements = get_target_elements(&html, String::from(".detail__date"));
        let book_id_elements = get_target_elements(&html, String::from(".thumbnail__cover >  a"));

        for index in 1..21 {
            read_days.push(
                ReadDay {
                    book_id: adapt_book_id(book_id_elements.get(index - 1).unwrap()),
                    read_day: adapt_read_day(&read_day_elements.get(index - 1).unwrap()),
                })
        }

        // 連続アクセスしない
        thread::sleep(Duration::from_millis(1000));
    }
    read_days
}

async fn get_asins(read_days: &Vec<ReadDay>) -> Vec<Asin> {
    let mut asins: Vec<Asin> = vec![];

    for read_day in read_days {
        let url = format!("https://bookmeter.com/books/{}", read_day.book_id);
        let html_body = get_html_body(&url).await.expect("failed to get html body");

        let elements = get_target_elements(&html_body, String::from("a"));

        for element in elements {
            if element.contains("www.amazon.co.jp/dp/product/") {
                asins.push(Asin {
                    book_id: read_day.book_id,
                    asin: adapt_asin(&element),
                })
            }
        }

        // 連続アクセスしない
        thread::sleep(Duration::from_millis(1000));
    }

    asins
}

fn adapt_read_day(element: &String) -> String {
    let html = Html::parse_fragment(element);
    let selector = Selector::parse(".detail__date").expect("failed to parse element");
    let day_str = html.select(&selector).next().unwrap().text().next().unwrap();

    let date = NaiveDate::parse_from_str(day_str, "%Y/%m/%d").unwrap();
    date.format("%Y-%m-%d 00:00:00").to_string()
}

fn adapt_book_id(element: &String) -> i32 {
    let html = Html::parse_fragment(element);
    let selector = Selector::parse("a").expect("failed to parse elemet");
    let href = html.select(&selector).next().unwrap().value().attr("href").expect("failed to get attributes");

    let reg = Regex::new("[0-9]+").unwrap();
    let caps = reg.captures(href).unwrap().get(0).unwrap();
    caps.as_str().parse::<i32>().expect("failed to parse")
}

fn adapt_asin(element: &String) -> i64 {
    let reg = Regex::new("www.amazon.co.jp/dp/product/([0-9]+)").unwrap();
    let cap = reg.captures(element).unwrap().get(1).unwrap();
    cap.as_str().parse::<i64>().expect("failed to parse i64")
}

#[cfg(test)]
mod html_exporter_tests {
    use scraper::{Html, Selector};
    use crate::{Asin, html_exporter, ReadBook, ReadDay};
    use crate::html_exporter::{adapt_asin, adapt_book_id, adapt_read_day, distinct_asin_by_book_id, distinct_read_day_by_book_id, generate_read_books, get_asins, get_read_days};

    #[test]
    fn テストが動くこと() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn htmlのbodyが取得できること() {
        let actual = html_exporter::get_html_body("https://www.rust-lang.org").await.unwrap();
        assert!(actual.contains("A language empowering everyone to build "));
    }

    #[test]
    fn htmlから特定のセレクタを取得できること() {
        // given
        let html = r#"
                            <!DOCTYPE html>
                            <meta charset="utf-8">
                            <title>Hello, world!</title>
                            <h1 class="foo">Hello, <i>world!</i></h1>
                            <h1 class="bar">Happy, <i>helloween!</i></h1>
                           "#.to_string();
        let selector = String::from("h1");
        // when
        let actual = html_exporter::get_target_elements(&html, selector);
        // then
        assert_eq!(actual.get(0).unwrap(), "<h1 class=\"foo\">Hello, <i>world!</i></h1>");
        assert_eq!(actual.get(1).unwrap(), "<h1 class=\"bar\">Happy, <i>helloween!</i></h1>");
    }

    #[tokio::test]
    async fn 指定ページ分の読了日情報が取得できること() {
        let actual = html_exporter::get_read_days(2).await;

        assert_eq!(actual.get(0).unwrap().read_day, String::from("2022-10-20 00:00:00"));
        assert_eq!(actual.get(0).unwrap().book_id, 19532708);
        assert_eq!(actual.get(1).unwrap().read_day, String::from("2022-10-13 00:00:00"));
        assert_eq!(actual.len(), 40);
    }

    #[test]
    fn read_dayが適切なフォーマットで取得できること() {
        let element = String::from("<div class=\"detail__date\">2021/11/02</div>");
        let actual = adapt_read_day(&element);

        assert_eq!(actual, String::from("2021-11-02 00:00:00"));
    }

    #[test]
    fn book_idが適切なフォーマットで取得できること() {
        let element = String::from("<a data-gtm-component-name\"BookThumbnail\" data-gtm-event-type=\"action_on_book_search_result\" href=\"/books/12434764\"><img src=\"https://m.media-amazon.com/images/I/41IsQ7z6tGL._SL500_.jpg\" alt=\"レシピを見ないで作れるようになりましょう。\" class=\"cover__image\"></a>");
        let actual = adapt_book_id(&element);

        assert_eq!(actual, 12434764);
    }

    #[tokio::test]
    async fn book_idに対応するasinが取得できること() {
        let read_days = vec![
            ReadDay { book_id: 12434764, read_day: String::from("2022-10-20 00:00:00") },
            ReadDay { book_id: 19532708, read_day: String::from("2022-10-20 00:00:00") },
        ];
        let actual = html_exporter::get_asins(&read_days).await;

        assert_eq!(actual.get(0).unwrap().asin, 4797393947);
        assert_eq!(actual.get(0).unwrap().book_id, 12434764);
        assert_eq!(actual.get(1).unwrap().asin, 4297127830);
    }

    #[test]
    fn asinが適切なフォーマットで取得できること(){
        let element = String::from("<a target=\"_blank\" class=\"image__cover\" href=\"https://www.amazon.co.jp/dp/product/4297127830/ref=as_li_tf_tl?camp=247&amp;creative=1211&amp;creativeASIN=4297127830&amp;ie=UTF8&amp;linkCode=as2&amp;tag=bookmeter_book_image_image_pc_logoff-22\"><img alt=\"良いコード/悪いコードで学ぶ設計入門 ―保守しやすい 成長し続けるコードの書き方\" src=\"https://m.media-amazon.com/images/I/41CRtHTRSCL._SL500_.jpg\"></a>");
        let actual = adapt_asin(&element);

        assert_eq!(actual, 4297127830);
    }

    #[test]
    fn 読了日とasinのリストから読了本リストを生成する() {
        let read_days = vec![
            ReadDay { book_id: 12434764, read_day: String::from("2022-10-21 00:00:00") },
            ReadDay { book_id: 19532708, read_day: String::from("2022-10-20 00:00:00") },
        ];
        let asins = vec![
            Asin{book_id: 12434764, asin: 4797393947},
            Asin{book_id: 19532708, asin: 4297127830},
        ];

        let actual = generate_read_books(&read_days, &asins);

        assert_eq!(actual.get(0).unwrap().asin, 4797393947);
        assert_eq!(actual.get(0).unwrap().read_day,  String::from("2022-10-21 00:00:00"));
        assert_eq!(actual.get(1).unwrap().asin, 4297127830);
        assert_eq!(actual.get(1).unwrap().read_day,  String::from("2022-10-20 00:00:00"));
    }

    #[test]
    fn book_idが重複するread_bookを排除したリストを返す() {
        let read_days = vec![
            ReadDay { book_id: 12434764, read_day: String::from("2022-10-21 00:00:00") },
            ReadDay { book_id: 12434764, read_day: String::from("2022-10-21 00:00:00") },
            ReadDay { book_id: 19532708, read_day: String::from("2022-10-20 00:00:00") },
        ];

        let actual = distinct_read_day_by_book_id(read_days);

        assert_eq!(actual.len(), 2);
        assert_eq!(actual.get(0).unwrap().book_id, 12434764);
        assert_eq!(actual.get(1).unwrap().book_id, 19532708);
    }

    #[test]
    fn book_idが重複するasinを排除したリストを返す() {
        let asins = vec![
            Asin{book_id: 12434764, asin: 4797393947},
            Asin{book_id: 12434764, asin: 4797393947},
            Asin{book_id: 19532708, asin: 4297127830},
        ];

        let actual = distinct_asin_by_book_id(asins);

        assert_eq!(actual.len(), 2);
        assert_eq!(actual.get(0).unwrap().book_id, 12434764);
        assert_eq!(actual.get(1).unwrap().book_id, 19532708);
    }

}