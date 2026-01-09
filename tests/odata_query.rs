use redfish::ODataQuery;
use url::Url;

#[test]
fn odata_query_appends_expected_pairs() {
    let q = ODataQuery::new()
        .select("Id,Name")
        .expand("Members")
        .top(10)
        .skip(5)
        .filter("Severity eq 'Critical'")
        .orderby("Created desc")
        .with_pair("oem", "x");

    let url = Url::parse("https://example.com/redfish/v1/Systems/1/LogServices/Log/Entries")
        .expect("valid base url");

    let url = q.apply_to_url(url);
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    // Compare decoded pairs to avoid URL-encoding differences between url versions.
    assert!(pairs.iter().any(|(k, v)| k == "$select" && v == "Id,Name"));
    assert!(pairs.iter().any(|(k, v)| k == "$expand" && v == "Members"));
    assert!(pairs.iter().any(|(k, v)| k == "$top" && v == "10"));
    assert!(pairs.iter().any(|(k, v)| k == "$skip" && v == "5"));
    assert!(
        pairs
            .iter()
            .any(|(k, v)| k == "$orderby" && v == "Created desc")
    );
    assert!(pairs.iter().any(|(k, v)| k == "oem" && v == "x"));
}

#[test]
fn odata_query_preserves_existing_query() {
    let q = ODataQuery::new().top(1);

    let url = Url::parse("https://example.com/redfish/v1/Systems?foo=bar").expect("valid url");
    let url = q.apply_to_url(url);
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    assert!(pairs.iter().any(|(k, v)| k == "foo" && v == "bar"));
    assert!(pairs.iter().any(|(k, v)| k == "$top" && v == "1"));
}
