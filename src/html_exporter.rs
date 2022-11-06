pub(crate) fn get_html_body() -> String {
    // let body = reqwest::get();
    // body
    String::from("aaa")
}

pub(crate) fn parse_html(html: String) -> String {
    // TODO
    String::from("html body")
}

pub(crate) fn print_target_element_text(html: String, selector: String) {
    println!("text!");
}


#[cfg(test)]
mod html_exporter_tests {
    use crate::html_exporter::get_html_body;

    #[test]
    fn テストが動くこと() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn htmlのbodyが取得できること() {
        let actual = get_html_body();
        println!(actual);
        assert!(actual.contains("<body class=\"layouts application\">"));
    }
}

// TODO 消す
// https://docs.rs/reqwest/latest/reqwest/
// https://zenn.dev/su_do/articles/b2ed44a4d5c024
// https://qiita.com/YoshiTheQiita/items/f66828d61293c75a4585
// https://shinshin86.hateblo.jp/entry/2022/03/25/060000