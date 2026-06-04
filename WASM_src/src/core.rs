use crate::config_file::ConfigFile;
use crate::*;
use crate::generate_engine::GenerateEngine;
use crate::generate_stage::*;

pub struct Core
{
    pub cursor_state: i32,
    pub cf: config_file::ConfigFile,
    pub names_sampler: Vec<String>,
    pub names_scheduler: Vec<String>,
    pub engine_list: Vec<GenerateEngine>,
    pub stage_list: Vec<GenerateStage>,
    pub stage_execute: usize,
}


impl Core
{
    pub fn new() -> Core
    {
        let mut self_ = Core
        {
            cursor_state: 0,
            cf: config_file::ConfigFile::new(),
            names_sampler: vec![],
            names_scheduler: vec![],
            engine_list: vec![],
            stage_list: vec![],
            stage_execute: 0,
        };

        self_.names_sampler.push("Euler".to_string());
        self_.names_sampler.push("DPM++ 2M".to_string());
        self_.names_sampler.push("LCM".to_string());
        self_.names_sampler.push("Euler A".to_string());
        self_.names_sampler.push("DPM++ 2M SDE".to_string());
        self_.names_sampler.push("DPM++ SDE".to_string());
        self_.names_scheduler.push("Karras".to_string());
        self_.names_scheduler.push("Simple".to_string());
        self_.names_scheduler.push("SGM Uniform".to_string());
        self_.names_scheduler.push("Normal".to_string());
        self_.names_scheduler.push("Exponential".to_string());
        self_
    }

    pub fn info(&mut self) -> String
    {
        self.cursor_state = self.cursor_state + 1;
        format!("{}", self.cursor_state)
    }

    pub fn find_engine(&self, name: String) -> &GenerateEngine
    {
        for i in 0..self.engine_list.len()
        {
            return &self.engine_list[i];
        }

        panic!("Unknown engine {}", name);
    }

    pub fn engine_info(&mut self, i: usize, info_type: i32) -> String
    {
        if info_type == 1
        {
            for ii in 0..self.engine_list.len()
            {
                if self.engine_list[ii].name == self.stage_list[i].undo_redo_curr.engine_name
                {
                    let mut s = String::new();
                    s.push_str(&self.engine_list[ii].sampler);
                    s.push_str(" / ");
                    s.push_str(&self.engine_list[ii].scheduler);
                    s.push_str(" / ");
                    s.push_str(&format!("{}", self.engine_list[ii].cfg));
                    s.push_str(" / ");
                    s.push_str(&format!("{}", self.engine_list[ii].steps));
                    return s;
                }
            }
        }

        if info_type == 2
        {
            let step_1 = self.stage_list[i].undo_redo_curr.steps_begin_value;
            let step_2 = self.stage_list[i].undo_redo_curr.steps_total_value;
            let mut s = String::new();
            if step_2 > 0
            {
                let step_3 = (1000 - (step_1 * 1000 / step_2)).min(1000);
                //s.push_str(&format!("1-({}/{}) = {}", step_1, step_2, tools::int_to_str(step_3, -3)));
                s.push_str(&tools::int_to_str(step_3, -3));
            }
            else
            {
                let step_3 = 1000;
                //s.push_str(&format!("1-({}/{}) = {}", step_1, step_2, tools::int_to_str(step_3, -3)));
                s.push_str(&tools::int_to_str(step_3, -3));
            }
            return s;
        }

        if info_type == 3
        {
            let step_1 = self.stage_list[i].undo_redo_curr.steps_begin_value;
            let step_2 = self.stage_list[i].undo_redo_curr.steps_total_value;
            let step_4 = self.stage_list[i].undo_redo_curr.steps_end_value;
            let mut s = String::new();
            let step_begin = if step_1 >= 0 { step_1 } else { 0 };
            let step_end = if step_4 <= step_2 { step_4 } else { step_2 };
            if step_end > step_begin
            {
                s.push_str(&format!("{}", step_end - step_begin));
            }
            else
            {
                s.push_str(&format!("0"));
            }
            return s;
        }


        String::new()
    }

    fn stage_chain(&mut self, idx: usize) -> i32
    {
        let mut souce_offset = 0;
        if self.stage_list[idx].undo_redo_curr.stage_type == GenerateStageUndoRedoStageTypeDef::Input
        {
            souce_offset = self.stage_list[idx].image_input_source();
        }
        if self.stage_list[idx].undo_redo_curr.stage_type == GenerateStageUndoRedoStageTypeDef::Process
        {
            souce_offset = self.stage_list[idx].undo_redo_curr.process_source;
        }

        if (souce_offset > 0) && (souce_offset <= (idx as i32))
        {
            (idx as i32) - souce_offset
        }
        else
        {
            -1
        }
    }

    fn stage_is_ready(&mut self, idx: usize, batch_w: i32, batch_h: i32, img_w: i32, img_h: i32) -> bool
    {
        if idx < self.stage_list.len()
        {
            let stage = &self.stage_list[idx];
            if (stage.batch_w == (batch_w as usize)) && (stage.batch_h == (batch_h as usize))
            {
                if (stage.image_w_o > 0) && (stage.image_h_o > 0) && (stage.image_w_o == img_w) && (stage.image_h_o == img_h)
                {
                    true
                }
                else
                {
                    false
                }
            }
            else
            {
                false
            }
        }
        else
        {
            false
        }
    }

    fn stage_image_size(&mut self, idx: usize, chain: &mut Vec<i32>, width: &mut Vec<i32>, heigh: &mut Vec<i32>)
    {
        let mut chain_work = 100;
        let mut idx_0 = idx as i32;
        let mut idx_chain = self.stage_chain(idx_0 as usize);
        chain.clear();
        chain.push(idx_0);
        width.clear();
        width.push(-1);
        heigh.clear();
        heigh.push(-1);

        while chain_work > 0
        {
            if idx_chain < 0
            {
                let stage = &self.stage_list[idx_0 as usize];
                if stage.undo_redo_curr.stage_type == GenerateStageUndoRedoStageTypeDef::Input
                {
                    for n in (0..chain.len()).rev()
                    {
                        width[n] = stage.undo_redo_curr.image_input_w;
                        heigh[n] = stage.undo_redo_curr.image_input_h;
                    }
                }
                return;
            }
            chain_work = chain_work - 1;
            idx_0 = idx_chain;
            idx_chain = self.stage_chain(idx_0 as usize);
            chain.push(idx_0);
            width.push(-1);
            heigh.push(-1);
        }
    }

    pub fn start(&mut self, batch_w: i32, batch_h: i32, indices: String)
    {
        let indices_array_i = tools::text_to_nums_i(&indices);
        let mut indices_array_w: Vec<usize> = Vec::new();
        let mut indices_array_o: Vec<usize> = Vec::new();

        // create complete batch
        for i in 0..self.stage_list.len()
        {
            if indices_array_i.contains(&(i as i32))
            {
                let mut vector_chain: Vec<i32> = Vec::new();
                let mut vector_w: Vec<i32> = Vec::new();
                let mut vector_h: Vec<i32> = Vec::new();
                self.stage_image_size(i, &mut vector_chain, &mut vector_w, &mut vector_h);

                if vector_w.len() <= 0
                {
                    js_exec(&format!("ERRORIncorrect chain or settings for stage {}", i + 1));
                    return;
                }

                if (vector_w[0] <= 0) || (vector_h[0] <= 0)
                {
                    js_exec(&format!("ERRORIncorrect chain or settings for stage {}", i + 1));
                    return;
                }

                indices_array_w.push(i as usize);
                let mut indices_array_last = 0;

                for n in 0..vector_chain.len()
                {
                    if vector_chain[n] >= 0
                    {
                        if !self.stage_is_ready(vector_chain[n] as usize, batch_w, batch_h, vector_w[n], vector_h[n])
                        {
                            while indices_array_last < n
                            {
                                indices_array_w.push(vector_chain[indices_array_last] as usize);
                                indices_array_last = indices_array_last + 1;
                            }
                            indices_array_w.push(vector_chain[n] as usize);
                        }
                    }
                    else
                    {
                        
                    }
                }
            }
        }

        // create sorrted batch list
        for i in 0..self.stage_list.len()
        {
            if indices_array_w.contains(&i)
            {
                indices_array_o.push(i);
            }
        }

        // prepare batch
        for i in 0..self.stage_list.len()
        {
            for ii in 0..self.engine_list.len()
            {
                if self.engine_list[ii].name == self.stage_list[i].undo_redo_curr.engine_name
                {
                    self.stage_list[i].engine = GenerateEngine::new_clone(&self.engine_list[ii]);
                }
                if indices_array_o.contains(&i)
                {
                    self.stage_list[i].prepare(batch_w, batch_h, i);
                }
                else
                {
                    self.stage_list[i].prepare_clear(batch_w, batch_h, i);
                }
            }
        }

        self.stage_execute = 0;
        self.callback(String::new());
    }

    pub fn stage_execute(&mut self, callback_data: String)
    {
        if self.stage_execute < self.stage_list.len()
        {
            if self.stage_list[self.stage_execute].execute_state == 9999
            {
                self.stage_execute = self.stage_execute + 1;
            }
        }

        if self.stage_execute < self.stage_list.len()
        {
            // Prepare image chain
            if (self.stage_execute > 0) && (self.stage_list[self.stage_execute].execute_state == 0)
            {
                let stage_chain_idx = self.stage_chain(self.stage_execute);
                if stage_chain_idx >= 0
                {
                    let souce_item = stage_chain_idx as usize;
                    self.stage_list[self.stage_execute].image_w_i = self.stage_list[souce_item].image_w_o;
                    self.stage_list[self.stage_execute].image_h_i = self.stage_list[souce_item].image_h_o;
                    self.stage_list[self.stage_execute].image_bitmap_i = self.stage_list[souce_item].image_bitmap_o.clone();
                    self.stage_list[self.stage_execute].image_latent_i = self.stage_list[souce_item].image_latent_o.clone();
                }
            }

            self.stage_list[self.stage_execute].execute(callback_data);
        }
    }

    pub fn callback(&mut self, callback_data: String)
    {
        self.stage_execute(callback_data);
    }

    pub fn get_picture(&mut self, stage: usize, idx: usize, data_type: i32) -> String
    {
        if stage < self.stage_list.len()
        {
            if idx < self.stage_list[stage].batch_s
            {
                match data_type {
                    0 => {
                        if self.stage_list[stage].image_bitmap_i.len() > idx
                        {
                            return self.stage_list[stage].image_bitmap_i[idx].to_string();
                        }
                    },
                    1 => {
                        if self.stage_list[stage].image_bitmap_o.len() > idx
                        {
                            return self.stage_list[stage].image_bitmap_o[idx].to_string();
                        }
                    },
                    _ => { return String::new(); }
                }
            }
        }
        String::new()
    }

    pub fn project_load(&mut self, txt: String)
    {
        let mut cf_x = ConfigFile::new();
        let txt_lines = tools::text_to_lines(&txt);
        cf_x.file_load_start();
        for i in 0..txt_lines.len()
        {
            cf_x.file_load_s(&txt_lines[i]);
        }

        let mut idx = 0;
        self.stage_list.clear();
        while cf_x.param_exists(format!("Stage{}CfgV", idx))
        {
            let mut stage_temp = GenerateStage::new(self.cf.param_get_s("ServerComfyUI".to_string()));
            stage_temp.text_load(&cf_x, idx);
            self.stage_list.push(stage_temp);
            idx = idx + 1;
        }
    }


    pub fn project_save(&mut self) -> String
    {
        let mut cf_x = ConfigFile::new();
        for i in 0..self.stage_list.len()
        {
            self.stage_list[i].text_save(&mut cf_x, i as i32);
        }
        cf_x.file_save_start();
        let mut ss = cf_x.file_save();
        let mut sss = String::new();
        while ss.len() > 0 
        {
            sss.push_str(&ss);
            sss.push_str("\n");
            ss = cf_x.file_save();
        }
        sss
    }

    pub fn data(&mut self, op: String, stage: i32, param1: String, param2: String, _param3: String) -> String
    {
        match op.as_str()
        {
            "config" => {

                let config_text = tools::text_to_lines(&param1);
                self.cf.file_load_start();
                for i in 0..config_text.len()
                {
                    self.cf.file_load_s(&config_text[i].to_string());
                }

                let engine_list_x = tools::text_to_nums_i(&self.cf.param_get_s("EngineList".to_string()));

                for i in 0..engine_list_x.len()
                {
                    let ii = engine_list_x[i];
                    if ii > 0
                    {
                        let t = self.cf.param_get_s(format!("Engine{}Name", ii));
                        if t.len() > 0
                        {
                            self.engine_list.push(GenerateEngine::new(&self.cf, ii));
                        }
                    }
                }


                let mut engines = String::new();
                for i in 0..self.engine_list.len()
                {
                    if i > 0
                    {
                        engines.push_str("\n");
                    }
                    engines.push_str(&self.engine_list[i].name);
                }

                self.stage_list.push(generate_stage::GenerateStage::new(self.cf.param_get_s("ServerComfyUI".to_string())));

                engines
            },
            "get_picture" => {
                self.get_picture(stage as usize, param1.parse().unwrap(), param2.parse().unwrap())
            },
            "project_load" => {
                self.project_load(param1);
                String::new()
            },
            "project_save" => {
                self.project_save()
            },
            "project_stage_data" => {
                if stage < 0
                {
                    format!("{}", self.stage_list.len())
                }
                else
                {
                    self.stage_list[stage as usize].info_data()
                }
            },
            "project_stage" => {
                if stage < 0
                {
                    format!("{}", self.stage_list.len())
                }
                else
                {
                    self.stage_list[stage as usize].info(stage)
                }
            },
            "project_value_get" => {
                let stage_us = stage as usize;
                match param1.as_str()
                {
                    "stage_type" => match self.stage_list[stage_us].undo_redo_curr.stage_type
                    {
                        GenerateStageUndoRedoStageTypeDef::Input => {
                            "0".to_string()
                        },
                        GenerateStageUndoRedoStageTypeDef::Process => {
                            "1".to_string()
                        },
                    },

                    "input_file" => self.stage_list[stage_us].undo_redo_curr.image_input_file.to_string(),
                    "input_width" => self.stage_list[stage_us].undo_redo_curr.image_input_w.to_string(),
                    "input_height" => self.stage_list[stage_us].undo_redo_curr.image_input_h.to_string(),
                    "input_row" => self.stage_list[stage_us].undo_redo_curr.image_input_row.to_string(),
                    "input_col" => self.stage_list[stage_us].undo_redo_curr.image_input_col.to_string(),
                    "input_zoom" => self.stage_list[stage_us].undo_redo_curr.image_input_zoom.to_string(),
                    "input_offsetx" => self.stage_list[stage_us].undo_redo_curr.image_input_offsetx.to_string(),
                    "input_offsety" => self.stage_list[stage_us].undo_redo_curr.image_input_offsety.to_string(),

                    "process_source" => self.stage_list[stage_us].undo_redo_curr.process_source.to_string(),
                    "process_source_type" => match self.stage_list[stage_us].undo_redo_curr.process_source_type
                    {
                        GenerateStageUndoRedoInputTypeDef::Latent => {
                            "1".to_string()
                        },
                        _ => {
                            "0".to_string()
                        }
                    },
                    "process_type" => {
                        let t1 = if self.stage_list[stage_us].undo_redo_curr.process_begin { 1 } else { 0 };
                        let t2 = if self.stage_list[stage_us].undo_redo_curr.process_end { 2 } else { 0 };
                        match t1 + t2
                        {
                            0 => "2",
                            1 => "1",
                            2 => "3",
                            3 => "0",
                            _ => "0"
                        }.to_string()
                    },
                    "process_model" => self.stage_list[stage_us].undo_redo_curr.engine_name.to_string(),
                    "process_recom" => self.engine_info(stage_us, 1),
                    "process_steps" => self.engine_info(stage_us, 3),
                    "process_denoise" => self.engine_info(stage_us, 2),
                    "process_sampler" => self.stage_list[stage_us].undo_redo_curr.sampler.to_string(),
                    "process_scheduler" => self.stage_list[stage_us].undo_redo_curr.scheduler.to_string(),
                    "process_seed_v" => self.stage_list[stage_us].undo_redo_curr.seed_value.to_string(),
                    "process_seed_i" => self.stage_list[stage_us].undo_redo_curr.seed_increment.to_string(),
                    "process_seed_d" => self.stage_list[stage_us].undo_redo_curr.seed_direction.to_string(),
                    "process_cfg_v" => self.stage_list[stage_us].undo_redo_curr.cfg_value.to_string(),
                    "process_cfg_i" => self.stage_list[stage_us].undo_redo_curr.cfg_increment.to_string(),
                    "process_cfg_d" => self.stage_list[stage_us].undo_redo_curr.cfg_direction.to_string(),
                    "process_step_t_v" => self.stage_list[stage_us].undo_redo_curr.steps_total_value.to_string(),
                    "process_step_t_i" => self.stage_list[stage_us].undo_redo_curr.steps_total_increment.to_string(),
                    "process_step_t_d" => self.stage_list[stage_us].undo_redo_curr.steps_total_direction.to_string(),
                    "process_step_b_v" => self.stage_list[stage_us].undo_redo_curr.steps_begin_value.to_string(),
                    "process_step_b_i" => self.stage_list[stage_us].undo_redo_curr.steps_begin_increment.to_string(),
                    "process_step_b_d" => self.stage_list[stage_us].undo_redo_curr.steps_begin_direction.to_string(),
                    "process_step_e_v" => self.stage_list[stage_us].undo_redo_curr.steps_end_value.to_string(),
                    "process_step_e_i" => self.stage_list[stage_us].undo_redo_curr.steps_end_increment.to_string(),
                    "process_step_e_d" => self.stage_list[stage_us].undo_redo_curr.steps_end_direction.to_string(),

                    "process_prompt_posi" => self.stage_list[stage_us].undo_redo_curr.prompt_posi.to_string(),
                    "process_prompt_nega" => self.stage_list[stage_us].undo_redo_curr.prompt_nega.to_string(),
                    _ => String::new()
                }
            },
            "project_value_set" => {
                let param2_num = param2.parse::<i32>().unwrap_or_else(|_| 0);
                let stage_us = stage as usize;
                match param1.as_str()
                {
                    "stage_type" => {
                        self.stage_list[stage_us].undo_redo_curr.stage_type = match param2.as_str() {
                            "0" => GenerateStageUndoRedoStageTypeDef::Input,
                            "1" => GenerateStageUndoRedoStageTypeDef::Process,
                            _ => GenerateStageUndoRedoStageTypeDef::Input,
                        };
                    },


                    "input_file" => self.stage_list[stage_us].undo_redo_curr.image_input_file = param2,
                    "input_width" => self.stage_list[stage_us].undo_redo_curr.image_input_w = param2_num,
                    "input_height" => self.stage_list[stage_us].undo_redo_curr.image_input_h = param2_num,
                    "input_row" => self.stage_list[stage_us].undo_redo_curr.image_input_row = param2_num,
                    "input_col" => self.stage_list[stage_us].undo_redo_curr.image_input_col = param2_num,
                    "input_zoom" => self.stage_list[stage_us].undo_redo_curr.image_input_zoom = param2_num,
                    "input_offsetx" => self.stage_list[stage_us].undo_redo_curr.image_input_offsetx = param2_num,
                    "input_offsety" => self.stage_list[stage_us].undo_redo_curr.image_input_offsety = param2_num,

                    "process_source" => self.stage_list[stage_us].undo_redo_curr.process_source = param2_num,
                    "process_source_type" => self.stage_list[stage_us].undo_redo_curr.process_source_type = match param2.as_str() {
                        "1" => GenerateStageUndoRedoInputTypeDef::Latent,
                        _ => GenerateStageUndoRedoInputTypeDef::Bitmap,
                    },
                    "process_type" => {
                        self.stage_list[stage_us].undo_redo_curr.process_begin = (param2_num == 0) || (param2_num == 1);
                        self.stage_list[stage_us].undo_redo_curr.process_end = (param2_num == 0) || (param2_num == 3);
                    },
                    "process_model" => self.stage_list[stage_us].undo_redo_curr.engine_name = param2,
                    "process_sampler" => self.stage_list[stage_us].undo_redo_curr.sampler = param2,
                    "process_scheduler" => self.stage_list[stage_us].undo_redo_curr.scheduler = param2,
                    "process_seed_v" => self.stage_list[stage_us].undo_redo_curr.seed_value = param2_num,
                    "process_seed_i" => self.stage_list[stage_us].undo_redo_curr.seed_increment = param2_num,
                    "process_seed_d" => self.stage_list[stage_us].undo_redo_curr.seed_direction = param2_num,
                    "process_cfg_v" => self.stage_list[stage_us].undo_redo_curr.cfg_value = param2_num,
                    "process_cfg_i" => self.stage_list[stage_us].undo_redo_curr.cfg_increment = param2_num,
                    "process_cfg_d" => self.stage_list[stage_us].undo_redo_curr.cfg_direction = param2_num,
                    "process_step_t_v" => self.stage_list[stage_us].undo_redo_curr.steps_total_value = param2_num,
                    "process_step_t_i" => self.stage_list[stage_us].undo_redo_curr.steps_total_increment = param2_num,
                    "process_step_t_d" => self.stage_list[stage_us].undo_redo_curr.steps_total_direction = param2_num,
                    "process_step_b_v" => self.stage_list[stage_us].undo_redo_curr.steps_begin_value = param2_num,
                    "process_step_b_i" => self.stage_list[stage_us].undo_redo_curr.steps_begin_increment = param2_num,
                    "process_step_b_d" => self.stage_list[stage_us].undo_redo_curr.steps_begin_direction = param2_num,
                    "process_step_e_v" => self.stage_list[stage_us].undo_redo_curr.steps_end_value = param2_num,
                    "process_step_e_i" => self.stage_list[stage_us].undo_redo_curr.steps_end_increment = param2_num,
                    "process_step_e_d" => self.stage_list[stage_us].undo_redo_curr.steps_end_direction = param2_num,

                    "process_prompt_posi" => self.stage_list[stage_us].undo_redo_curr.prompt_posi = param2,
                    "process_prompt_nega" => self.stage_list[stage_us].undo_redo_curr.prompt_nega = param2,
                    _ => {}
                }
                String::new()
            },
            "stage_list" => {
                let stage_us = stage as usize;
                match param1.as_str()
                {
                    "add" => {
                        let stage_temp = self.stage_list[stage_us].new_copy();
                        self.stage_list.push(stage_temp);
                        String::from(format!("{}", self.stage_list.len() - 1))
                    }
                    "rem" => {
                        if (self.stage_list.len() > 1) && (stage >= 0) && (stage_us < self.stage_list.len())
                        {
                            self.stage_list.remove(stage_us);
                            let mut stage_us_x = stage_us;
                            if stage_us_x >= self.stage_list.len()
                            {
                                stage_us_x = self.stage_list.len() - 1;
                            }
                            String::from(format!("{}", stage_us_x))
                        }
                        else
                        {
                            String::from("-1")
                        }
                    }
                    "mv_up" => {
                        if (self.stage_list.len() > 1) && (stage >= 1) && (stage_us < self.stage_list.len())
                        {
                            self.stage_list.swap(stage_us, stage_us - 1);
                            String::from(format!("{}", stage - 1))
                        }
                        else
                        {
                            String::from("-1")
                        }
                    },
                    "mv_dn" => {
                        if (self.stage_list.len() > 1) && (stage >= 0) && (stage_us < (self.stage_list.len() - 1))
                        {
                            self.stage_list.swap(stage_us, stage_us + 1);
                            String::from(format!("{}", stage + 1))
                        }
                        else
                        {
                            String::from("-1")
                        }
                    },
                    _ => String::from("-1")
                }
            },
            _ => String::new()
        }
    }
}
