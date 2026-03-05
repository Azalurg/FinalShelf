use crate::{
    models::{
        author::{AuthorListItem, AuthorWithBooks},
        query::{ListParams, ListResponse},
    },
    services::authors_service,
};

#[tauri::command]
pub async fn get_authors_list_command(params: ListParams) -> Result<ListResponse<AuthorListItem>, String> {
    params.validate()?;
    authors_service::list_authors(&params)
}

#[tauri::command]
pub async fn get_author_command(name: String) -> Result<AuthorWithBooks, String> {
    authors_service::get_author(&name)
}
