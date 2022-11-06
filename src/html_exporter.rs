pub async fn get_html_body(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
//
// pub(crate) fn parse_html(html: String) -> String {
//     // TODO
//     String::from("html body")
// }
//
// pub(crate) fn print_target_element_text(html: String, selector: String) {
//     println!("text!");
// }
//

#[cfg(test)]
mod html_exporter_tests {
    use crate::html_exporter::get_html_body;

    #[test]
    fn テストが動くこと() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn htmlのbodyが取得できること() {
        let actual = get_html_body("https://www.rust-lang.org").await.unwrap();
        assert!(actual.contains("A language empowering everyone to build "));
    }
}

// TODO 消す
// https://docs.rs/reqwest/latest/reqwest/
// https://zenn.dev/su_do/articles/b2ed44a4d5c024
// https://qiita.com/YoshiTheQiita/items/f66828d61293c75a4585
// https://shinshin86.hateblo.jp/entry/2022/03/25/060000