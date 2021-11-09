/*
Dependancies needed:
```
reqwest = { version = "0.11", features = ["blocking", "json"] }
select = "0.5.0"
```
*/
use reqwest;

use select::document::Document;
use select::predicate::Class;

pub fn watch() {
    let url = "https://www.reddit.com/r/todayilearned/";

    // If you call reqwest::get it's async, use blocking for
    // a blocking version
    let r = match reqwest::blocking::get(url) {
       Ok(r) => r,
       Err(_) => panic!("Unreachable.")
    };

    // Reads the request
    let results = match Document::from_read(r) {
        Ok(result) => result,
        Err(_) => std::process::abort()
    };

    // Finds all html elements with the class
    let results = results.find(Class("_eYtD2XCVieq6emjKBH3m"));
    for result in results {
        let text = result.text();
        let prefix = &text[..3];

        // All posts being with either TIL or [TIL], the first 3 chracters
        // can be used to check whether or not it is a valid post.
        if prefix == "TIL" || prefix == "[TI" {
            println!("{}\n", text);
        }
    }
}

