#[derive(Debug)]
pub struct QueryParams {
    pub filter_author: Option<String>,
    pub filter_genre: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>, // "asc" or "desc"
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl QueryParams {
    pub fn new(
        filter_author: Option<String>,
        filter_genre: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Self {
        Self {
            filter_author,
            filter_genre,
            sort_by,
            sort_order,
            page,
            page_size,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Sprawdź sortowanie
        if let Some(sort_order) = &self.sort_order {
            if sort_order != "asc" && sort_order != "desc" {
                return Err(format!("Invalid sort_order: {}", sort_order));
            }
        }

        // Sprawdź kolumnę sortowania
        if let Some(sort_by) = &self.sort_by {
            if sort_by != "author" && sort_by != "title" {
                return Err(format!("Invalid sort_by: {}", sort_by));
            }
        }

        // Sprawdź paginację
        if let Some(page) = self.page {
            if page <= 0 {
                return Err("Page number must be greater than 0.".to_string());
            }
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err("Page size must be greater than 0.".to_string());
            }
        }

        Ok(())
    }

    pub fn sanitize(&mut self) {
        self.filter_author = self.filter_author.as_ref().map(|s| s.replace('%', "").replace('_', ""));
        self.filter_genre = self.filter_genre.as_ref().map(|s| s.replace('%', "").replace('_', ""));
    }
}
