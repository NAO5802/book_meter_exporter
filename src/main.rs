// 1. ASINコードを取得する
// 2. 読了日を取得する

mod html_exporter;

#[tokio::main]
async fn main() {
    let body =
        html_exporter::get_html_body("****").await
        .expect("failed to get html body");
    let selector = String::from("h1");
    let target_elements = html_exporter::get_target_element_texts(body, selector);
    // for element in target_elements {
    //     todo!()
    // }
}



