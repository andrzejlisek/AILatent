//use wa_bindgen::prelude::*;
use wasm_bindgen::prelude::*;

mod core;
mod config_file;
mod tools;
mod generate_engine;
mod generate_stage;
mod generate_stage_comfyui;
mod generate_stage_comfyui_json;

use crate::core::Core;

#[wasm_bindgen]
pub struct Environment
{
    core: Core,
}

#[wasm_bindgen]
impl Environment
{
    pub fn new() -> Environment
    {
        let self_ = Environment
        {
            core: Core::new()
        };
        self_
    }


    pub fn data(&mut self, op: String, stage: i32, param1: String, param2: String, param3: String) -> String
    {
        self.core.data(op, stage, param1, param2, param3)
    }


    pub fn startproc(&mut self, batch_w: i32, batch_h: i32, indices: String)
    {
        self.core.start(batch_w, batch_h, indices);
    }

    pub fn callback(&mut self, param: String)
    {
        self.core.callback(param);
    }
}

#[macro_export]
macro_rules! js_print
{
    ($($arg:tt)*) => {
        $crate::js_log(&format!($($arg)*))
    };
}


#[wasm_bindgen]
extern {
    pub fn js_exec(s: &str);
}


#[wasm_bindgen]
extern {
    pub fn js_log(s: &str);
}

#[wasm_bindgen]
extern {
    pub fn js_alert(s: &str);
}

#[wasm_bindgen]
extern {
    pub fn js_uuid() -> String;
}

