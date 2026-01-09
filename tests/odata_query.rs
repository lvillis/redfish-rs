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
    let qs = url.query().unwrap_or_default();

    // We don't assert on exact ordering beyond presence; URL encoding may vary.
    assert!(qs.contains("$select=Id%2CName"));
    assert!(qs.contains("$expand=Members"));
    assert!(qs.contains("$top=10"));
    assert!(qs.contains("$skip=5"));
    assert!(qs.contains("$orderby=Created+desc"));
    assert!(qs.contains("oem=x"));
}

#[test]
fn odata_query_preserves_existing_query() {
    let q = ODataQuery::new().top(1);

    let url = Url::parse("https://example.com/redfish/v1/Systems?foo=bar").expect("valid url");
    let url = q.apply_to_url(url);
    let qs = url.query().unwrap_or_default();

    assert!(qs.contains("foo=bar"));
    assert!(qs.contains("$top=1"));
}
