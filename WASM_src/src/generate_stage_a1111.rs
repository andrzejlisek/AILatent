use serde_json::Value;


use crate::{js_exec, js_print, tools};
use crate::generate_stage::{GenerateStage, GenerateStageUndoRedoInputTypeDef};
use crate::generate_stage::GenerateStageElement;
use crate::generate_stage::GenerateStageUndoRedo;

impl GenerateStage
{
    pub fn prepare_process_a1111(&mut self, _batch_w: i32, _batch_h: i32)
    {
        self.prepare_element_exec(1);

        self.prepare_process_getmask(1);
        self.prepare_process_getmask(2);
        self.prepare_process_getmask(3);

        for i in 0..self.image_bitmap_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_bypass_bitmap_latent");
            elem.param1i = i as i32;
            self.execute_elements.push(elem);
        }

        self.prepare_element_exec(2);

        for i in 0..self.image_bitmap_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_a1111_begin");
            elem.param1i = i as i32;
            self.execute_elements.push(elem);

            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_a1111_end");
            elem.param1i = i as i32;
            self.execute_elements.push(elem);
        }

        self.prepare_element_exec(3);

        for i in 0..self.image_bitmap_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_bypass_latent_bitmap");
            elem.param1i = i as i32;
            self.execute_elements.push(elem);
        }

        self.prepare_element_exec(9);
    }

    pub fn execute_a1111(&mut self, elem_idx: usize, callback_data: &String) -> bool
    {
        let elem = &self.execute_elements[elem_idx];

        match elem.type_string.as_str()
        {
            "" => {
                js_exec("");
                true
            },
            "process_a1111_begin" => {
                let idx = elem.param1i as usize;

                let blank_input_image = GenerateStage::graph_is_magic(&self.image_latent_i[idx]);

                let mut json = String::new();
                json.push_str(&format!("{{\n"));
                json.push_str(&format!("\"prompt\": \"##PromptPosi##\",\n"));
                json.push_str(&format!("\"negative_prompt\": \"##PromptNega##\",\n"));
                if blank_input_image
                {
                }
                else
                {
                    json.push_str(&format!("\"init_images\": [\n"));
                    json.push_str(&format!("\"##imageInit##\"\n"));
                    json.push_str(&format!("],\n"));
                }
                if self.work_mask_file_2.len() > 4
                {
                    json.push_str(&format!("\"mask\": \"##imageMask##\",\n"));
                }
                json.push_str(&format!("\"width\": \"##imgW##\",\n"));
                json.push_str(&format!("\"height\": \"##imgH##\",\n"));

                json.push_str(&format!("\"sampler_name\": \"##sampler##\",\n"));
                json.push_str(&format!("\"scheduler\": \"##scheduler##\",\n"));
                json.push_str(&format!("\"schedulers_sigma\": \"##scheduler1##\",\n"));
                json.push_str(&format!("\"schedulers_timestep_spacing\": \"##scheduler2##\",\n"));
                json.push_str(&format!("\"seed\": \"##Seed##\",\n"));
                json.push_str(&format!("\"cfg_scale\": \"##PromptCfg##\",\n"));
                json.push_str(&format!("\"steps\": \"##StepCount##\",\n"));
                json.push_str(&format!("\"denoising_strength\": \"##Denoise##\",\n"));
                json.push_str(&format!("\"sd_model_checkpoint\": \"##ModelNameDiff##\",\n"));
                json.push_str(&format!("\"override_settings\": {{\n"));
                json.push_str(&format!("\"sd_model_checkpoint\": \"##ModelNameDiff##\"\n"));
                json.push_str(&format!("}}\n"));
                json.push_str(&format!("}}\n"));

                json = GenerateStage::json_tag(json, "##PromptPosi##".to_string(), self.undo_redo_curr.prompt_posi.to_string());
                json = GenerateStage::json_tag(json, "##PromptNega##".to_string(), self.undo_redo_curr.prompt_nega.to_string());
                json = GenerateStage::json_tag_i(json, "##imgW##".to_string(), self.image_w_i);
                json = GenerateStage::json_tag_i(json, "##imgH##".to_string(), self.image_h_i);
                json = GenerateStage::json_tag(json, "##sampler##".to_string(), GenerateStageUndoRedo::sampler_scheduler_by_name(12, &self.undo_redo_curr.sampler));
                json = GenerateStage::json_tag(json, "##scheduler##".to_string(), GenerateStageUndoRedo::sampler_scheduler_by_name(22, &self.undo_redo_curr.scheduler));
                json = GenerateStage::json_tag(json, "##scheduler1##".to_string(), GenerateStageUndoRedo::sampler_scheduler_by_name(23, &self.undo_redo_curr.scheduler));
                json = GenerateStage::json_tag(json, "##scheduler2##".to_string(), GenerateStageUndoRedo::sampler_scheduler_by_name(24, &self.undo_redo_curr.scheduler));
                json = GenerateStage::json_tag_(json, "##Seed##".to_string(), format!("{}", tools::int_to_unsigned_long(self.get_batch_seed(elem.param1i))));
                json = GenerateStage::json_tag_(json, "##PromptCfg##".to_string(), tools::int_to_str(self.get_batch_cfg(elem.param1i), 1));

                if self.get_batch_steps_total(elem.param1i) > 0
                {
                    json = GenerateStage::json_tag_i(json, "##StepCount##".to_string(), self.get_batch_steps_total(elem.param1i));
                    json = GenerateStage::json_tag_(json, "##Denoise##".to_string(), GenerateStage::steps_denoise(self.get_batch_steps_begin(elem.param1i), self.get_batch_steps_total(elem.param1i), 2));
                }
                else
                {
                    json = GenerateStage::json_tag_i(json, "##StepCount##".to_string(), 0);
                    json = GenerateStage::json_tag_(json, "##Denoise##".to_string(), GenerateStage::steps_denoise(0, 0, 2));
                }

                json = GenerateStage::json_tag(json, "##ModelNameDiff##".to_string(), self.engine.model_diffusion.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameText##".to_string(), self.engine.model_text.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameVae##".to_string(), self.engine.model_vae.to_string());

                if self.work_mask_file_2.len() > 4
                {
                    json = GenerateStage::json_tag_(json, "##imageMask##".to_string(), format!("\"{}\"", &self.work_mask_file_2));
                }
                if blank_input_image
                {
                    js_print!("{}", json);
                    self.js_exec_comfyui_web("/sdapi/v1/txt2img", "", &json);
                }
                else
                {
                    let bitmap_mask = (elem.param2i >= 10) && (self.work_mask_file_1.len() > 2);

                    let raw = if bitmap_mask
                    {
                        &GenerateStage::graph_mask_1(self.image_bitmap_i[elem.param1i as usize].to_string(), &self.work_mask_file_1, self.mask_color_r, self.mask_color_g, self.mask_color_b)
                    }
                    else
                    {
                        &self.image_bitmap_i[elem.param1i as usize]
                    };
                    json = GenerateStage::json_tag_(json, "##imageInit##".to_string(), format!("\"{}\"", &raw));
                    self.js_exec_comfyui_web("/sdapi/v1/img2img", "", &json);
                }

                true
            },
            "process_a1111_end" => {
                let data_obj: Value = serde_json::from_str(&callback_data).expect("JSON parse error");
                let img_data = data_obj["images"][0].as_str().unwrap();

                let idx = elem.param1i as usize;
                self.image_latent_o[idx] = img_data.to_string();
                js_exec("");
                true
            },
            _ => false,
        }

    }
}
