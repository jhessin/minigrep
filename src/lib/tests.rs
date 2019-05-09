use super::*;

fn setup_query(query: &str) -> (&str, &str) {
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    (query, contents)
}

#[test]
fn one_result() {
    let (query, contents) = setup_query("duct");

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn no_results() {
    let (query, contents) = setup_query("bob");

    assert_eq!(Vec::<&str>::new(), search(query, contents));
}

#[test]
fn multiple_results() {
    let (query, contents) = setup_query("st");

    assert_eq!(
        vec!["Rust:", "safe, fast, productive."],
        search(query, contents)
    );
}

#[test]
fn config_read() {
    let input = [
        " ".to_owned(),
        "Something".to_owned(),
        "Somefile.txt".to_owned(),
    ];

    let query = input[1].clone();
    let filename = input[2].clone();
    let case_sensitive = true;

    assert_eq!(Ok(Config { query, filename, case_sensitive }), Config::new(&input))
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
