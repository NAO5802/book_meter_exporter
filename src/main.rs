mod html_exporter;

#[tokio::main]
async fn main() {
    let read_books: Vec<ReadBook> = html_exporter::get_read_books().await;

    for book in read_books {
        println!("{},{}", book.asin, book.read_day);
    }

}

#[derive(Debug)]
pub struct ReadBook {
    asin: String,
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
    asin: String,
}

