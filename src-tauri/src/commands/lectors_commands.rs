use crate::{
    models::{
        lector::{Lector, LectorWithBooks},
        query::{ListParams, ListResponse},
    },
    services::lectors_service,
};

#[tauri::command]
pub async fn get_lectors_list_command(params: ListParams) -> Result<ListResponse<Lector>, String> {
    params.validate()?;
    lectors_service::list_lectors(&params)
}

#[tauri::command]
pub async fn get_lector_command(lector_name: String) -> Result<LectorWithBooks, String> {
    lectors_service::get_lector(&lector_name)
}
