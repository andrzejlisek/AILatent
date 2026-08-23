use serde_json::Value;


use crate::{js_exec, js_print, js_uuid, tools};
use crate::generate_stage::{GenerateStage, GenerateStageUndoRedoInputTypeDef};
use crate::generate_stage::GenerateStageElement;
use crate::generate_stage::GenerateStageUndoRedo;

impl GenerateStage
{
    fn prepare_element_bitmap_upload(&mut self, file_uuid: &str, mode: i32, use_mask: bool)
    {
        let file_name_bitmap = format!("{}_bitmap", file_uuid);
        let file_name_latent = format!("{}_latent", file_uuid);

        for i in 0..self.image_bitmap_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_upload_begin");
            elem.param1s = file_name_bitmap.to_string();
            if mode == 2 { elem.param1s = file_name_latent.to_string(); }
            elem.param1i = i as i32;
            elem.param2i = mode;
            if use_mask
            {
                elem.param2i = elem.param2i + 10;
            }
            self.execute_elements.push(elem);

            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_upload_end");
            elem.param1s = file_name_bitmap.to_string();
            if mode == 2 { elem.param1s = file_name_latent.to_string(); }
            elem.param1i = i as i32;
            elem.param2i = mode;
            self.execute_elements.push(elem);
        }
    }

    fn prepare_element_latent_upload(&mut self, file_uuid: &str, mode: i32)
    {
        let file_name_latent = format!("{}_latent", file_uuid);

        for i in 0..self.image_latent_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("latent_upload_begin");
            elem.param1s = file_name_latent.to_string();
            elem.param1i = i as i32;
            elem.param2i = mode;
            self.execute_elements.push(elem);

            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("latent_upload_end");
            elem.param1s = file_name_latent.to_string();
            elem.param1i = i as i32;
            elem.param2i = mode;
            self.execute_elements.push(elem);
        }
    }

    fn prepare_element_bitmap_download(&mut self, _file_uuid: &str, mode: i32, use_mask: bool)
    {
        for i in 0..self.image_bitmap_o.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_download_begin");
            elem.param1i = i as i32;
            elem.param2i = mode;
            self.execute_elements.push(elem);

            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_download_end");
            elem.param1i = i as i32;
            elem.param2i = mode;
            if use_mask
            {
                elem.param2i = elem.param2i + 10;
            }
            self.execute_elements.push(elem);
        }
    }

    fn prepare_element_latent_download(&mut self, _file_uuid: &str, mode: i32)
    {
        for i in 0..self.image_latent_i.len()
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("latent_download_begin");
            elem.param1i = i as i32;
            elem.param2i = mode;
            self.execute_elements.push(elem);

            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("latent_download_end");
            elem.param1i = i as i32;
            elem.param2i = mode;
            self.execute_elements.push(elem);
        }
    }

    fn prepare_element_bitmap_to_latent(&mut self, file_uuid: &str, use_mask: bool)
    {
        let file_name_bitmap = format!("{}_bitmap", file_uuid);

        self.prepare_element_bitmap_upload(file_uuid, 0, use_mask);

        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("bitmap_to_latent_begin");
        elem.param1s = file_name_bitmap.to_string();
        self.execute_elements.push(elem);

        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("bitmap_to_latent_end");
        self.execute_elements.push(elem);

        self.prepare_element_latent_download(&file_uuid, 0);
    }

    fn prepare_element_latent_to_bitmap(&mut self, file_uuid: &str, use_mask: bool)
    {
        let file_name_latent = format!("{}_latent", file_uuid);

        self.prepare_element_latent_upload(file_uuid, 0);

        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("latent_to_bitmap_begin");
        elem.param1s = file_name_latent.to_string();
        self.execute_elements.push(elem);

        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("latent_to_bitmap_end");
        self.execute_elements.push(elem);

        self.prepare_element_bitmap_download(&file_uuid, 0, use_mask);
    }


    pub fn prepare_element_exec(&mut self, id: i32)
    {
        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("exec");
        elem.param1i = id;
        self.execute_elements.push(elem);
    }


    pub fn prepare_input_uni(&mut self, batch_w: i32, batch_h: i32)
    {
        self.image_w_i = 0;
        self.image_h_i = 0;
        self.image_w_o = 0;
        self.image_h_o = 0;

        self.prepare_element_exec(1);

        self.prepare_process_getmask(11);

        if self.image_input_source() > 0
        {
            {
                let i = 0;

                let mut elem = GenerateStageElement::new();
                elem.type_string = String::from("bitmap_input_i_begin");
                elem.param1i = GenerateStage::select_from_batch(i, batch_w, batch_h, self.undo_redo_curr.image_input_row, self.undo_redo_curr.image_input_col) + 1;
                elem.param2i = i + 1;
                self.execute_elements.push(elem);
                let mut elem = GenerateStageElement::new();
                elem.type_string = String::from("bitmap_input_i_end");
                elem.param1i = GenerateStage::select_from_batch(i, batch_w, batch_h, self.undo_redo_curr.image_input_row, self.undo_redo_curr.image_input_col) + 1;
                elem.param2i = i + 1;
                self.execute_elements.push(elem);
            }

            for i in 0..(batch_w * batch_h)
            {
                let mut elem = GenerateStageElement::new();
                elem.type_string = String::from("bitmap_input_o_begin");
                elem.param1i = GenerateStage::select_from_batch(i, batch_w, batch_h, self.undo_redo_curr.image_input_row, self.undo_redo_curr.image_input_col) + 1;
                elem.param2i = i + 1;
                self.execute_elements.push(elem);
                let mut elem = GenerateStageElement::new();
                elem.type_string = String::from("bitmap_input_o_end");
                elem.param1i = GenerateStage::select_from_batch(i, batch_w, batch_h, self.undo_redo_curr.image_input_row, self.undo_redo_curr.image_input_col) + 1;
                elem.param2i = i + 1;
                self.execute_elements.push(elem);
            }
        }
        else
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_input_i_begin");
            elem.param1i = 0;
            self.execute_elements.push(elem);
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_input_i_end");
            elem.param1i = 0;
            self.execute_elements.push(elem);

            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_input_o_begin");
            elem.param1i = 0;
            self.execute_elements.push(elem);
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("bitmap_input_o_end");
            elem.param1i = 0;
            self.execute_elements.push(elem);
        }

        self.prepare_element_exec(9);
    }



    pub fn prepare_process_getmask(&mut self, mask_type: i32)
    {
        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_mask_begin");
            elem.param1i = mask_type;
            self.execute_elements.push(elem);
        }

        {
            let mut elem = GenerateStageElement::new();
            elem.type_string = String::from("process_mask_end");
            elem.param1i = mask_type;
            self.execute_elements.push(elem);
        }
    }


    pub fn prepare_process_comfyui(&mut self, _batch_w: i32, _batch_h: i32)
    {
        self.prepare_element_exec(1);

        self.prepare_process_getmask(1);
        self.prepare_process_getmask(2);
        self.prepare_process_getmask(3);


        let mut file_uuid = js_uuid();

        if self.undo_redo_curr.process_source_type == GenerateStageUndoRedoInputTypeDef::Latent
        {
            self.prepare_element_latent_upload(&file_uuid, 1);
        }
        else
        {
            self.prepare_element_bitmap_to_latent(&file_uuid, true);


            file_uuid = js_uuid();
            self.prepare_element_latent_upload(&file_uuid, 1);
        }

        self.prepare_element_bitmap_upload(&file_uuid, 2, false);

        self.prepare_element_exec(2);

        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("paint_begin");
        elem.param1s = format!("{}_latent", file_uuid);
        self.execute_elements.push(elem);

        let mut elem = GenerateStageElement::new();
        elem.type_string = String::from("paint_end");
        self.execute_elements.push(elem);

        self.prepare_element_latent_download(&file_uuid, 1);

        self.prepare_element_exec(3);


        file_uuid = js_uuid();
        self.prepare_element_latent_to_bitmap(&file_uuid, true);

        self.prepare_element_exec(9);
    }


    pub fn js_exec_comfyui_web(&self, path: &str, params: &str, data: &str)
    {
        js_exec(&format!("WEB\n{}\n{}\n{}", format!("{}{}", self.engine.server_addr, path), params, data));
    }

    pub fn js_exec_comfyui_json(&self, json: &String)
    {
        js_exec(&format!("COMFYUI\n{}\nx\n{}", self.engine.server_addr, json));
    }


    fn execute_finish_latent(&mut self, callback_data: &str)
    {
        for i in 0..self.batch_s
        {
            let node_id = format!("{}", i * 10 + 1000);
            let data_obj: Value = serde_json::from_str(callback_data).expect("JSON parse error");
            let img_file_name = data_obj["outputs"][node_id]["latents"][0]["filename"].as_str().unwrap();
            let img_file_url = format!("/view?filename={}&subfolder=&type=output", img_file_name);
            self.execute_elements[self.execute_state + (i * 2)].param1s = img_file_url.to_string();

        }
    }

    fn execute_finish_bitmap(&mut self, callback_data: &str)
    {
        for i in 0..self.batch_s
        {
            let node_id = format!("{}", i * 10 + 1000);
            let data_obj: Value = serde_json::from_str(&callback_data).expect("JSON parse error");
            let img_file_name = data_obj["outputs"][node_id]["images"][0]["filename"].as_str().unwrap();
            let img_file_url = format!("/view?filename={}&subfolder=&type=output", img_file_name);
            self.execute_elements[self.execute_state + (i * 2)].param1s = img_file_url.to_string();
        }
    }

    pub fn execute_dummy(&mut self, callback_data: String)
    {
        self.execute_state = self.execute_state + 1;

        js_exec("");
    }

    pub fn execute_comfyui(&mut self, elem_idx: usize, callback_data: &String) -> bool
    {
        let elem = &self.execute_elements[elem_idx];

        match elem.type_string.as_str()
        {
            "" => {
                js_exec("");
                true
            },
            "bitmap_input_i_begin" | "bitmap_input_o_begin" => {
                let params = if elem.type_string == "bitmap_input_o_begin"
                {
                    format!("{},{},{},{},{},{},{},{}", self.undo_redo_curr.image_input_w, self.undo_redo_curr.image_input_h, self.undo_redo_curr.image_input_zoom, self.undo_redo_curr.image_input_offsetx, self.undo_redo_curr.image_input_offsety, self.undo_redo_curr.image_input_color_r, self.undo_redo_curr.image_input_color_g, self.undo_redo_curr.image_input_color_b)
                }
                else
                {
                    format!("{},{},{},{},{},{},{},{}", 0, 0, 0, 0, 0, 0, 0, 0)
                };

                if self.image_input_source() > 0
                {
                    js_exec(&format!("PICTUREDATA\n{}\n{}\n", params, self.image_bitmap_i[elem.param1i as usize - 1]));
                }
                else
                {
                    js_exec(&format!("PICTUREFILE\n{}\n{}\n", params, self.undo_redo_curr.image_input_file));
                }
                true
            },
            "bitmap_input_i_end" | "bitmap_input_o_end" => {
                let image_file_data = if callback_data.len() > 0
                {
                    callback_data.to_string()
                }
                else
                {
                    GenerateStage::graph_create_magic(self.undo_redo_curr.image_input_w, self.undo_redo_curr.image_input_h)
                };
                let img_temp = GenerateStage::graph_file_to_imag(&image_file_data);
                if elem.type_string == "bitmap_input_o_end"
                {
                    self.image_w_o = img_temp.width() as i32;
                    self.image_h_o = img_temp.height() as i32;
                }
                else
                {
                    self.image_w_i = img_temp.width() as i32;
                    self.image_h_i = img_temp.height() as i32;
                }

                let bitmap_mask = (elem.type_string == "bitmap_input_o_end") && (self.work_mask_file_1.len() > 2);


                if elem.param2i > 0
                {
                    self.image_latent_o[(elem.param2i - 1) as usize] = self.image_latent_i[(elem.param1i - 1) as usize].clone();
                    let raw = if bitmap_mask
                    {
                        GenerateStage::graph_mask_1(image_file_data.to_string(), &self.work_mask_file_1, self.mask_color_r, self.mask_color_g, self.mask_color_b)
                    }
                    else
                    {
                        image_file_data.to_string()
                    };
                    self.image_bitmap_o[(elem.param2i - 1) as usize] = raw;
                }
                else
                {
                    for i in 0..self.batch_s
                    {
                        self.image_latent_o[i] = self.image_latent_i[i].clone();
                        let raw = if bitmap_mask
                        {
                            GenerateStage::graph_mask_1(image_file_data.to_string(), &self.work_mask_file_1, self.mask_color_r, self.mask_color_g, self.mask_color_b)
                        }
                        else
                        {
                            image_file_data.to_string()
                        };
                        self.image_bitmap_o[i] = raw;
                    }
                }
                js_exec("");
                true
            },
            "bitmap_upload_begin" => {
                let bitmap_mode = elem.param2i % 10;
                let bitmap_mask = (elem.param2i >= 10) && (self.work_mask_file_1.len() > 2);
                match bitmap_mode
                {
                    0 => {
                        if self.image_bitmap_i[elem.param1i as usize].len() > 0
                        {
                            let raw = if bitmap_mask
                            {
                                &GenerateStage::graph_mask_1(self.image_bitmap_i[elem.param1i as usize].to_string(), &self.work_mask_file_1, self.mask_color_r, self.mask_color_g, self.mask_color_b)
                            }
                            else
                            {
                                &self.image_bitmap_i[elem.param1i as usize]
                            };
                            self.js_exec_comfyui_web("/upload/image", &format!("X_BINARYI!{}{}.png!", elem.param1s, elem.param1i), raw);
                        }
                        else
                        {
                            js_exec("");
                        }
                    },
                    1 => {
                        if self.image_bitmap_o[elem.param1i as usize].len() > 0
                        {
                            let raw = if bitmap_mask
                            {
                                &GenerateStage::graph_mask_1(self.image_bitmap_o[elem.param1i as usize].to_string(), &self.work_mask_file_1, self.mask_color_r, self.mask_color_g, self.mask_color_b)
                            }
                            else
                            {
                                &self.image_bitmap_o[elem.param1i as usize]
                            };
                            self.js_exec_comfyui_web("/upload/image", &format!("X_BINARYI!{}{}.png!", elem.param1s, elem.param1i), raw);
                        }
                        else
                        {
                            js_exec("");
                        }
                    },
                    2 => {
                        if self.work_mask_file_2.len() > 2
                        {
                            let raw = &self.work_mask_file_2;
                            self.js_exec_comfyui_web("/upload/image", &format!("X_BINARYI!{}{}x.png!", elem.param1s, elem.param1i), raw);
                        }
                        else
                        {
                            js_exec("");
                        }
                    },
                    _ => { js_exec("") },
                }
                true
            },
            "bitmap_upload_end" => {
                js_exec("");
                true
            },
            "latent_upload_begin" => {
                if elem.param2i == 0
                {
                    if self.image_latent_o[elem.param1i as usize].len() > 0
                    {
                        self.js_exec_comfyui_web("/upload/image", &format!("X_BINARYI!{}{}.latent!", elem.param1s, elem.param1i), &self.image_latent_o[elem.param1i as usize]);
                    }
                    else
                    {
                        js_exec("");
                    }
                }
                if elem.param2i == 1
                {
                    if self.image_latent_i[elem.param1i as usize].len() > 0
                    {
                        self.js_exec_comfyui_web("/upload/image", &format!("X_BINARYI!{}{}.latent!", elem.param1s, elem.param1i), &self.image_latent_i[elem.param1i as usize]);
                    }
                    else
                    {
                        js_exec("");
                    }
                }
                true
            },
            "latent_upload_end" => {
                js_exec("");
                true
            },
            "bitmap_to_latent_begin" => {
                self.work_file_name_work = elem.param1s.to_string();

                let mut json = include_str!("comfy_convBitmapToLatent.json").to_string();

                // 0 - latent for SD 1.5, SHXL 1.0
                // 1 - latent for SD 3.5, FLUX
                let latent_type = 0;
                let latent_width = self.image_w_i;
                let latent_height = self.image_h_i;
                let latent_blank = (self.image_bitmap_i[0].len() == 0) || (GenerateStage::graph_is_magic(&self.image_bitmap_i[0]));

                if latent_blank
                {
                    json = tools::get_resource(&format!("comfy_newImage{}.json", latent_type)).to_string();
                }

                json = GenerateStage::json_insert(json, "Memory".to_string(), "comfy_memory.json".to_string(), "Memo".to_string());
                if latent_blank
                {
                    json = GenerateStage::json_variant(json, "CheckpointMemo".to_string(), self.engine.model_ckpt);
                }
                else
                {
                    json = GenerateStage::json_variant(json, "CheckpointMemo".to_string(), false);
                }
                json = GenerateStage::json_variant(json, "SplitModelMemo".to_string(), !self.engine.model_ckpt);
                json = GenerateStage::json_variant(json, "Checkpoint".to_string(), self.engine.model_ckpt);
                json = GenerateStage::json_variant(json, "SplitModel".to_string(), !self.engine.model_ckpt);

                if !latent_blank
                {
                    if false
                    {
                        json = json.replace("\"class_type\": \"VAEEncode\"", "\"class_type\": \"VAEEncodeTiled\"");
                        json = GenerateStage::json_templ(json, "##TILE##".to_string(), "\"tile_size\": 512,\"overlap\": 64,\"temporal_size\": 64,\"temporal_overlap\": 8,".to_string());
                    }
                    else
                    {
                        json = GenerateStage::json_templ(json, "##TILE##".to_string(), "".to_string());
                    }
                }
                json = GenerateStage::json_tag(json, "##ModelNameDiff##".to_string(), self.engine.model_diffusion.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameText##".to_string(), self.engine.model_text.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameVae##".to_string(), self.engine.model_vae.to_string());

                if latent_blank
                {
                    json = GenerateStage::json_repeat(json, self.batch_s as i32, &self);
                    json = GenerateStage::json_tag_i(json, "##imgW##".to_string(), latent_width);
                    json = GenerateStage::json_tag_i(json, "##imgH##".to_string(), latent_height);

                }
                else
                {
                    json = GenerateStage::json_load_file_batch(json, elem.param1s.to_string(), self.batch_s as i32, false);
                    json = GenerateStage::json_save_file_batch(json, elem.param1s.to_string(), self.batch_s as i32, true, 0); //// ..true, (latentType == 2) ? 1 : 0);
                }

                json = GenerateStage::json_clean(json);

                self.js_exec_comfyui_json(&json);
                true
            },
            "bitmap_to_latent_end" => {
                self.execute_finish_latent(&callback_data);
                //elem = &self.execute_elements[elem_idx];

                js_exec("");
                true
            },
            "latent_download_begin" => {
                self.js_exec_comfyui_web(&elem.param1s, "X_BINARYO!1!", "");
                true
            },
            "latent_download_end" => {
                match elem.param2i
                {
                    0 => { self.image_latent_i[elem.param1i as usize] = callback_data.to_string(); },
                    1 => { self.image_latent_o[elem.param1i as usize] = callback_data.to_string(); },
                    _ => {},
                }
                js_exec("");
                true
            },
            "bitmap_download_begin" => {
                self.js_exec_comfyui_web(&elem.param1s, "X_BINARYO!1!", "");
                true
            },
            "bitmap_download_end" => {
                let bitmap_mode = elem.param2i % 10;
                let bitmap_mask = (elem.param2i >= 10) && (self.work_mask_file_3.len() > 2);
                match bitmap_mode
                {
                    0 => { self.image_bitmap_o[elem.param1i as usize] = if bitmap_mask
                        {
                            GenerateStage::graph_mask_3(callback_data.to_string(), &self.work_mask_file_3, &self.image_bitmap_i[elem.param1i as usize])
                        }
                        else
                        {
                            if elem.param1i == 0
                            {
                                let img_temp = GenerateStage::graph_file_to_imag(&callback_data);
                                self.image_w_o = img_temp.width() as i32;
                                self.image_h_o = img_temp.height() as i32;
                            }
                            callback_data.to_string()
                        };
                    },
                    1 => { self.image_bitmap_i[elem.param1i as usize] = if bitmap_mask
                        {
                            GenerateStage::graph_mask_3(callback_data.to_string(), &self.work_mask_file_3, &self.image_bitmap_i[elem.param1i as usize])
                        }
                        else
                        {
                            if elem.param1i == 0
                            {
                                let img_temp = GenerateStage::graph_file_to_imag(&callback_data);
                                self.image_w_o = img_temp.width() as i32;
                                self.image_h_o = img_temp.height() as i32;
                            }
                            callback_data.to_string()
                        };
                    },
                    _ => {},
                }
                js_exec("");
                true
            },
            "latent_to_bitmap_begin" => {
                let mut json = include_str!("comfy_convLatentToBitmap.json").to_string();


                json = GenerateStage::json_insert(json, "Memory".to_string(), "comfy_memory.json".to_string(), "Memo".to_string());
                json = GenerateStage::json_variant(json, "CheckpointMemo".to_string(), false);
                json = GenerateStage::json_variant(json, "SplitModelMemo".to_string(), !self.engine.model_ckpt);
                json = GenerateStage::json_variant(json, "Checkpoint".to_string(), self.engine.model_ckpt);
                json = GenerateStage::json_variant(json, "SplitModel".to_string(), !self.engine.model_ckpt);

                if false
                {
                    json = json.replace("\"class_type\": \"VAEDecode\"", "\"class_type\": \"VAEDecodeTiled\"");
                    json = GenerateStage::json_templ(json, "##TILE##".to_string(), "\"tile_size\": 512,\"overlap\": 64,\"temporal_size\": 64,\"temporal_overlap\": 8,".to_string());
                }
                else
                {
                    json = GenerateStage::json_templ(json, "##TILE##".to_string(), "".to_string());
                }
                json = GenerateStage::json_tag(json, "##ModelNameDiff##".to_string(), self.engine.model_diffusion.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameText##".to_string(), self.engine.model_text.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameVae##".to_string(), self.engine.model_vae.to_string());
                json = GenerateStage::json_load_file_batch(json, elem.param1s.to_string(), self.batch_s as i32, true);
                json = GenerateStage::json_save_file_batch(json, elem.param1s.to_string(), self.batch_s as i32, false, 0); //// ..true, (latentType == 2) ? 1 : 0);
                json = GenerateStage::json_clean(json);

                self.js_exec_comfyui_json(&json);
                true
            },
            "latent_to_bitmap_end" => {
                self.execute_finish_bitmap(&callback_data);
                //elem = &self.execute_elements[elem_idx];

                js_exec("");
                true
            },
            "paint_begin" => {
                self.work_file_name_work = elem.param1s.to_string();

                let mut json = if self.work_mask_file_2.len() > 2
                {
                    include_str!("comfy_paintMask.json").to_string()
                }
                else
                {
                    include_str!("comfy_paint.json").to_string() 
                }; 

                json = GenerateStage::json_insert(json, "Memory".to_string(), "comfy_memory.json".to_string(), "Memo".to_string());
                json = GenerateStage::json_variant(json, "CheckpointMemo".to_string(), false);

                json = GenerateStage::json_variant(json, "SplitModelMemo".to_string(), false);

                //json = GenerateStage::json_variant(json, "SplitModelMemo".to_string(), !self.engine.model_ckpt);
                json = GenerateStage::json_variant(json, "Checkpoint".to_string(), self.engine.model_ckpt);
                json = GenerateStage::json_variant(json, "SplitModel".to_string(), !self.engine.model_ckpt);

                json = GenerateStage::json_tag(json, "##ModelNameDiff##".to_string(), self.engine.model_diffusion.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameText##".to_string(), self.engine.model_text.to_string());
                json = GenerateStage::json_tag(json, "##ModelNameVae##".to_string(), self.engine.model_vae.to_string());

                json = GenerateStage::json_tag(json, "##PromptPosi##".to_string(), self.undo_redo_curr.prompt_posi.to_string());
                json = GenerateStage::json_tag(json, "##PromptNega##".to_string(), self.undo_redo_curr.prompt_nega.to_string());
                json = GenerateStage::json_tag(json, "##sampler##".to_string(), GenerateStageUndoRedo::sampler_scheduler_by_name(11, &self.undo_redo_curr.sampler));
                json = GenerateStage::json_tag(json, "##scheduler##".to_string(), GenerateStageUndoRedo::sampler_scheduler_by_name(21, &self.undo_redo_curr.scheduler));

                json = GenerateStage::json_repeat(json, self.batch_s as i32, &self);
                json = GenerateStage::json_clean(json);

                self.js_exec_comfyui_json(&json);
                true
            },
            "paint_end" => {
                self.execute_finish_latent(&callback_data);
                //elem = &self.execute_elements[elem_idx];


                js_exec("");
                true
            },
            _ => false,
        }




    }

}