use serde_json::Value;

use std::{io::{self, Write}, thread, time::Duration};
use std::cell::RefCell;
use crate::{core::Core, generate_stage::GenerateStage};

mod core;
mod config_file;
mod tools;
mod generate_engine;
mod generate_stage;
mod generate_stage_graph;
mod generate_stage_bypass;
mod generate_stage_comfyui;
mod generate_stage_comfyui_json;
mod generate_stage_a1111;

static mut GLOBAL_CORE: *mut Core = std::ptr::null_mut();

thread_local! {
    static EVENT_QUEUE: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

pub fn js_uuid() -> String
{
    String::from("10000000-1000-4000-8000-100000000000")
}

pub fn js_exec(s: &str)
{
    println!("Test JavaScript {}", s);
    /*unsafe {
        if !GLOBAL_CORE.is_null() {
            (*GLOBAL_CORE).callback();
        } else {
            println!("BŁĄD: GLOBAL_CORE nie jest zainicjalizowany!");
        }
    }*/
    EVENT_QUEUE.with(|q| {
        q.borrow_mut().push(s.to_string());
    });
}

pub fn js_log(s: &str)
{
    println!("{}", s);
}

pub fn js_alert(s: &str)
{
    println!("{}", s);
}

#[macro_export]
macro_rules! js_print
{
    ($($arg:tt)*) => {
        $crate::js_log(&format!($($arg)*))
    };
}


fn json_test()
{

let json_data = r#"
{
  "klucz1": {
    "nazwa": "abc"
  },
  "klucz2": {
    "nazwa": "def"
  }
}
"#;

// 1. Parsowanie tekstu do obiektu JSON (typu Value)
let json_obj: Value = serde_json::from_str(json_data).expect("Błąd parsowania JSON");

// 2. Odczytywanie wartości
// W Rust używamy operatora indeksowania [].
// Ponieważ Rust jest silnie typowany, na koniec musimy powiedzieć,
// jakiego typu oczekujemy (np. as_str()) i rozpakować wynik (unwrap).

let nazwa1 = json_obj["klucz1"]["nazwa"].as_str().unwrap();
let nazwa2 = json_obj["klucz2"]["nazwa"].as_str().unwrap();

println!("Klucz 1: {}", nazwa1); // Wypisze: abc
println!("Klucz 2: {}", nazwa2); // Wypisze: def


}


fn test_redirect()
{
    let batch_w = 3;
    let batch_h = 2;
    let image_input_col = 4;
    let image_input_row = 0;

    for i in 0..(2 * batch_w * batch_h)
    {
        let mut idx = i % (batch_w * batch_h);
        let sel_row = (image_input_row - 1) % batch_h;
        let sel_col = (image_input_col - 1) % batch_w;
        if (image_input_row > 0) && (image_input_col > 0)
        {
            idx = sel_row * batch_w + sel_col;
        }
        else
        {
            if image_input_row > 0
            {
                idx = (i % batch_w) + (sel_row * batch_w);
            }
            if image_input_col > 0
            {
                idx = ((i / batch_w) % batch_h) * batch_w + sel_col;
            }
        }
        println!("{}. {}", i, idx);
    }
}


fn test_json_tag()
{
    //let mut json = "{\"qqq\":\"##PromptPosi##\"}".to_string();
    //json = GenerateStage::json_tag(json, "##PromptPosi##".to_string(), "Poznań city".to_string());
    //println!("{}", json);

/*


            let mut json = include_str!("comfy_paint.json").to_string();
            //let mut json = include_str!("comfy_paintMask.json").to_string();

            json = GenerateStage::json_insert(json, "Memory".to_string(), "comfy_memory.json".to_string(), "Memo".to_string());
            json = GenerateStage::json_variant(json, "CheckpointMemo".to_string(), false);

            // Do PaintStep
            json = GenerateStage::json_variant(json, "SplitModelMemo".to_string(), false);

            // Do zamiany Bitmap i Latent
            //json = GenerateStage::json_variant(json, "SplitModelMemo".to_string(), !self.engine.model_ckpt);
            /*json = GenerateStage::json_variant(json, "Checkpoint".to_string(), self.engine.model_ckpt);
            json = GenerateStage::json_variant(json, "SplitModel".to_string(), !self.engine.model_ckpt);

            json = GenerateStage::json_tag(json, "##ModelNameDiff##".to_string(), self.engine.model_diffusion.to_string());
            json = GenerateStage::json_tag(json, "##ModelNameText##".to_string(), self.engine.model_text.to_string());
            json = GenerateStage::json_tag(json, "##ModelNameVae##".to_string(), self.engine.model_vae.to_string());*/

            js_print!("Kazwa: [{}]", self.undo_redo_curr.prompt_posi.to_string());
            let rrrrrrrrrr = "Poznań City Center".to_string();
            //json = GenerateStage::json_tag(json, "##PromptPosi##".to_string(), self.undo_redo_curr.prompt_posi.to_string());
            json = GenerateStage::json_tag(json, "##PromptPosi##".to_string(), rrrrrrrrrr);
            //json = GenerateStage::json_tag(json, "##PromptNega##".to_string(), self.undo_redo_curr.prompt_nega.to_string());
            //json = GenerateStage::json_tag(json, "##sampler##".to_string(), self.comfyui_names_sampler.get(&self.undo_redo_curr.sampler).unwrap().to_string());
            //json = GenerateStage::json_tag(json, "##scheduler##".to_string(), self.comfyui_names_scheduler.get(&self.undo_redo_curr.scheduler).unwrap().to_string());
            //json = GenerateStage::json_tag(json, "##sampler##".to_string(), "dpmpp_2m".to_string());
            //json = GenerateStage::json_tag(json, "##scheduler##".to_string(), "karras".to_string());

            json = GenerateStage::json_repeat(json, self.batch_s as i32, &self);
            json = GenerateStage::json_clean(json);

            println!("Zapytanie {}", &json);


*/

}


pub fn steps_denoise(step_begin: i32, step_total: i32, digits: u32) -> String
{
    let factor: i32 = 10_i32.pow(digits);
    let denoise = if step_total > 0
    {
        (factor - (step_begin * factor / step_total)).min(factor)
    }
    else
    {
        factor
    };
    tools::int_to_str(denoise, 0 - (digits as i32))
}

fn main() {

    let v = steps_denoise(15, 20, 2);

    print!("{}", v);

    return;


    let t = -3;
    println!("{}", tools::int_to_str(1, t));
    println!("{}", tools::int_to_str(12, t));
    println!("{}", tools::int_to_str(123, t));
    println!("{}", tools::int_to_str(1234, t));
    println!("{}", tools::int_to_str(12345, t));

    return;

    // Od 10 do 0
    for x in (0..=10).rev() {
        println!("{}", x);
    }


    // Od 9 do 0
    for x in (0..10).rev() {
        println!("{}", x);
    }

    //test_redirect();
    //test_json_tag();

    let tekst = "Dworzec\nPoznań Główny 🚂";
    let bezpieczny_json = tools::json_text_to_ascii(tekst);

    println!("Oryginał: {}", tekst);
    println!("Do JSON:  {}", bezpieczny_json);


    return;

    let xxx = "qqq\nwww\"cc\"www\neee".to_string();
    let yyy = tools::multiline_encode(&xxx);
    let zzz = tools::multiline_decode(&yyy);
    println!("{}", xxx);
    println!("{}", zzz);
    //json_test();
    return;


    //js_print!("{}", include_str!("comfy_convLatentToBitmap.json"));
    let mut eeee = String::from("\"##TILE##\":\"\",\nqqqq\n\n\"vae_name\": \"##ModelNameVae##\"\naaasss");
    eeee = GenerateStage::json_templ(eeee, "##TILE##".to_string(), "NowyMoDEL".to_string());
    eeee = GenerateStage::json_tag(eeee, "##ModelNameVae##".to_string(), "NowyMoDEL".to_string());
    print!("{}", eeee);
    return;
    js_print!("START");
    let mut c = Core::new();
    unsafe {
        GLOBAL_CORE = &mut c as *mut Core;
    }
    c.start(3, 2, String::from("XX"), 0);


    loop {
        // Sprawdzamy, czy w kolejce są jakieś zadania od JavaScriptu
        let pending_task = EVENT_QUEUE.with(|q| {
            let mut queue = q.borrow_mut();
            if queue.is_empty() {
                None
            } else {
                // Pobieramy najstarsze zadanie (FIFO)
                Some(queue.remove(0))
            }
        });

        match pending_task {
            Some(_s) => {
                // STOS JEST TUTAJ CAŁKOWICIE PUSTY!
                // Wywołujemy callback dokładnie tak, jak robi to przeglądarka
                // po zakończeniu asynchronicznego fetch().
                unsafe {
                    if !GLOBAL_CORE.is_null() {
                        (*GLOBAL_CORE).callback(String::from("Powrot"));
                    } else {
                        println!("BŁĄD: GLOBAL_CORE nie jest zainicjalizowany!");
                    }
                }
            }
            None => {
                // Kolejka jest pusta, nie ma więcej asynchronicznych wywołań.
                // Kończymy pętlę zdarzeń.
                break;
            }
        }
    }














    unsafe {
        GLOBAL_CORE = std::ptr::null_mut();
    }
    js_print!("STOP");

    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(100));
}
