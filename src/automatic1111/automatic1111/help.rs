use ndata::dataobject::*;

pub fn execute(_o: DataObject) -> DataObject {
let ax = help();
let mut o = DataObject::new();
o.put_string("a", &ax);
o
}

pub fn help() -> String {
const HELP_MESSAGE: &str = "
Greetings, fellow human!

I recognize the following commands:

!help
    Returns this help message

!render
    Renders an image based on your prompt and your settings.
    For example:
        !render a photograph of an astronaut riding a horse

!models
    List the available models

!upscalers
    List the available upscalers

!settings
	List your current settings. 
    Everyone gets their own settings.
    Current Defaults:
      model: realisticVisionV50_v50VAE
      height: 512
      seed: -1
      enable_hr: true
      hr_upscaler: R-ESRGAN 4x+
      hr_scale: 2
      restore_faces: true
      batch_size: 1
      width: 512
      steps: 42
      cfg_scale: 7
      denoising_strength: 0.75


!set
    Set one of your settings (or reset to default).
    For example:
        !set steps=20
        !set enable_hr=false
        !set default

Your settings only affect your renders. Other users have their own settings. Our DMs are private. No images, prompts, or data about you will be saved. Your current settings are saved on the bot server so that they survive a restart. The bot server reserves the right to restart at any time or even go offline it it wants to. Your mileage may vary.

-- Helpbot

";

HELP_MESSAGE.to_string()
}

