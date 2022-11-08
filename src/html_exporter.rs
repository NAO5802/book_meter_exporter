use scraper::{Html, Selector};
use scraper::html::Select;

pub async fn get_html_body(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

pub fn get_target_element_texts(html: String, selector_name: String) -> Vec<String> {
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse(selector_name.as_str()).unwrap();

    let mut result:Vec<String> = vec![];
    for element in document.select(&selector) {
        result.push(element.text().next().unwrap().to_string())
    }
    result
}

#[cfg(test)]
mod html_exporter_tests {
    use scraper::{Html, Selector};
    use crate::html_exporter;

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
    fn htmlから特定のセレクタの中身の文字列リストを取得できること() {
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
        let actual = html_exporter::get_target_element_texts(html, selector);
        // then
        assert_eq!(actual.get(0).unwrap().as_str(), "Hello, ");
        assert_eq!(actual.get(1).unwrap().as_str(), "Happy, ");
    }
}