use ndata::dataobject::*;
use ndata::dataarray::DataArray;
use crate::quickjourney::quickjourney::settings::get_settings;
use flowlang::datastore::DataStore;

pub fn execute(o: DataObject) -> DataObject {
let a0 = o.get_int("author_id");
let ax = upscalers(a0);
let mut o = DataObject::new();
o.put_string("a", &ax);
o
}

pub fn upscalers(author_id:i64) -> String {
let mut meta = DataStore::globals().get_object("system").get_object("apps").get_object("quickjourney").get_object("runtime");
let baseurl;
if meta.has("baseurl") { baseurl = meta.get_string("baseurl"); }
else { baseurl = "http://localhost:7860".to_string(); }
let url = &(baseurl+"/sdapi/v1/upscalers");

let mut settings = get_settings(author_id.to_string());
let resp = attohttpc::get(&url).send();
if resp.is_ok() {
  let resp = resp.unwrap();
  let response = resp.text().unwrap();
  let d = DataArray::from_string(&response);
  let mut i = 1;
  let mut s = "Available Upscaler:\n".to_string();
  for m in d.objects(){
    let m = m.object();
    s += &i.to_string();
    s += ": ";
    s += &m.get_string("name");
    s += "\n";
    i += 1;
  }
  s += "\nCurrent Upscaler: ";
  s += &settings.get_string("hr_upscaler");
  return s;
}
format!("ERROR {:?}", resp).to_string()
}

