use serde::{Deserialize, Serialize};

/// Generic query parameters for all list endpoints.
///
/// **Pagination:** `page` (1-based), `limit` (1..100, default 21).
/// **Sorting:** `sort_by` (validated per-entity), `sort_order` (`"asc"` | `"desc"`).
/// **Search:** `search` — free-text LIKE across relevant fields (sanitized).
/// **Filters:** entity-specific exact-match filters (books only).
#[derive(Debug, Default, Deserialize)]
pub struct ListParams {
    // Pagination
    pub page: Option<i64>,
    pub limit: Option<i64>,

    // Sorting
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,

    // Free-text search (LIKE across relevant fields)
    pub search: Option<String>,

    // Which fields to search in (e.g. ["title", "author", "genre", "lector"]).
    // If None or empty, searches all fields.
    pub search_fields: Option<Vec<String>>,

    // Book-specific exact-match filters
    pub author_name: Option<String>,
    pub genre: Option<String>,
    pub lector: Option<String>,
    pub read_status: Option<bool>,
    pub min_score: Option<i32>,
}

impl ListParams {
    /// Resolved page number (min 1).
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    /// Resolved page size, clamped to [1, 100].
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(21).clamp(1, 100)
    }

    /// SQL offset derived from page and limit.
    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }

    /// Whether the sort order is descending.
    pub fn is_desc(&self) -> bool {
        self.sort_order.as_deref().unwrap_or("asc") == "desc"
    }

    /// Returns the `sort_by` value if it is in the `allowed` list, otherwise `default`.
    pub fn sort_field(&self, allowed: &[&str], default: &str) -> String {
        self.sort_by
            .as_ref()
            .filter(|s| allowed.contains(&s.as_str()))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Build a sanitized SQL LIKE pattern from the `search` field.
    /// Returns `None` if search is empty/blank.
    pub fn search_pattern(&self) -> Option<String> {
        self.search
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let sanitized = s.replace('%', "").replace('_', "");
                format!("%{}%", sanitized)
            })
    }

    /// Check if a specific field should be included in the search.
    /// Returns true if `search_fields` is None/empty (search all) or contains the field.
    pub fn should_search_field(&self, field: &str) -> bool {
        match &self.search_fields {
            Some(fields) if !fields.is_empty() => fields.iter().any(|f| f == field),
            _ => true,
        }
    }

    /// Validate the parameters. Returns `Err` with a human-readable message on failure.
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref order) = self.sort_order {
            if order != "asc" && order != "desc" {
                return Err(format!(
                    "Invalid sort_order: '{}'. Must be 'asc' or 'desc'.",
                    order
                ));
            }
        }
        if let Some(page) = self.page {
            if page < 1 {
                return Err("Page must be >= 1.".to_string());
            }
        }
        if let Some(limit) = self.limit {
            if limit < 1 {
                return Err("Limit must be >= 1.".to_string());
            }
        }
        if let Some(score) = self.min_score {
            if score < 0 {
                return Err("min_score must be >= 0.".to_string());
            }
        }
        Ok(())
    }
}

/// Generic paginated response for all list endpoints.
#[derive(Serialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

impl<T> ListResponse<T> {
    pub fn new(items: Vec<T>, total_count: i64, page: i64, limit: i64) -> Self {
        Self {
            items,
            total_count,
            page,
            limit,
            total_pages: if limit > 0 {
                (total_count + limit - 1) / limit
            } else {
                0
            },
        }
    }
}
