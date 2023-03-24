pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    println!("prefix {}, request path {}", prefix, request_path);
    let mut prefix_iter = prefix.split("*/");
    let flag = match prefix_iter.next() {
        None => (false, false, false, 0),
        Some(p) => (
            request_path.starts_with(p),
            request_path == p,
            p == prefix,
            p.len(),
        ),
    };
    if flag.0 {
        let mut remaining_path: String = request_path[flag.3..].to_string();
        while let Some(prefix_elt) = prefix_iter.next() {
            println!("prefix_iter: {:?}", prefix_elt);
            // let mut prefix_elt_compl = prefix_elt.to_string();
            // if !prefix_elt_compl.ends_with('/') && prefix_elt_compl != remaining_path {
            //     prefix_elt_compl.push('/');
            // }
            if let Some(idx) = remaining_path.find(&prefix_elt) {
                println!(
                    "idx in remaining path: {}, remaining path: {}",
                    idx + prefix_elt.len(),
                    remaining_path
                );
                if remaining_path.len() == idx + prefix_elt.len() {
                    return true;
                } else if !prefix_elt.ends_with('/')
                    && !(remaining_path[idx + prefix_elt.len()..idx + prefix_elt.len() + 1] == *"/")
                {
                    println!(
                        "{}",
                        remaining_path[idx + prefix_elt.len()..idx + prefix_elt.len() + 1]
                            .to_string()
                    );
                    return false;
                } else {
                    remaining_path = remaining_path[idx + prefix_elt.len()..].to_string();
                    println!("shorten");
                }
            } else {
                return false;
            }
        }
        if !flag.1 && flag.2 {
            return false;
        }
        return true;
    }
    return false;
}

pub fn exercise9() {
    let result;
    result = prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/books");
    println!("result: {result}\n");
    let result;
    result = prefix_matches("/v1/publishers/*/books", "/v1/publishers/bar/books");
    println!("result: {result}\n");
    let result;
    result = prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/books/book1");
    println!("result: {result}\n");

    let result;
    result = prefix_matches("/v1/publishers/*/books", "/v1/publishers");
    println!("result: {result}\n");

    let result;
    result = prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/booksByAuthor");
    println!("result: {result}\n");

    let result;
    result = prefix_matches("/v1/publishers", "/v1/publishersBooks");
    println!("result: {result}\n");
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
