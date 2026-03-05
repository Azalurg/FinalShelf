use crate::{
    models::{
        book::Book,
        query::{ListParams, ListResponse},
    },
    services::books_service,
};

#[tauri::command]
pub async fn get_books_list_command(params: ListParams) -> Result<ListResponse<Book>, String> {
    params.validate()?;
    books_service::list_books(&params)
}

#[tauri::command]
pub async fn get_book_command(title: String) -> Result<Book, String> {
    books_service::get_book(&title)
}

#[tauri::command]
pub async fn update_book_command(book: Book) -> Result<Book, String> {
    books_service::update_book(&book)
}
