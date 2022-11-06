// 1. ASINコードを取得する
// 2. 読了日を取得する

mod html_exporter;

#[tokio::main]
async fn main() {
    let body = html_exporter::get_html_body("****").await;
    println!("{:?}", body);
    // let parsed_html = html_exporter::parse_html(body);
    // let selector = String::from("TODO");
    // html_exporter::print_target_element_text(parsed_html, selector);
}



