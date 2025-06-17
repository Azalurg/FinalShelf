#[derive(Debug, Default)]
pub struct QueryParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub author_name: Option<String>,
    pub genre: Option<String>,
    pub title: Option<String>,
    pub lector: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub read_status: Option<bool>,
}

// impl QueryParams {
//     pub fn new(
//         page: Option<i64>,
//         limit: Option<i64>,
//         author_name: Option<String>,
//         genre: Option<String>,
//         title: Option<String>,
//         lector: Option<String>,
//         sort_by: Option<String>,
//         sort_order: Option<String>,
//         read_status: Option<bool>,
//     ) -> Self {
//         Self {
//             page,
//             limit,
//             author_name,
//             genre,
//             title,
//             lector,
//             sort_by,
//             sort_order,
//             read_status,
//         }
//     }
// }
// pub fn validate(&self) -> Result<(), String> {
//     // Sprawdź sortowanie
//     if let Some(sort_order) = &self.sort_order {
//         if sort_order != "asc" && sort_order != "desc" {
//             return Err(format!("Invalid sort_order: {}", sort_order));
//         }
//     }

//     // Sprawdź kolumnę sortowania
//     if let Some(sort_by) = &self.sort_by {
//         if sort_by != "author" && sort_by != "title" {
//             return Err(format!("Invalid sort_by: {}", sort_by));
//         }
//     }

//     // Sprawdź paginację
//     if let Some(page) = self.page {
//         if page <= 0 {
//             return Err("Page number must be greater than 0.".to_string());
//         }
//     }

//     if let Some(page_size) = self.limit {
//         if self.page <= 0 {
//             return Err("Page size must be greater than 0.".to_string());
//         }
//     }

//     Ok(())
// }

//     pub fn sanitize(&mut self) {
//         self.filter_author = self.filter_author.as_ref().map(|s| s.replace('%', "").replace('_', ""));
//         self.filter_genre = self.filter_genre.as_ref().map(|s| s.replace('%', "").replace('_', ""));
//     }
// }
