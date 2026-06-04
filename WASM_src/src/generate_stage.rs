use std::collections::HashMap;
use crate::{config_file::ConfigFile, generate_engine::GenerateEngine, js_exec, tools};

#[derive(Debug, Clone, PartialEq)]
pub enum GenerateStageUndoRedoInputTypeDef { Latent, Bitmap, None, LatentGray }

#[derive(Debug, Clone, PartialEq)]
pub enum GenerateStageUndoRedoStageTypeDef { Input, Process }

#[derive(Debug, Clone, PartialEq)]
pub enum GenerateStageExecutionState { Idle, Input, Working, Output }


pub struct GenerateStageUndoRedo
{
    pub stage_type: GenerateStageUndoRedoStageTypeDef,

    pub image_input_w: i32,
    pub image_input_h: i32,
    pub image_input_file: String,
    pub image_input_row: i32,
    pub image_input_col: i32,
    pub image_input_zoom: i32,
    pub image_input_offsetx: i32,
    pub image_input_offsety: i32,

    pub process_source: i32,
    pub process_source_type: GenerateStageUndoRedoInputTypeDef,
    pub engine_name: String,
    pub sampler: String,
    pub scheduler: String,

    pub process_begin: bool,
    pub process_end: bool,

    pub prompt_posi: String,
    pub prompt_nega: String,

    pub seed_value: i32,
    pub seed_increment: i32,
    pub seed_direction: i32,

    pub cfg_value: i32,
    pub cfg_increment: i32,
    pub cfg_direction: i32,

    pub steps_total_value: i32,
    pub steps_total_increment: i32,
    pub steps_total_direction: i32,

    pub steps_begin_value: i32,
    pub steps_begin_increment: i32,
    pub steps_begin_direction: i32,

    pub steps_end_value: i32,
    pub steps_end_increment: i32,
    pub steps_end_direction: i32,
}

impl GenerateStageUndoRedo
{
    pub fn new() -> GenerateStageUndoRedo
    {
        let self_ = GenerateStageUndoRedo {
            stage_type : GenerateStageUndoRedoStageTypeDef::Input,
            image_input_row: 0,
            image_input_col: 0,
            image_input_w: 1024,
            image_input_h: 1024,
            image_input_file: String::new(),
            image_input_zoom: 100,
            image_input_offsetx: 0,
            image_input_offsety: 0,
            process_begin: true,
            process_end: true,
            process_source_type: GenerateStageUndoRedoInputTypeDef::Bitmap,
            process_source: 1,
            engine_name: String::new(),
            prompt_posi: String::new(),
            prompt_nega: String::new(),
            sampler: String::from("DPM++ 2M"),
            scheduler: String::from("Karras"),
            seed_value: 0,
            seed_increment: 1,
            seed_direction: 0,
            cfg_value: 70,
            cfg_increment: 0,
            cfg_direction: 0,
            steps_total_value: 20,
            steps_total_increment: 0,
            steps_total_direction: 0,
            steps_begin_value: 0,
            steps_begin_increment: 0,
            steps_begin_direction: 0,
            steps_end_value: 20,
            steps_end_increment: 0,
            steps_end_direction: 0,
        };
        self_
    }

    pub fn new_from(obj: &GenerateStageUndoRedo) -> GenerateStageUndoRedo
    {
        let self_ = GenerateStageUndoRedo {
            image_input_row: obj.image_input_row,
            image_input_col: obj.image_input_col,
            process_source_type: obj.process_source_type.clone(),
            process_source: obj.process_source,
            stage_type : obj.stage_type.clone(),
            image_input_w: obj.image_input_w,
            image_input_h: obj.image_input_h,
            image_input_file: obj.image_input_file.clone(),
            image_input_zoom: obj.image_input_zoom,
            image_input_offsetx: obj.image_input_offsetx,
            image_input_offsety: obj.image_input_offsety,
            engine_name: obj.engine_name.clone(),
            sampler: obj.sampler.clone(),
            scheduler: obj.scheduler.clone(),
            process_begin: obj.process_begin,
            process_end: obj.process_end,
            prompt_posi: obj.prompt_posi.clone(),
            prompt_nega: obj.prompt_nega.clone(),
            seed_value: obj.seed_value,
            seed_increment: obj.seed_increment,
            seed_direction: obj.seed_direction,
            cfg_value: obj.cfg_value,
            cfg_increment: obj.cfg_increment,
            cfg_direction: obj.cfg_direction,
            steps_total_value: obj.steps_total_value,
            steps_total_increment: obj.steps_total_increment,
            steps_total_direction: obj.steps_total_direction,
            steps_begin_value: obj.steps_begin_value,
            steps_begin_increment: obj.steps_begin_increment,
            steps_begin_direction: obj.steps_begin_direction,
            steps_end_value: obj.steps_end_value,
            steps_end_increment: obj.steps_end_increment,
            steps_end_direction: obj.steps_end_direction,
        };
        self_
    }

    pub fn compare_equals(&self, obj: &GenerateStageUndoRedo) -> bool
    {
        if self.process_source_type != obj.process_source_type { return false; }
        if self.process_source != obj.process_source { return false; }
        if self.stage_type != obj.stage_type { return false; }
        if self.image_input_w != obj.image_input_w { return false; }
        if self.image_input_h != obj.image_input_h { return false; }
        if self.image_input_file != obj.image_input_file { return false; }
        if self.engine_name != obj.engine_name { return false; }
        if self.sampler != obj.sampler { return false; }
        if self.scheduler != obj.scheduler { return false; }
        if self.process_begin != obj.process_begin { return false; }
        if self.process_end != obj.process_end { return false; }
        if self.prompt_posi != obj.prompt_posi { return false; }
        if self.prompt_nega != obj.prompt_nega { return false; }
        if self.seed_value != obj.seed_value { return false; }
        if self.seed_increment != obj.seed_increment { return false; }
        if self.seed_direction != obj.seed_direction { return false; }
        if self.cfg_value != obj.cfg_value { return false; }
        if self.cfg_increment != obj.cfg_increment { return false; }
        if self.cfg_direction != obj.cfg_direction { return false; }
        if self.steps_total_value != obj.steps_total_value { return false; }
        if self.steps_total_increment != obj.steps_total_increment { return false; }
        if self.steps_total_direction != obj.steps_total_direction { return false; }
        if self.steps_begin_value != obj.steps_begin_value { return false; }
        if self.steps_begin_increment != obj.steps_begin_increment { return false; }
        if self.steps_begin_direction != obj.steps_begin_direction { return false; }
        if self.steps_end_value != obj.steps_end_value { return false; }
        if self.steps_end_increment != obj.steps_end_increment { return false; }
        if self.steps_end_direction != obj.steps_end_direction { return false; }
        true
    }
}


pub struct GenerateStageElement
{
    pub type_string: String,
    pub param1i: i32,
    pub param2i: i32,
    pub param1s: String,
}

impl GenerateStageElement
{
    pub fn new() -> GenerateStageElement
    {
        let self_ = GenerateStageElement {
            type_string: String::new(),
            param1i: 0,
            param2i: 0,
            param1s: String::new(),
        };
        self_
    }
}

pub struct GenerateStage
{
    pub comfyui_names_server: String,
    pub comfyui_names_sampler: HashMap<String, String>,
    pub comfyui_names_scheduler: HashMap<String, String>,


    pub undo_redo_curr: GenerateStageUndoRedo,
    pub undo_redo_u: Vec<GenerateStageUndoRedo>,
    pub undo_redo_r: Vec<GenerateStageUndoRedo>,

    // pub gsi: GenerateStageInpaint,

    pub execution_state: GenerateStageExecutionState,

    pub engine: GenerateEngine,
    pub batch_w: usize,
    pub batch_h: usize,
    pub batch_s: usize,

    pub exec_idx: usize,

    pub image_w_i: i32,
    pub image_h_i: i32,
    pub image_bitmap_i: Vec<String>,
    pub image_latent_i: Vec<String>,
    pub image_latent_o: Vec<String>,
    pub image_bitmap_o: Vec<String>,
    pub image_w_o: i32,
    pub image_h_o: i32,

    pub execute_state: usize,

    pub execute_elements: Vec<GenerateStageElement>,

    pub temp_file_name_work: String,

}

impl GenerateStage
{
    pub fn new_copy(&mut self) -> GenerateStage
    {
        let mut self_ = GenerateStage::new(self.comfyui_names_server.clone());
        self_.sampler_scheduler();
        self_.undo_redo_curr = GenerateStageUndoRedo::new();

        self_.undo_redo_curr.stage_type = self.undo_redo_curr.stage_type.clone();

        self_.undo_redo_curr.image_input_w = self.undo_redo_curr.image_input_w;
        self_.undo_redo_curr.image_input_h = self.undo_redo_curr.image_input_h;
        self_.undo_redo_curr.image_input_file = self.undo_redo_curr.image_input_file.clone();
        self_.undo_redo_curr.image_input_row = self.undo_redo_curr.image_input_row;
        self_.undo_redo_curr.image_input_col = self.undo_redo_curr.image_input_col;
        self_.undo_redo_curr.image_input_zoom = self.undo_redo_curr.image_input_zoom;
        self_.undo_redo_curr.image_input_offsetx = self.undo_redo_curr.image_input_offsetx;
        self_.undo_redo_curr.image_input_offsety = self.undo_redo_curr.image_input_offsety;

        self_.undo_redo_curr.process_source_type = self.undo_redo_curr.process_source_type.clone();
        self_.undo_redo_curr.process_source = self.undo_redo_curr.process_source;

        self_.undo_redo_curr.engine_name = self.undo_redo_curr.engine_name.clone();
        self_.undo_redo_curr.sampler = self.undo_redo_curr.sampler.clone();
        self_.undo_redo_curr.scheduler = self.undo_redo_curr.scheduler.clone();

        self_.undo_redo_curr.process_begin = self.undo_redo_curr.process_begin;
        self_.undo_redo_curr.process_end = self.undo_redo_curr.process_end;

        self_.undo_redo_curr.prompt_posi = self.undo_redo_curr.prompt_posi.clone();
        self_.undo_redo_curr.prompt_nega = self.undo_redo_curr.prompt_nega.clone();

        self_.undo_redo_curr.seed_value = self.undo_redo_curr.seed_value;
        self_.undo_redo_curr.seed_increment = self.undo_redo_curr.seed_increment;
        self_.undo_redo_curr.seed_direction = self.undo_redo_curr.seed_direction;

        self_.undo_redo_curr.cfg_value = self.undo_redo_curr.cfg_value;
        self_.undo_redo_curr.cfg_increment = self.undo_redo_curr.cfg_increment;
        self_.undo_redo_curr.cfg_direction = self.undo_redo_curr.cfg_direction;

        self_.undo_redo_curr.steps_total_value = self.undo_redo_curr.steps_total_value;
        self_.undo_redo_curr.steps_total_increment = self.undo_redo_curr.steps_total_increment;
        self_.undo_redo_curr.steps_total_direction = self.undo_redo_curr.steps_total_direction;

        self_.undo_redo_curr.steps_begin_value = self.undo_redo_curr.steps_begin_value;
        self_.undo_redo_curr.steps_begin_increment = self.undo_redo_curr.steps_begin_increment;
        self_.undo_redo_curr.steps_begin_direction = self.undo_redo_curr.steps_begin_direction;

        self_.undo_redo_curr.steps_end_value = self.undo_redo_curr.steps_end_value;
        self_.undo_redo_curr.steps_end_increment = self.undo_redo_curr.steps_end_increment;
        self_.undo_redo_curr.steps_end_direction = self.undo_redo_curr.steps_end_direction;

        self_
    }

    pub fn new(comfyui_names_server_x: String) -> GenerateStage
    {
        let mut self_ = GenerateStage {
            comfyui_names_sampler: HashMap::new(),
            comfyui_names_scheduler: HashMap::new(),
            comfyui_names_server: comfyui_names_server_x,

            exec_idx: 0,
            temp_file_name_work: String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
            execute_state: 0,
            execute_elements: Vec::new(),

            undo_redo_curr: GenerateStageUndoRedo::new(),
            undo_redo_u: Vec::new(),
            undo_redo_r: Vec::new(),
            execution_state: GenerateStageExecutionState::Idle,
            batch_w: 0,
            batch_h: 0,
            batch_s: 0,

            image_w_i: 0,
            image_h_i: 0,
            image_bitmap_i: vec![String::new()],
            image_latent_i: vec![String::new()],
            image_latent_o: vec![String::new()],
            image_bitmap_o: vec![String::new()],
            image_w_o: 0,
            image_h_o: 0,
            engine: GenerateEngine::new_blank(),
        };
        self_.sampler_scheduler();
        self_
    }

    fn sampler_scheduler(&mut self)
    {
        self.comfyui_names_sampler.clear();
        self.comfyui_names_sampler.insert("DPM++ 2M".to_string(), "dpmpp_2m".to_string());
        self.comfyui_names_sampler.insert("Euler".to_string(), "euler".to_string());
        self.comfyui_names_sampler.insert("LCM".to_string(), "lcm".to_string());
        self.comfyui_names_sampler.insert("DPM++ 2M SDE".to_string(), "dpmpp_2m_sde".to_string());
        self.comfyui_names_sampler.insert("DPM++ SDE".to_string(), "dpmpp_sde".to_string());
        self.comfyui_names_sampler.insert("Euler A".to_string(), "euler_ancestral".to_string());

        self.comfyui_names_scheduler.clear();
        self.comfyui_names_scheduler.insert("Karras".to_string(), "karras".to_string());
        self.comfyui_names_scheduler.insert("Simple".to_string(), "simple".to_string());
        self.comfyui_names_scheduler.insert("SGM Uniform".to_string(), "sgm_uniform".to_string());
        self.comfyui_names_scheduler.insert("Normal".to_string(), "normal".to_string());
        self.comfyui_names_scheduler.insert("Exponential".to_string(), "exponential".to_string());
    }

    pub fn undo_redo_clear(&mut self)
    {
        self.undo_redo_u.clear();
        self.undo_redo_r.clear();
    }

    pub fn undo_redo_add(&mut self)
    {
        if self.undo_redo_u.len() < 1
        {
            self.undo_redo_u.push(GenerateStageUndoRedo::new_from(&self.undo_redo_curr));
        }
        else
        {
            if !self.undo_redo_curr.compare_equals(self.undo_redo_u.get(self.undo_redo_u.len() - 1).unwrap())
            {
                self.undo_redo_u.push(GenerateStageUndoRedo::new_from(&self.undo_redo_curr));
            }
        }
    }

    pub fn undo_redo_ins(&mut self)
    {
        if self.undo_redo_r.len() < 1
        {
            self.undo_redo_r.push(GenerateStageUndoRedo::new_from(&self.undo_redo_curr));
        }
        else
        {
            if !self.undo_redo_curr.compare_equals(self.undo_redo_r.get(self.undo_redo_r.len() - 1).unwrap())
            {
                self.undo_redo_r.push(GenerateStageUndoRedo::new_from(&self.undo_redo_curr));
            }
        }
    }

    pub fn undo_redo_prev(&mut self)
    {
        while (self.undo_redo_u.len() > 0) && self.undo_redo_curr.compare_equals(self.undo_redo_u.get(self.undo_redo_u.len() - 1).unwrap())
        {
            self.undo_redo_u.remove(self.undo_redo_u.len() - 1);
        }
        
        if self.undo_redo_u.len() > 0
        {
            self.undo_redo_ins();
            self.undo_redo_curr = GenerateStageUndoRedo::new_from(self.undo_redo_u.get(self.undo_redo_u.len() - 1).unwrap());
            self.undo_redo_ins();
            self.undo_redo_u.remove(self.undo_redo_u.len() - 1);
        }
    }

    pub fn undo_redo_next(&mut self)
    {
        while (self.undo_redo_r.len() > 0) && self.undo_redo_curr.compare_equals(self.undo_redo_r.get(self.undo_redo_r.len() - 1).unwrap())
        {
            self.undo_redo_r.remove(self.undo_redo_r.len() - 1);
        }
        
        if self.undo_redo_r.len() > 0
        {
            self.undo_redo_add();
            self.undo_redo_curr = GenerateStageUndoRedo::new_from(self.undo_redo_r.get(self.undo_redo_r.len() - 1).unwrap());
            self.undo_redo_add();
            self.undo_redo_r.remove(self.undo_redo_r.len() - 1);
        }
    }

    pub fn text_load(&mut self, cf: &ConfigFile, idx: i32)
    {
        let prefix = format!("Stage{}", idx);
        self.undo_redo_curr.stage_type = GenerateStageUndoRedoStageTypeDef::Input;
        if cf.param_get_s(format!("{}Type", prefix)) == "input"
        {
            self.undo_redo_curr.stage_type = GenerateStageUndoRedoStageTypeDef::Input;
        }
        if cf.param_get_s(format!("{}Type", prefix)) == "process"
        {
            self.undo_redo_curr.stage_type = GenerateStageUndoRedoStageTypeDef::Process;
        }
        self.undo_redo_curr.image_input_w = cf.param_get_i(format!("{}ImageW", prefix));
        self.undo_redo_curr.image_input_h = cf.param_get_i(format!("{}ImageH", prefix));
        self.undo_redo_curr.image_input_file = cf.param_get_s(format!("{}ImageFile", prefix));
        self.undo_redo_curr.image_input_row = cf.param_get_i(format!("{}ImageRow", prefix));
        self.undo_redo_curr.image_input_col = cf.param_get_i(format!("{}ImageCol", prefix));
        self.undo_redo_curr.image_input_zoom = cf.param_get_i(format!("{}ImageZoom", prefix));
        self.undo_redo_curr.image_input_offsetx = cf.param_get_i(format!("{}ImageOffsetX", prefix));
        self.undo_redo_curr.image_input_offsety = cf.param_get_i(format!("{}ImageOffsetY", prefix));
        self.undo_redo_curr.process_source_type = match cf.param_get_s(format!("{}ProcessSourceType", prefix)).as_str() {
            "bitmap" => { GenerateStageUndoRedoInputTypeDef::Bitmap },
            "none" => { GenerateStageUndoRedoInputTypeDef::None },
            "latent" => { GenerateStageUndoRedoInputTypeDef::Latent },
            "latent_gray" => { GenerateStageUndoRedoInputTypeDef::LatentGray },
            _ => { GenerateStageUndoRedoInputTypeDef::Bitmap }
        };
        self.undo_redo_curr.process_source = cf.param_get_i(format!("{}ProcessSource", prefix));
        self.undo_redo_curr.process_begin = cf.param_get_b(format!("{}ProcessBegin", prefix));
        self.undo_redo_curr.process_end = cf.param_get_b(format!("{}ProcessEnd", prefix));
        self.undo_redo_curr.engine_name = cf.param_get_s(format!("{}Model", prefix));
        self.undo_redo_curr.sampler = cf.param_get_s(format!("{}Sampler", prefix));
        self.undo_redo_curr.scheduler = cf.param_get_s(format!("{}Scheduler", prefix));
        self.undo_redo_curr.seed_value = cf.param_get_i(format!("{}SeedV", prefix));
        self.undo_redo_curr.seed_increment = cf.param_get_i(format!("{}SeedI", prefix));
        self.undo_redo_curr.seed_direction = cf.param_get_i(format!("{}SeedD", prefix));
        self.undo_redo_curr.cfg_value = cf.param_get_i(format!("{}CfgV", prefix));
        self.undo_redo_curr.cfg_increment = cf.param_get_i(format!("{}CfgI", prefix));
        self.undo_redo_curr.cfg_direction = cf.param_get_i(format!("{}CfgD", prefix));
        self.undo_redo_curr.steps_total_value = cf.param_get_i(format!("{}StepsTotalV", prefix));
        self.undo_redo_curr.steps_total_increment = cf.param_get_i(format!("{}StepsTotalI", prefix));
        self.undo_redo_curr.steps_total_direction = cf.param_get_i(format!("{}StepsTotalD", prefix));
        self.undo_redo_curr.steps_begin_value = cf.param_get_i(format!("{}StepsBeginV", prefix));
        self.undo_redo_curr.steps_begin_increment = cf.param_get_i(format!("{}StepsBeginI", prefix));
        self.undo_redo_curr.steps_begin_direction = cf.param_get_i(format!("{}StepsBeginD", prefix));
        self.undo_redo_curr.steps_end_value = cf.param_get_i(format!("{}StepsEndV", prefix));
        self.undo_redo_curr.steps_end_increment = cf.param_get_i(format!("{}StepsEndI", prefix));
        self.undo_redo_curr.steps_end_direction = cf.param_get_i(format!("{}StepsEndD", prefix));
        self.undo_redo_curr.prompt_posi = tools::multiline_decode(&cf.param_get_s(format!("{}PromptPosi", prefix)));
        self.undo_redo_curr.prompt_nega = tools::multiline_decode(&cf.param_get_s(format!("{}PromptNega", prefix)));
        //GSI.textLoad(CF, prefix + "Inpaint");
    }

    pub fn text_save(&mut self, cf: &mut ConfigFile, idx: i32)
    {
        let prefix = format!("Stage{}", idx);
        match self.undo_redo_curr.stage_type {
            GenerateStageUndoRedoStageTypeDef::Input => {
                cf.param_set_s(format!("{}Type", prefix), "input".to_string());
            },
            GenerateStageUndoRedoStageTypeDef::Process => {
                cf.param_set_s(format!("{}Type", prefix), "process".to_string());
            },
        }

        cf.param_set_i(format!("{}ImageW", prefix), self.undo_redo_curr.image_input_w);
        cf.param_set_i(format!("{}ImageH", prefix), self.undo_redo_curr.image_input_h);
        cf.param_set_s(format!("{}ImageFile", prefix), self.undo_redo_curr.image_input_file.to_string());
        cf.param_set_i(format!("{}ImageRow", prefix), self.undo_redo_curr.image_input_row);
        cf.param_set_i(format!("{}ImageCol", prefix), self.undo_redo_curr.image_input_col);
        cf.param_set_i(format!("{}ImageZoom", prefix), self.undo_redo_curr.image_input_zoom);
        cf.param_set_i(format!("{}ImageOffsetX", prefix), self.undo_redo_curr.image_input_offsetx);
        cf.param_set_i(format!("{}ImageOffsetY", prefix), self.undo_redo_curr.image_input_offsety);
        match self.undo_redo_curr.process_source_type {
            GenerateStageUndoRedoInputTypeDef::None => { cf.param_set_s(format!("{}ProcessSourceType", prefix), "none".to_string()); },
            GenerateStageUndoRedoInputTypeDef::Bitmap => { cf.param_set_s(format!("{}ProcessSourceType", prefix), "bitmap".to_string()); },
            GenerateStageUndoRedoInputTypeDef::Latent => { cf.param_set_s(format!("{}ProcessSourceType", prefix), "latent".to_string()); },
            GenerateStageUndoRedoInputTypeDef::LatentGray => {  cf.param_set_s(format!("{}ProcessSourceType", prefix), "latent_gray".to_string());  },
        }
        cf.param_set_i(format!("{}ProcessSource", prefix), self.undo_redo_curr.process_source);
        cf.param_set_b(format!("{}ProcessBegin", prefix), self.undo_redo_curr.process_begin);
        cf.param_set_b(format!("{}ProcessEnd", prefix), self.undo_redo_curr.process_end);
        cf.param_set_s(format!("{}Model", prefix), self.undo_redo_curr.engine_name.to_string());
        cf.param_set_s(format!("{}Sampler", prefix), self.undo_redo_curr.sampler.to_string());
        cf.param_set_s(format!("{}Scheduler", prefix), self.undo_redo_curr.scheduler.to_string());
        cf.param_set_i(format!("{}SeedV", prefix), self.undo_redo_curr.seed_value);
        cf.param_set_i(format!("{}SeedI", prefix), self.undo_redo_curr.seed_increment);
        cf.param_set_i(format!("{}SeedD", prefix), self.undo_redo_curr.seed_direction);
        cf.param_set_i(format!("{}CfgV", prefix), self.undo_redo_curr.cfg_value);
        cf.param_set_i(format!("{}CfgI", prefix), self.undo_redo_curr.cfg_increment);
        cf.param_set_i(format!("{}CfgD", prefix), self.undo_redo_curr.cfg_direction);
        cf.param_set_i(format!("{}StepsTotalV", prefix), self.undo_redo_curr.steps_total_value);
        cf.param_set_i(format!("{}StepsTotalI", prefix), self.undo_redo_curr.steps_total_increment);
        cf.param_set_i(format!("{}StepsTotalD", prefix), self.undo_redo_curr.steps_total_direction);
        cf.param_set_i(format!("{}StepsBeginV", prefix), self.undo_redo_curr.steps_begin_value);
        cf.param_set_i(format!("{}StepsBeginI", prefix), self.undo_redo_curr.steps_begin_increment);
        cf.param_set_i(format!("{}StepsBeginD", prefix), self.undo_redo_curr.steps_begin_direction);
        cf.param_set_i(format!("{}StepsEndV", prefix), self.undo_redo_curr.steps_end_value);
        cf.param_set_i(format!("{}StepsEndI", prefix), self.undo_redo_curr.steps_end_increment);
        cf.param_set_i(format!("{}StepsEndD", prefix), self.undo_redo_curr.steps_end_direction);
        cf.param_set_s(format!("{}PromptPosi", prefix), tools::multiline_encode(&self.undo_redo_curr.prompt_posi.to_string()));
        cf.param_set_s(format!("{}PromptNega", prefix), tools::multiline_encode(&self.undo_redo_curr.prompt_nega.to_string()));
        //GSI.textSave(CF, prefix + "Inpaint");
    }


    pub fn image_input_source(&self) -> i32
    {
        self.undo_redo_curr.image_input_file.parse::<i32>().unwrap_or_else(|_| 0)
    }


    pub fn prepare_clear(&mut self, _batch_w: i32, _batch_h: i32, _exec_idx: usize)
    {
        self.execute_state = 9999;
        self.execute_elements.clear();
    }

    pub fn prepare(&mut self, batch_w: i32, batch_h: i32, exec_idx: usize)
    {
        self.exec_idx = exec_idx;
        self.batch_w = batch_w as usize;
        self.batch_h = batch_h as usize;
        self.batch_s = (batch_w * batch_h) as usize;
        self.image_w_i = 0;
        self.image_h_i = 0;
        self.image_w_o = 0;
        self.image_h_o = 0;
        if self.image_bitmap_i.len() != self.batch_s
        {
            self.image_bitmap_i = vec![String::new(); (self.batch_s) as usize];
            self.image_latent_i = vec![String::new(); (self.batch_s) as usize];
            self.image_latent_o = vec![String::new(); (self.batch_s) as usize];
            self.image_bitmap_o = vec![String::new(); (self.batch_s) as usize];
        }

        self.execute_state = 0;
        self.execute_elements.clear();

        match self.undo_redo_curr.stage_type
        {
            GenerateStageUndoRedoStageTypeDef::Input => {
                self.prepare_input_comfyui(batch_w, batch_h);
            },
            GenerateStageUndoRedoStageTypeDef::Process => {
                self.prepare_process_comfyui(batch_w, batch_h);
            },
        }
    }

    pub fn execute(&mut self, callback_data: String)
    {
        if self.execute_state < self.execute_elements.len()
        {
            self.execute_comfyui(callback_data);
        }
        else
        {
            self.execute_state = 9999;
            js_exec(&format!("EXEC0{}", self.exec_idx));
        }
    }
}

impl GenerateStage
{
    pub fn select_from_batch(i: i32, batch_w: i32, batch_h: i32, image_input_row: i32, image_input_col: i32) -> i32
    {
        let sel_row = (image_input_row - 1) % batch_h;
        let sel_col = (image_input_col - 1) % batch_w;
        if (image_input_row > 0) && (image_input_col > 0)
        {
            sel_row * batch_w + sel_col
        }
        else
        {
            if image_input_row > 0
            {
                return (i % batch_w) + (sel_row * batch_w);
            }
            if image_input_col > 0
            {
                return ((i / batch_w) % batch_h) * batch_w + sel_col;
            }
            i % (batch_w * batch_h)
        }
    }


    pub fn get_batch_value(&self, idx: i32, v: i32, i: i32, d: i32) -> i32
    {
        let batch_w = self.batch_w as i32;
        let batch_h = self.batch_h as i32;
        if (self.batch_h > 0) && (self.batch_w > 0)
        {
            let idx_h = idx % batch_w;
            let idx_v = idx / batch_w;
            match d
            {
                0 => { return 0.max(v + ((idx_v * batch_w + idx_h) * i)) },
                1 => { return 0.max(v + ((idx_h * batch_h + idx_v) * i)) },
                2 => { return 0.max(v + ((idx_h) * i)) },
                3 => { return 0.max(v + ((idx_v) * i)) },
                _ => {}
            }
        }
        0.max(v + (idx * i))
    }

    pub fn get_batch_seed(&self, i: i32) -> i32
    {
        self.get_batch_value(i, self.undo_redo_curr.seed_value, self.undo_redo_curr.seed_increment, self.undo_redo_curr.seed_direction)
    }

    pub fn get_batch_steps_begin(&self, i: i32) -> i32
    {
        self.get_batch_value(i, self.undo_redo_curr.steps_begin_value, self.undo_redo_curr.steps_begin_increment, self.undo_redo_curr.steps_begin_direction)
    }

    pub fn get_batch_steps_end(&self, i: i32) -> i32
    {
        self.get_batch_value(i, self.undo_redo_curr.steps_end_value, self.undo_redo_curr.steps_end_increment, self.undo_redo_curr.steps_end_direction)
    }

    pub fn get_batch_steps_total(&self, i: i32) -> i32
    {
        self.get_batch_value(i, self.undo_redo_curr.steps_total_value, self.undo_redo_curr.steps_total_increment, self.undo_redo_curr.steps_total_direction)
    }

    pub fn get_batch_cfg(&self, i: i32) -> i32
    {
        self.get_batch_value(i, self.undo_redo_curr.cfg_value, self.undo_redo_curr.cfg_increment, self.undo_redo_curr.cfg_direction)
    }

    pub fn info_data(&self) -> String
    {
        format!("{},{},{}", self.batch_w, self.batch_h, self.comfyui_names_server)
    }

    pub fn info(&self, info_idx: i32) -> String
    {
        let mut info_text = String::new();
        info_text.push_str(&format!("{}. ", info_idx + 1));

        match self.execution_state
        {
            GenerateStageExecutionState::Input => {
                info_text.push_str("Input ");
            },
            GenerateStageExecutionState::Working => {
                info_text.push_str("Working ");
            },
            GenerateStageExecutionState::Output => {
                info_text.push_str("Output ");
            },
            _ => { }
        }

        if (self.image_w_i == self.image_w_o) && (self.image_h_i == self.image_h_o)
        {
            info_text.push_str(&format!("{}x{} - {}x{}  ", self.batch_w, self.batch_h, self.image_w_i, self.image_h_i));
        }
        else
        {
            info_text.push_str(&format!("{}x{} - {}x{} -> {}x{}  ", self.batch_w, self.batch_h, self.image_w_i, self.image_h_i, self.image_w_o, self.image_h_o));
        }
        if self.undo_redo_curr.stage_type == GenerateStageUndoRedoStageTypeDef::Input
        {
            if self.undo_redo_curr.image_input_file.len() == 0
            {
                info_text.push_str("Blank input");
            }
            else
            {
                let num = self.image_input_source();
                if num > 0
                {
                    info_text.push_str("Image redirect from ");
                    info_text.push_str(&format!("{}", info_idx + 1 - num));
                    if (self.undo_redo_curr.image_input_col == 0) && (self.undo_redo_curr.image_input_row == 0)
                    {
                        info_text.push_str(&format!(" - redirect all"));
                    }
                    if (self.undo_redo_curr.image_input_col > 0) && (self.undo_redo_curr.image_input_row == 0)
                    {
                        info_text.push_str(&format!(" - redirect column {}", self.undo_redo_curr.image_input_col));
                    }
                    if (self.undo_redo_curr.image_input_col == 0) && (self.undo_redo_curr.image_input_row > 0)
                    {
                        info_text.push_str(&format!(" - redirect row {}", self.undo_redo_curr.image_input_row));
                    }
                    if (self.undo_redo_curr.image_input_col > 0) && (self.undo_redo_curr.image_input_row > 0)
                    {
                        info_text.push_str(&format!(" - redirect column {} and row {}", self.undo_redo_curr.image_input_col, self.undo_redo_curr.image_input_row));
                    }
                }
                else
                {
                    info_text.push_str("Image file [");
                    info_text.push_str(&self.undo_redo_curr.image_input_file);
                    info_text.push_str("]");
                }
            }
            info_text.push_str(&format!(" - {}x{}", self.undo_redo_curr.image_input_w, self.undo_redo_curr.image_input_h));
        }
        if self.undo_redo_curr.stage_type == GenerateStageUndoRedoStageTypeDef::Process
        {
            info_text.push_str(&self.undo_redo_curr.engine_name);
            info_text.push_str(&format!(" - source: {}", info_idx + 1 - self.undo_redo_curr.process_source));
            if self.undo_redo_curr.process_begin && self.undo_redo_curr.process_end
            {
                info_text.push_str(" - full - ");
            }
            else
            {
                if self.undo_redo_curr.process_begin
                {
                    info_text.push_str(" - begin - ");
                }
                if self.undo_redo_curr.process_end
                {
                    info_text.push_str(" - end - ");
                }
                if self.undo_redo_curr.process_begin == self.undo_redo_curr.process_end
                {
                    info_text.push_str(" - middle - ");
                }
            }
            info_text.push_str(&format!("from {}{}", self.undo_redo_curr.steps_begin_value, tools::val_sign(self.undo_redo_curr.steps_begin_increment, -1)));
            info_text.push_str(&format!(" to {}{}", self.undo_redo_curr.steps_end_value, tools::val_sign(self.undo_redo_curr.steps_end_increment, -1)));
            info_text.push_str(&format!(" of {}{}", self.undo_redo_curr.steps_total_value, tools::val_sign(self.undo_redo_curr.steps_total_increment, -1)));
            info_text.push_str(&format!(" at {}{}", self.undo_redo_curr.cfg_value, tools::val_sign(self.undo_redo_curr.cfg_increment, -1)));
            //String MaskStr = (GSI.existsMask() || GSI.existsGray() || GSI.existsPost()) ? " Inpaint" : " ";
        }
        info_text
    }
}