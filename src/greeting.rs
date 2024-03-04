
use rand::prelude::*;
pub fn greeting() -> String{
    let mut generator= rand::thread_rng();
    let number =  generator.gen_range(1..9);
    let message;
    match number {
        1 => message = "Welcome aboard Captain".to_string(),
        2 => message = "Welcome back Captain".to_string(),
        3 => message = "It's never too late to add to the Log".to_string(),
        4 => message = "Sunset at sea beats a day in the Office".to_string(),
        5 => message = "The most important thing is that we're all in this together. If a memory is lost, we'll just make a new one - Solanum.".to_string(),
        6 => message = "We should savor every moment, embrace the present, and live for the future. - Hornfels".to_string(),
        7 => message = "It's a scary universe out there, but at least we're all in it together. - Feldspar".to_string(),
        8 => message = "The greatest mystery isn't our universe; it's what lies beyond. - Gabro".to_string(),
        9 => message = "No amount of marshmallow can fix that problem. Believe me, I've tried. - Chert".to_string(),
        _ => message = "die".to_string(),
    };
    message 
}
