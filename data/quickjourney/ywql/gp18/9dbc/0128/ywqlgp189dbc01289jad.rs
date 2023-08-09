let mut meta = DataStore::globals().get_object("system").get_object("apps").get_object("quickjourney").get_object("runtime");
let baseurl;
if meta.has("baseurl") { baseurl = meta.get_string("baseurl"); }
else { baseurl = "http://localhost:7860".to_string(); }
let url = &(baseurl+"/sdapi/v1/txt2img");

let mut settings = get_settings(author_id.to_string());
let build = Path::new("/tmp");

let mut d = DataObject::new();
d.put_string("prompt", &prompt);
d.put_int("width", settings.get_int("width"));
d.put_int("height", settings.get_int("height"));
d.put_int("steps", settings.get_int("steps"));
d.put_int("seed", settings.get_int("seed"));
d.put_int("batch_size", settings.get_int("batch_size"));
d.put_float("hr_scale", settings.get_float("hr_scale"));
d.put_boolean("enable_hr", settings.get_boolean("enable_hr"));
d.put_string("hr_upscaler", &settings.get_string("hr_upscaler"));
d.put_boolean("restore_faces", settings.get_boolean("restore_faces"));
d.put_float("cfg_scale", settings.get_float("cfg_scale"));
d.put_float("denoising_strength", settings.get_float("denoising_strength"));

let mut o = DataObject::new();
o.put_string("sd_model_checkpoint", &settings.get_string("model"));
d.put_object("override_settings", o);

let resp = attohttpc::post(&url)
  .header("Accept", "application/json")
  .header("Content-Type", "application/json")
  .text(d.to_string())
  .send();

let mut o = DataObject::new();

if resp.is_ok() {
  let resp = resp.unwrap();
  let response = resp.text().unwrap();
  let mut d = DataObject::from_string(&response);
  let imgs = d.get_array("images");
  let dd = DataObject::from_string(&d.get_string("info"));
  
  o.put_string("title", "Text to Image");
  o.put_string("description", &("Prompt: ".to_string()+&prompt));
  
  fn make(a:&str, b:&str, c:bool) -> DataArray {
    let mut v = DataArray::new();
    v.push_string(a);
    v.push_string(b);
    v.push_boolean(c);
    v
  }
  let mut a = DataArray::new();
  a.push_array(make("Seed", &dd.get_int("seed").to_string(), true));
  a.push_array(make("Sampler", &dd.get_string("sampler_name"), true));
  a.push_array(make("Steps", &dd.get_float("steps").to_string(), true));
  a.push_array(make("CFG Scale", &dd.get_float("cfg_scale").to_string(), true));
  a.push_array(make("Denoise", &dd.get_float("denoising_strength").to_string(), true));
  a.push_array(make("Clip Skip", &dd.get_int("clip_skip").to_string(), true));
  
  let mut fields = DataArray::new();
  fields.push_array(a);
  fields.push_array(make("Model", &settings.get_string("model"), false));
  o.put_array("fields", fields);
  
  let mut files = DataArray::new();
  for img in imgs.objects() {
    let img = img.string();
    let char2: Vec<char> = img.chars().collect::<Vec<_>>();
    let b = Base64::decode(char2);

    let fname = unique_session_id()+".png";
    let f = build.join(&fname);
    let _x = std::fs::create_dir_all(&build);
    let _x = std::fs::write(&f, &b).unwrap();
    files.push_string(&f.into_os_string().into_string().unwrap());
  }
  o.put_array("files", files);
}
else {
  o.put_string("content", &("ERROR ".to_string() + &prompt));
}

o