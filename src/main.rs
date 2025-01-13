use rand::seq::SliceRandom;
use rayon::prelude::*;
use reqwest::{blocking, header};
use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let orig = fs::read_to_string("dist/input.txt")?;
    let langs: Vec<&str> = vec![
        "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "ny", "zh",
        "zh-TW", "co", "hr", "cs", "da", "nl", "en", "eo", "et", "tl", "fi", "fr", "fy", "gl",
        "ka", "de", "el", "gu", "ht", "ha", "haw", "he", "hi", "hmn", "hu", "is", "ig", "id", "ga",
        "it", "ja", "jv", "kn", "kk", "km", "rw", "ko", "ku", "ky", "lo", "la", "lv", "lt", "lb",
        "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn", "my", "ne", "no", "or", "ps", "fa", "pl",
        "pt", "pa", "ro", "ru", "sm", "gd", "sr", "st", "sn", "sd", "si", "sk", "sl", "so", "es",
        "su", "sw", "sv", "tg", "ta", "te", "th", "tr", "uk", "ur", "ug", "uz", "vi", "cy", "xh",
        "yi", "yo", "zu",
    ];

    let mut sects: Vec<String> = orig
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    sects = sects
        .into_par_iter()
        .map(|x| {
            let mut cur = x.to_string();
            println!("{}", cur);

            let mut sl = "en";
            for _ in 0..50 {
                let tl = langs.choose(&mut rand::thread_rng()).unwrap();
                cur = make(sl, tl, cur).unwrap();

                sl = tl;
            }
            
            make(sl, "en", cur).unwrap()
        })
        .collect::<Vec<String>>();

    let data = sects.join("\n\n");
    fs::write("dist/out.txt", data)?;

    Ok(())
}

fn make(sl: &str, tl: &str, cur: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = blocking::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"));

    let res = client
        .get(format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&dt=t&sl={}&tl={}&q={}",
            sl, tl, cur
        ))
        .headers(headers)
        .send()?
        .text()?;

    let fin = serde_json::from_str::<Value>(&res)?[0][0][0]
        .to_string()
        .replace("\\", "")
        .replace("\"", "");
    println!("{} -> {}: {}", sl, tl, fin);

    Ok(fin)
}
