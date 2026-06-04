pub fn int_to_unsigned_long(val: i32) -> u64
{
    (val as u32) as u64
}

pub fn string_index_of(s: &String, chr: char) -> isize
{
    s.chars().position(|c| c == chr).map(|i| i as isize).unwrap_or(-1)
}

pub fn string_index_of_s(s: &str, substr: &str) -> isize
{
    //s.find(substr).map(|i| i as isize).unwrap_or(-1)
    s.find(substr)
        .map(|byte_idx| s[..byte_idx].chars().count() as isize)
        .unwrap_or(-1)
}


pub fn substring(text: &str, start: usize, end: usize) -> String
{
    text.chars().skip(start).take(end - start).collect()
}

pub fn substring_(text: &str, start: usize) -> String
{
    text.chars().skip(start).collect()
}


pub fn text_to_lines(txt: &String) -> Vec<String>
{
    txt.lines().map(String::from).collect()
}

pub fn text_to_nums_i(txt: &String) -> Vec<i32>
{
    txt.split(',').map(|s| s.parse::<i32>().unwrap_or_else(|_| 0)).collect()
}

/*pub fn text_to_nums_f(txt: &String) -> Vec<f64>
{
    txt.split(',').map(|s| s.parse::<f64>().unwrap_or_else(|_| 0.0)).collect()
}*/

/*pub fn text_to_strings(txt: &String) -> Vec<String>
{
    txt.split(',').map(|s| s.to_string()).collect()
}*/

pub fn get_resource(name: &str) -> &'static str
{
    match name {
        "comfy_convBitmapToLatent.json" => include_str!("comfy_convBitmapToLatent.json"),
        "comfy_convLatentToBitmap.json" => include_str!("comfy_convLatentToBitmap.json"),
        "comfy_latentComposite.json" => include_str!("comfy_latentComposite.json"),
        "comfy_memory.json" => include_str!("comfy_memory.json"),
        "comfy_newImage0.json" => include_str!("comfy_newImage0.json"),
        "comfy_newImage1.json" => include_str!("comfy_newImage1.json"),
        "comfy_paint.json" => include_str!("comfy_paint.json"),
        "comfy_paintMask.json" => include_str!("comfy_paintMask.json"),
        _ => "",
    }
}

pub fn int_to_str(val: i32, digits: i32) -> String
{
    let mut val_str = format!("{}", val);
    let mut val_dec = String::new();
    let mut digits_x = digits;
    while digits_x < 0
    {
        if val_str.len() < ((0 - digits_x) + 1) as usize
        {
            val_str = format!("0{}", val_str);
        }
        val_dec = format!("{}{}", substring_(&val_str, val_str.len() - 1), val_dec);
        val_str = substring(&val_str, 0, val_str.len() - 1);
        digits_x = digits_x + 1;
    }
    while digits_x > 0
    {
        if val_str.len() < (digits_x + 1) as usize
        {
            val_str = format!("0{}", val_str);
        }
        if (val_dec.len() > 0) || (substring_(&val_str, val_str.len() - 1) != "0")
        {
            val_dec = substring_(&val_str, val_str.len() - 1);
        }
        val_str = substring(&val_str, 0, val_str.len() - 1);
        digits_x = digits_x - 1;
    }
    if val_dec.len() > 0
    {
        format!("{}.{}", val_str, val_dec)
    }
    else
    {
        val_str
    }
}

pub fn multiline_encode(x: &str) -> String
{
    let mut xxx = String::new();
    xxx.push_str("\"");
    let x_len = x.chars().count();
    let mut cc = x.chars();
    for _i in 0..x_len
    {
        let c = cc.next().unwrap();
        match c {
            ';' => xxx.push_str("\\#"),
            '\\' => xxx.push_str("\\\\"),
            '\"' => xxx.push_str("\\_"),
            '\'' => xxx.push_str("\\="),
            '\r' => xxx.push_str("\\r"),
            '\n' => xxx.push_str("\\n"),
            '\t' => xxx.push_str("\\t"),
            _ => xxx.push(c),
        }
    }
    xxx.push_str("\"");
    xxx
}

pub fn multiline_decode(x: &str) -> String
{
    if x.starts_with("\"") && x.ends_with("\"")
    {
        let mut xxx = String::new();
        let x_len = x.chars().count() - 2;
        let mut cc = x.chars();
        cc.next();
        let mut in_char = false;
        for _i in 0..x_len
        {
            let c = cc.next().unwrap();
            if in_char
            {
                match c {
                    '#' => xxx.push(';'),
                    '\\' => xxx.push('\\'),
                    '_' => xxx.push('\"'),
                    '=' => xxx.push('\''),
                    'r' => xxx.push('\r'),
                    'n' => xxx.push('\n'),
                    't' => xxx.push('\t'),
                    _ => {}
                }
                in_char = false;
            }
            else
            {
                if c == '\\'
                {
                    in_char = true;
                }
                else
                {
                    xxx.push(c);
                }
            }
        }
        xxx
    }
    else
    {
        x.to_string()
    }
}

pub fn val_sign(val: i32, dec: i32) -> String
{
    if val > 0
    {
        return format!("+{}", (val / dec.abs()));
    }
    if val < 0
    {
        return format!("-{}", ((0 - val) / dec.abs()));
    }
    if dec > 0 { String::from("0") } else { String::from("") }
}

/*pub fn val_decibel(val: i32) -> String
{
    if val >= 201
    {
        return String::from("+inf");
    }
    if val <= -201
    {
        return String::from("-inf");
    }
    format!("{}.{} dB", val_sign(val, 10), val.abs() % 10)
}*/

pub fn json_text_to_ascii(text: &str) -> String
{
    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if c.is_ascii()
        {
            result.push(c);
        }
        else
        {
            let mut buf = [0; 2];
            for &mut unit in c.encode_utf16(&mut buf) {
                result.push_str(&format!("\\u{:04x}", unit));
            }
        }
    }
    result
}

