use crate::{js_exec, js_print, tools};
use crate::generate_stage::{GenerateStage, GenerateStageUndoRedoInputTypeDef};
use crate::generate_stage::GenerateStageElement;

impl GenerateStage
{
    pub fn prepare_process_bypass(&mut self, _batch_w: i32, _batch_h: i32)
    {
        self.prepare_element_exec(1);

        self.prepare_process_getmask(1);
        self.prepare_process_getmask(2);
        self.prepare_process_getmask(3);

        if self.undo_redo_curr.process_source_type == GenerateStageUndoRedoInputTypeDef::Latent
        {

        }
        else
        {
            /*for i in 0..self.image_bitmap_i.len()
            {
                let mut elem = GenerateStageElement::new();
                elem.type_string = String::from("process_bypass_bitmap_latent");
                elem.param1i = i as i32;
                self.execute_elements.push(elem);
            }*/
        }

        self.prepare_element_exec(2);

        for i in 0..self.image_bitmap_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_bypass_paint");
            elem.param1i = i as i32;
            self.execute_elements.push(elem);
        }

        self.prepare_element_exec(3);

        if self.undo_redo_curr.process_source_type == GenerateStageUndoRedoInputTypeDef::Latent
        {

        }
        else
        {
            /*for i in 0..self.image_bitmap_i.len()
            {
                let mut elem = GenerateStageElement::new();
                elem.type_string = String::from("process_bypass_latent_bitmap");
                elem.param1i = i as i32;
                self.execute_elements.push(elem);
            }*/
        }

        self.prepare_element_exec(9);
    }


    pub fn execute_bypass(&mut self, elem_idx: usize, callback_data: &String) -> bool
    {
        let elem = &self.execute_elements[elem_idx];

        match elem.type_string.as_str()
        {
            "" => {
                js_exec("");
                true
            },
            "process_bypass_bitmap_latent" => {
                self.image_latent_i[elem.param1i as usize] = self.image_bitmap_i[elem.param1i as usize].to_string();
                js_exec("");
                true
            },
            "process_bypass_paint" => {
                let idx = elem.param1i as usize;

                self.image_bitmap_o[idx] = self.image_bitmap_i[idx].to_string();
                self.image_latent_o[idx] = self.image_latent_i[idx].to_string();

                if idx == 0
                {
                    let img_temp = GenerateStage::graph_file_to_imag(&self.image_bitmap_o[0]);
                    self.image_w_o = img_temp.width() as i32;
                    self.image_h_o = img_temp.height() as i32;
                }

                js_exec("");
                true
            },
            "process_bypass_latent_bitmap" => {
                self.image_bitmap_o[elem.param1i as usize] = self.image_latent_o[elem.param1i as usize].to_string();

                if elem.param1i == 0
                {
                    let img_temp = GenerateStage::graph_file_to_imag(&self.image_bitmap_o[0]);
                    self.image_w_o = img_temp.width() as i32;
                    self.image_h_o = img_temp.height() as i32;
                }

                js_exec("");
                true
            },
            "process_mask_begin" => {
                if elem.param1i == 11
                {
                    js_exec(&format!("MASK\n{}\n{}\n{}\n{}", self.exec_idx, self.undo_redo_curr.image_input_w, self.undo_redo_curr.image_input_h, elem.param1i - 10));
                }
                else
                {
                    js_exec(&format!("MASK\n{}\n{}\n{}\n{}", self.exec_idx, self.image_w_i, self.image_h_i, elem.param1i));
                }
                true
            },
            "process_mask_end" => {
                if elem.param1i == 11 { self.work_mask_file_1 = callback_data.to_string(); }
                if elem.param1i == 1 { self.work_mask_file_1 = callback_data.to_string(); }
                if elem.param1i == 2 { self.work_mask_file_2 = callback_data.to_string(); }
                if elem.param1i == 3 { self.work_mask_file_3 = callback_data.to_string(); }
                js_exec("");
                true
            },
            "exec" => {
                match elem.param1i {
                    1 => { self.execution_state = crate::generate_stage::GenerateStageExecutionState::Input },
                    2 => { self.execution_state = crate::generate_stage::GenerateStageExecutionState::Working },
                    3 => { self.execution_state = crate::generate_stage::GenerateStageExecutionState::Output },
                    _ => { self.execution_state = crate::generate_stage::GenerateStageExecutionState::Idle },
                }
                js_exec(&format!("EXEC{}{}", elem.param1i, self.exec_idx));
                true
            },
            _ => false,
        }
    }
}