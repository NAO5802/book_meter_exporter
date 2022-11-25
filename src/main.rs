mod html_exporter;

#[tokio::main]
async fn main() {
    let read_books: Vec<ReadBook> = html_exporter::get_read_books().await;

    for book in read_books {
        println!("{:?}", book);
    }

}

#[derive(Debug)]
pub struct ReadBook {
    asin: i64,
    read_day: String,
}

#[derive(Debug)]
pub struct ReadDay {
    book_id: i32,
    read_day: String,
}

#[derive(Debug)]
pub struct Asin {
    book_id: i32,
    asin: i64,
}

// output２(できたらcsv)
// book_id, 読了日, asin
// book_id, 読了日, asin
// book_id, 読了日, asin

// リストのnページにアクセス
// nページの読了日とbook_idのペア全部とる
// ページ数分繰り返す
// book_id, 読了日
// book_id, 読了日
// book_id, 読了日

// book_idのリストを渡して、その分だけ個別ページアクセス&asinをとる
// book_id, asin
// book_id, asin
// book_id, asin

