use ndata::dataobject::*;
use crate::automatic1111::automatic1111::settings::set_default;
use crate::automatic1111::automatic1111::settings::get_settings;
use crate::automatic1111::automatic1111::settings::get_settings_path;
use std::fs::File;
use std::io::Write;

pub fn execute(o: DataObject) -> DataObject {
let a0 = o.get_string("prompt");
let a1 = o.get_int("author_id");
let ax = set(a0, a1);
let mut o = DataObject::new();
o.put_string("a", &ax);
o
}

pub fn set(prompt:String, author_id:i64) -> String {
let mut settings = get_settings(author_id.to_string());
if &prompt == "default" {
  set_default(settings.clone());
  return "Your settings have been reset to default values.".to_string();
}

let mut isok = false;
let mut s = "".to_string();

let input = &prompt;
let i = input.find("=");
if i.is_some(){
  let i = i.unwrap();
  let key = &input[0..i];
  let val = &input[i+1..];

  if key == "hr_upscaler" || key == "upscaler" { settings.put_string(key, val); isok = true; }
  else if key == "model" { settings.put_string(key, val); isok = true; }
  else if key == "enable_hr" { 
    let i = str::parse::<bool>(val);
    if i.is_ok(){
      let i = i.unwrap();
      settings.put_boolean("enable_hr", i);
      isok = true;
    }
    else { s += "The enable_hr value must be either true or false. "; }
  }
  else if key == "restore_faces" { 
    let i = str::parse::<bool>(val);
    if i.is_ok(){
      let i = i.unwrap();
      settings.put_boolean("restore_faces", i);
      isok = true;
    }
    else { s += "The restore_faces value must be either true or false. "; }
  }
  else if key == "hr_scale" { 
    let i = str::parse::<f64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i >= 1.2 && i <= 4.0 {
        settings.put_float("hr_scale", i);
        isok = true; 
      }
    }
    if !isok { s += "Hires scale must be a number between 1.2 and 4.0. "; }
  }
  else if key == "cfg_scale" { 
    let i = str::parse::<f64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i >= 1.0 && i <= 30.0 {
        settings.put_float("cfg_scale", i);
        isok = true; 
      }
    }
    if !isok { s += "The classifier-free guidance scale must be a number between 1.0 and 30.0. "; }
  }
  else if key == "denoising_strength" { 
    let i = str::parse::<f64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i >= 0.0 && i <= 1.0 {
        settings.put_float("denoising_strength", i);
        isok = true; 
      }
    }
    if !isok { s += "The denoising strength must be a number between 0.0 and 1.0. "; }
  }
  else if key == "width" { 
    let i = str::parse::<i64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i > 255 && i < 2049 {
        settings.put_int("width", i);
        isok = true; 
      }
    }
    if !isok { s += "Width must be an integer between 256 and 2048. "; }
  }
  else if key == "height" { 
    let i = str::parse::<i64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i > 255 && i < 2049 {
        settings.put_int("height", i);
        isok = true; 
      }
    }
    if !isok { s += "Height must be an integer between 256 and 2048. "; }
  }
  else if key == "batch_size" { 
    let i = str::parse::<i64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i > 0 && i < 7 {
        settings.put_int("batch_size", i);
        isok = true; 
      }
    }
    if !isok { s += "Batch size must be an integer between 1 and 6. "; }
  }
  else if key == "seed" { 
    let i = str::parse::<i64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      settings.put_int("seed", i);
      isok = true;
    }
    else { s += "Seed must be an integer. "; }
  }
  else if key == "steps" { 
    let i = str::parse::<i64>(val);
    if i.is_ok(){
      let i = i.unwrap();
      if i > 0 && i < 151 {
        settings.put_int(key, i); 
        isok = true; 
      }
    }
    if !isok { s += "Steps must be an integer between 1 and 150. "; }
  }

  if isok {
    s = s + "OK "+key+" is now "+val;
    save_settings(author_id.to_string(), settings.clone());
  }
  else {
    s += "That does not compute.";
  }
}
else {
  s = format!("ERROR: Set value using 'set' command in format '!set key=value'");
}

s
}

fn save_settings(u:String, d:DataObject) {
  let path = get_settings_path(u.clone());
  let mut file = File::create(path).unwrap();
  file.write_all(d.to_string().as_bytes()).unwrap();
}

