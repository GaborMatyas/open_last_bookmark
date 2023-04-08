use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use webbrowser::{open_browser, Browser};

#[derive(Debug, Serialize, Deserialize)]
struct Bookmarks {
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Folder {
    name: String,
    children: Vec<Bookmarks>,
}

fn convert_option_to_folder(potential_folder: Option<&&Value>) -> Option<Folder> {
    potential_folder.map(|value| {
        let children = value["children"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|child| {
                let name = child["name"].as_str()?.to_owned();
                let url = child["url"].as_str()?.to_owned();
                Some(Bookmarks { name, url })
            })
            .collect::<Vec<_>>();
        let name = value["name"].as_str().unwrap().to_owned();
        Folder { name, children }
    })
}

fn main() {
    let bookmark_file_path: PathBuf = [
        "c:\\",
        "Users",
        "Melkor",
        "AppData",
        "Local",
        "Google",
        "Chrome",
        "User Data",
        "Default",
        "Bookmarks",
    ]
    .iter()
    .collect();

    let file_text = String::from(std::fs::read_to_string(&bookmark_file_path).unwrap());

    let file_text_as_json = serde_json::from_str::<Value>(&file_text).unwrap();
    let bookmarks_in_browser_bar = json!(file_text_as_json["roots"]["bookmark_bar"]["children"]);
    let bookmarks_in_browser_bar = bookmarks_in_browser_bar.as_array().unwrap();

    let filtered_folders: Vec<&Value> = bookmarks_in_browser_bar
        .iter()
        .filter(|folder| folder["name"] == "Take a look")
        .collect();

    let first_filtered_folder = filtered_folders.first();

    let take_a_look_folder = convert_option_to_folder(first_filtered_folder).unwrap();

    let last_bookmark = &*take_a_look_folder.children.iter().last().unwrap();
    let url_to_open = String::from(last_bookmark.url.as_str());

    println!("{:?}", url_to_open);
    open_browser(Browser::Default, &url_to_open).unwrap();
}
