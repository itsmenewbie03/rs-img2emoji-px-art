use color_name::Color;
use image::GenericImageView;

fn rgb_to_4bit_color_name(r: u8, g: u8, b: u8) -> String {
    let r_4bit = r;
    let g_4bit = g;
    let b_4bit = b;
    Color::similar([r_4bit, g_4bit, b_4bit])
}
fn rgb_to_emoji(r: u8, g: u8, b: u8) -> String {
    // TODO: print color name
    rgb_to_4bit_color_name(r, g, b)
}

fn color_to_emoji(name: &str) -> &str {
    match name {
        "Darkolivegreen" => "🤎",
        "Black" => "🖤",
        "Dimgray" => "🩶",
        "Gray" => "🤍",
        "Maroon" => "❤️",
        "Midnightblue" => "💙",
        "Saddlebrown" => "💛",
        "Darkslategray" => "🩶",
        "Slategray" => "🩶",
        "Sienna" => "🤎",
        "Darkgreen" => "💚",
        _ => name,
    }
}

fn main() {
    let img = image::open("src/anya.png").unwrap();
    let step = 23;
    let (w, h) = img.dimensions();
    // INFO: declare a map Color Name -> Occurence
    let mut color_map: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

    for row in (0..h).step_by(step) {
        let mut line = String::new();
        for col in (0..w).step_by(step) {
            if col >= w {
                let pixel = img.get_pixel(col - 1, row);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let color_name = rgb_to_emoji(r, g, b);
                let key = color_name.clone().to_string();
                let key_ref: &str = color_name.as_str().as_ref();
                color_map.insert(key, color_map.get(key_ref).unwrap_or(&0) + 1);
                line.push_str(color_to_emoji(key_ref));
                continue;
            }
            if row >= h {
                let pixel = img.get_pixel(col, row - 1);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let color_name = rgb_to_emoji(r, g, b);
                let key = color_name.clone().to_string();
                let key_ref: &str = color_name.as_str().as_ref();
                line.push_str(color_to_emoji(key_ref));
                color_map.insert(key, color_map.get(key_ref).unwrap_or(&0) + 1);
                continue;
            }
            let pixel = img.get_pixel(col, row);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let color_name = rgb_to_emoji(r, g, b);
            let key = color_name.clone().to_string();
            let key_ref: &str = color_name.as_str().as_ref();
            line.push_str(color_to_emoji(key_ref));
            color_map.insert(key, color_map.get(key_ref).unwrap_or(&0) + 1);
            continue;
        }
        println!("{}", line);
    }
    // println!("{:?}", color_map);
}
