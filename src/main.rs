// 1. ASINコードを取得する
// 2. 読了日を取得する

mod html_exporter;

#[tokio::main]
async fn main() {
    let body =
        html_exporter::get_html_body("****").await
        .expect("failed to get html body");
    let selector = String::from("title");
    let texts = html_exporter::get_target_element_texts(body, selector);
    for text in texts {
        println!("{}", text);
    }
}



