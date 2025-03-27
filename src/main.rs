use std::io::Read;

use clap::Parser;

macro_rules! set_panic {
    ($msg:expr) => {
        std::panic::set_hook(Box::new(|_| {
            println!("{}", $msg);            
        }));
    }
}

#[derive(Parser)]
#[command(version, long_about = Some("Creates rainbow output"))]
struct Args {
    #[arg(short, required(false), default_value_t=0.8)]
    /// Changes gamma setting
    gamma: f64,
    #[arg(short, required(false), default_value_t=-1.0)]
    /// Ratio between zero and one; outputs R G B values instead of colored string
    position: f64,
    #[arg(required(false), default_value_t=String::new())]
    text: String,
    #[arg(short, required(false), default_value_t=1.0)]
    /// Multiply all color values by this POSITIVE number (decimal allowed)
    brightness: f64
}

fn rgb_scale(position: f64, gamma: f64, brightness: f64) -> (u8, u8, u8) {
    
    /* mostly copied from https://stackoverflow.com/questions/1472514/convert-light-frequency-to-rgb */
    let (mut r, mut g, mut b);
    let mut factor = 0.0;

    let position = 780.0 - position + 380.0;
    //{ dbg!(&position); }

    if position >= 380.0 && position < 440.0 {
        r = -(position - 440.0) / (440.0 - 380.0);
        g = 0.0;
        b = 1.0;
    } else if position >= 440.0 && position < 490.0 {
        r = 0.0;
        g = (position - 440.0) / (490.0 - 440.0);
        b = 1.0;
    } else if position >= 490.0 && position < 510.0 {
        r = 0.0;
        g = 1.0;
        b = -(position - 510.0) / (510.0 - 490.0);
    } else if position >= 510.0 && position < 580.0 {
        r = (position - 510.0) / (580.0 - 510.0);
        g = 1.0;
        b = 0.0;
    } else if position >= 580.0 && position < 645.0 {
        r = 1.0;
        g = -(position - 645.0) / (645.0 - 580.0);
        b = 0.0;
    } else if position >= 645.0 && position < 781.0 {
        r = 1.0;
        g = 0.0;
        b = 0.0;
    } else {
        r = 0.0;
        g = 0.0;
        b = 0.0;
    }

    if position >= 380.0 && position < 420.0 {
        factor = 0.3 + 0.7 * (position - 380.0) / (420.0 - 380.0);
    } else if position >= 420.0 && position < 701.0 {
        factor = 1.0;
    } else if position >= 701.0 && position < 781.0 {
        factor = 0.3 + 0.7 * (780.0 - position) / (780.0 - 700.0);
    }


    // Don't want 0^x = 1 for x <> 0
    r = (if r == 0.0 { 0.0 } else { 255.0 * (r * factor).powf(gamma) }) * brightness;
    g = (if g == 0.0 { 0.0 } else { 255.0 * (g * factor).powf(gamma) }) * brightness;
    b = (if b == 0.0 { 0.0 } else { 255.0 * (b * factor).powf(gamma) }) * brightness;

    r = if r > 255.0 { 255.0 } else { r };
    g = if g > 255.0 { 255.0 } else { g };
    b = if b > 255.0 { 255.0 } else { b };


    (r as u8,
     g as u8,
     b as u8)
}

const HELP_MSG: &str = "Usage: `roygbiv [OPTIONS] TEXT`
Avaliable options:
 -g :\tSet gamma level.
 -p :\tOutputs RGB values based on a decimal between 0 and 1, inclusive.
 -b :\tAdjusts the brightness. Default is 1.0";

fn main() {

    set_panic!(HELP_MSG);

    let clapargs = Args::parse();
   
    let gamma = clapargs.gamma;
    //TODO: proper argument parsing
    
    let mut output = String::new();
    let mut count = 1;

    //println!("checkpoint 1");

    if clapargs.position >= 0.0 && clapargs.position <= 1.0 {
        let ratio = 380.0 + ( (780.0-380.0) * clapargs.position );
        let (r, g, b) = rgb_scale(ratio, gamma, clapargs.brightness);


        println!("{r} {g} {b}");
        return;
    }

    let mut input_string = clapargs.text;

    {
        let mut inpipe = std::io::stdin();

        //println!("checkpoint 2");

        let mut pipestring = String::new();
        //let mut pipestring = inpipe.lines().collect::<Result<String, std::io::Error>>();

        use std::io::IsTerminal;
        if !inpipe.is_terminal() {
            let _ = inpipe.read_to_string(&mut pipestring);
            if !pipestring.is_empty() {
                //println!("DEBUG: got non-empty string");
                pipestring.shrink_to_fit();
                input_string = pipestring.clone();
            } else {
                //println!("checkpoint 3");

                //input_string = clapargs.text.clone();
            }

        }
    }
/*
    match inpipe.read_line(&mut pipestring) {
        Ok(0) => {},
        _e => {
        println!("{}", _e.unwrap());
            while
                if let Err(e) = inpipe.read_line(&mut pipestring) { 
                    true 
                } else {
                    false
                } {}
        }
        
    }
    */

    //println!("checkpoint 2.1");


    //println!("checkpoint 4");

    //let pipestring = 

    //println!("DEBUG: \n{}", pipestring.unwrap_or_else(|_| {String::new()}));

    let mut char_count = input_string.chars().count() - 
        match input_string.chars().last().unwrap_or(' ') {
            '\n' => 1,
            _ => 0
        };

    //println!("{char_count}");

    for ch in input_string.chars() {
        if ch == ' ' {
            output += " ";
        } else if ch == '\n' {
            output += "\n";
            char_count -= 1;
            continue;
        } else {

            let ratio =
                380.0 + 
                ( ( (780.0-380.0) / (char_count+1) as f64)
                * (count as f64) );
            let (r, g, b) = rgb_scale(ratio, gamma, clapargs.brightness);
       
            output += format!(
                "\x1b[38;2;{};{};{}m{}",
                r, g, b, ch
            ).as_str();
        }

        count += 1;
    }

    print!("{}{}", output,
        match output.chars().last().unwrap_or(' ') {
            '\n' => "",
            _ => "\n"
        }
    );
}
