use finalshelf::models::query::ListParams;

#[test]
fn clamps_pagination_and_limit() {
    let params = ListParams {
        page: Some(-2),
        limit: Some(150),
        ..Default::default()
    };

    assert_eq!(params.page(), 1);
    assert_eq!(params.limit(), 100);
}

#[test]
fn sanitizes_search_pattern() {
    let params = ListParams {
        search: Some("%weird_term_".to_string()),
        ..Default::default()
    };

    assert_eq!(params.search_pattern(), Some("%weirdterm%".to_string()));
}

#[test]
fn validates_sort_order_and_min_score() {
    let params = ListParams {
        sort_order: Some("down".to_string()),
        min_score: Some(-1),
        ..Default::default()
    };

    let result = params.validate();
    assert!(result.is_err());
}
