use diesel::dsl::count;
use diesel::prelude::*;
use crate::{
    db::establish_connection,
    models::lector::Lector,
    schema::books::dsl,
};

pub fn get_lectors_list() -> Vec<Lector> {
    let conn = &mut establish_connection();

    // Construct the query
    let query = dsl::books
        .filter(dsl::lector.is_not_null())
        .group_by(dsl::lector)
        .select((dsl::lector, count(dsl::lector)));

    // Execute the query and map the results to the Lector struct
    query
        .load::<(Option<String>, i64)>(conn)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(lector_name, books_count)| {
            lector_name.map(|name| Lector {
                name,
                books_amount: books_count,
            })
        })
        .collect()
}
