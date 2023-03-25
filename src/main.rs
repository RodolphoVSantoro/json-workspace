use json_workspace::{parse_curl, JSONWorksPace};

fn main() {
    let mut jwp = Box::new(JSONWorksPace::new(Some(".\\test_jwp")));
    let json_file_string = jwp
        .load_file(String::from(".\\test_jwp\\test2.json"))
        .unwrap();
    println!("{json_file_string}");
    let curl_file_string = jwp
        .load_file(String::from(".\\test_jwp\\test1.curl"))
        .unwrap();
    let curl_request = parse_curl(&curl_file_string).unwrap();
    let curl_request_as_json = curl_request.to_beautified_json().unwrap();
    println!("{curl_request_as_json}");
    let empty_file_string = jwp
        .load_file(String::from(".\\test_jwp\\empty.json"))
        .unwrap();
    println!("'{empty_file_string}'");
    let file_tree = jwp.load_file_tree().unwrap();
    println!("{file_tree:?}");
}
