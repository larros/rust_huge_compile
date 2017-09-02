#[macro_use]
extern crate askama;

use askama::Template;

use std::collections::BTreeMap;

#[derive(Template)]
#[template(path = "file.template")]
struct MyTemplate {
    map: BTreeMap<String, &'static str>,
    holder: KeyHolder,
}

struct KeyHolder {
    key: String,
}

impl KeyHolder {
    fn key_as_ref(self: &Self) -> &String {
        &self.key
    }
}

fn main() {
    let key = "a".to_owned();
    let mut map = BTreeMap::new();
    map.insert(key.clone(), "");
    let template = MyTemplate {map: map, holder: KeyHolder{key: key}};
    println!("{}", template.render())
}
