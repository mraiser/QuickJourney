use ndata::dataobject::*;
use flowlang::datastore::DataStore;
use flowlang::flowlang::file::read_all_string::read_all_string;
use std::path::PathBuf;
use ndata::data::Data;

pub fn execute(o: DataObject) -> DataObject {
let a0 = o.get_int("author_id");
let ax = settings(a0);
let mut o = DataObject::new();
o.put_string("a", &ax);
o
}

pub fn settings(author_id:i64) -> String {
  let mut settings = get_settings(author_id.to_string());
  let mut s = "".to_string();
  for (k,v) in settings.objects(){
    s += &k;
    s += ": ";
    s += &Data::as_string(v);
    s += "\n";
  }
  s
}

pub fn get_settings_path(u:String) -> PathBuf {
  let mut path = DataStore::new().root.parent().unwrap().join("runtime").join("discord").join("user_settings");
  let mut path = get_sub_dir(path, &u, 4, 4);
  std::fs::create_dir_all(path.clone());
  path.join(u+".json")
}

pub fn set_default(mut d:DataObject) {
  d.put_int("width", 512);
  d.put_int("height", 512);
  d.put_int("steps", 42);
  d.put_float("hr_scale", 2.0);
  d.put_float("cfg_scale", 7.0);
  d.put_float("denoising_strength", 0.75);
  d.put_int("seed", -1);
  d.put_int("batch_size", 1);
  d.put_boolean("enable_hr", true);
  d.put_string("hr_upscaler", "R-ESRGAN 4x+");
  d.put_boolean("restore_faces", true);
  d.put_string("model", "realisticVisionV50_v50VAE");
}

fn get_sub_dir(mut path: PathBuf, id: &str, chars: usize, levels: usize) -> PathBuf {
  let l:usize = chars * levels;
  let mut s = id.to_string();
  while s.len()<l {
    s = s + "_";
  }
  let mut i = 0;
  while i<levels{
    let n:usize = i * chars;
    let m:usize = n + chars;
    i = i + 1;
    let sub = &s[n..m];
    path.push(sub);
  }
  path
}

pub fn get_settings(u:String) -> DataObject {
  let mut globals = DataStore::globals();
  if !globals.has("AUTOMATIC1111"){
    globals.put_object("AUTOMATIC1111", DataObject::new());
  }
  let mut globalsettings = globals.get_object("AUTOMATIC1111");

  if !globalsettings.has(&u) {
    let path = get_settings_path(u.clone());
    let mut d;
    if path.exists(){
      let s = read_all_string(path.into_os_string().into_string().unwrap());
      let mut d2 = DataObject::from_string(&s);
      d = DataObject::new();
      set_default(d.clone());
      for (k,v) in d2.objects(){
        d.set_property(&k,v);
      }
    }
    else {
      d = DataObject::new();
      set_default(d.clone());
    }
    globalsettings.put_object(&u, d.clone());
    return d;
  }
  globalsettings.get_object(&u)
}

