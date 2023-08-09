let mut settings = get_settings(author_id.to_string());
let url = "http://192.168.100.61:7860/sdapi/v1/sd-models";
let resp = attohttpc::get(&url).send();
if resp.is_ok() {
  let resp = resp.unwrap();
  let response = resp.text().unwrap();
  let d = DataArray::from_string(&response);
  let mut i = 1;
  let mut s = "Available Models:\n".to_string();
  for m in d.objects(){
    let m = m.object();
    s += &i.to_string();
    s += ": ";
    s += &m.get_string("model_name");
    s += "\n";
    i += 1;
  }
  s += "\nCurrent Model: ";
  s += &settings.get_string("model");
  return s;
}
format!("ERROR {:?}", resp).to_string()