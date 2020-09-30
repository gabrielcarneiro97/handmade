use std::path::PathBuf;

pub fn root_dir() -> PathBuf {
    ["."].iter().collect()
}

pub fn output_dir() -> PathBuf {
    root_dir().join("output")
}

pub fn src_dir() -> PathBuf {
    root_dir().join("src")
}

pub fn assets_dir() -> PathBuf {
    src_dir().join("assets")
}

pub fn page_path(page : usize, page_name : Option<&str>) -> PathBuf {
    let name = match page_name {
        Some(name) => name,
        None => "page"
    };

    let name = format!("{}-{}", name, page);
    let mut file_name = output_dir().join(name);
    file_name.set_extension("png");
    file_name
}

pub fn letters_path() -> PathBuf {
    root_dir().join("letters")
}

pub fn dic_path(dic_name: &Option<&str>) -> PathBuf {
    match dic_name {
        Some(name) => letters_path().join(name),
        None => letters_path().join("default")
    }
}

pub fn letter_path(letter_name : String, dic_name: &Option<&str>) -> PathBuf {
    let mut path = dic_path(dic_name).join(letter_name);
    path.set_extension("png");
    path
}
