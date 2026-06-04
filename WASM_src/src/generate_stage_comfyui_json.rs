use crate::generate_stage::GenerateStage;
use crate::tools;
use crate::tools::*;

impl GenerateStage
{
    pub fn json_templ(json:String, tag:String, value:String) -> String
    {
        json.replace(&format!("\"{}\":\"\",\n", tag), &value)
    }

    pub fn json_tag(json:String, tag:String, value:String) -> String
    {
        let mut value_x = value;
        value_x = value_x.replace("\\", "\\\\");
        value_x = value_x.replace("\"", "\\\"");
        value_x = value_x.replace("\r", "\\r");
        value_x = value_x.replace("\n", "\\n");
        value_x = value_x.replace("\t", "\\t");
        json.replace(&format!("\"{}\"", tag), &format!("\"{}\"", tools::json_text_to_ascii(&value_x)))
    }

    pub fn json_tag_(json:String, tag:String, value:String) -> String
    {
        json.replace(&format!("\"{}\"", tag), &value)
    }

    pub fn json_tag_i(json:String, tag:String, value:i32) -> String
    {
        json.replace(&format!("\"{}\"", tag), &format!("{}", value))
    }

    pub fn json_tag_b(json:String, tag:String, value:bool) -> String
    {
        if value
        {
            json.replace(&format!("\"{}\"", tag), "true")
        }
        else
        {
            json.replace(&format!("\"{}\"", tag), "false")
        }
    }



    pub fn json_load_file_batch(json: String, file_name: String, batch_len: i32, latent: bool) -> String
    {
        let id_start = 2000;
        let mut sb = String::new();
        for i in 0..(batch_len - 1)
        {
            let id = id_start + i;
            let id1 = id + 1;
            let id2 = id_start + ((batch_len - 1) * 2) - i;
            sb.push_str(&format!("{}{}{}", "  \"", id, "\": {\n"));
            sb.push_str("    \"inputs\": {\n");
            if latent
            {
                sb.push_str("      \"samples1\": [\n");
            }
            else
            {
                sb.push_str("      \"image1\": [\n");
            }
            sb.push_str(&format!("{}{}{}", "        \"", id1, "\",\n"));
            sb.push_str("        0\n");
            sb.push_str("      ],\n");
            if latent
            {
                sb.push_str("      \"samples2\": [\n");
            }
            else
            {
                sb.push_str("      \"image2\": [\n");
            }
            sb.push_str(&format!("{}{}{}", "        \"", id2, "\",\n"));
            sb.push_str("        0\n");
            sb.push_str("      ]\n");
            sb.push_str("    },\n");
            if latent
            {
                sb.push_str("    \"class_type\": \"LatentBatch\",\n");
            }
            else
            {
                sb.push_str("    \"class_type\": \"ImageBatch\",\n");
            }
            sb.push_str("    \"_meta\": {\n");
            if latent
            {
                sb.push_str("      \"title\": \"LatentBatch\"\n");
            }
            else
            {
                sb.push_str("      \"title\": \"Batch Images\"\n");
            }
            sb.push_str("    }\n");
            sb.push_str("  },\n");
        }
        for i in 0..batch_len
        {
            let id = id_start - 1 + batch_len + i;
            sb.push_str(&format!("{}{}{}", "  \"", id, "\": {\n"));
            sb.push_str("    \"inputs\": {\n");
            if latent
            {
                sb.push_str(&format!("{}{}{}{}", "      \"latent\": \"", file_name, i, ".latent\"\n"));
            }
            else
            {
                sb.push_str(&format!("{}{}{}{}", "      \"image\": \"", file_name, i, ".png\"\n"));
            }
            sb.push_str("    },\n");
            if latent
            {
                sb.push_str("    \"class_type\": \"LoadLatent\",\n");
            }
            else
            {
                sb.push_str("    \"class_type\": \"LoadImage\",\n");
            }
            sb.push_str("    \"_meta\": {\n");
            if latent
            {
                sb.push_str("      \"title\": \"LoadLatent\"\n");
            }
            else
            {
                sb.push_str("      \"title\": \"Load Image\"\n");
            }
            sb.push_str("    }\n");
            sb.push_str("  },\n");
        }
        if latent
        {
            return GenerateStage::json_templ(json, "##LOAD_LATENT##".to_string(), sb);
        }
        else
        {
            return GenerateStage::json_templ(json, "##LOAD_BITMAP##".to_string(), sb);
        }
    }

    pub fn json_save_file_batch(json: String, file_name: String, batch_len: i32, latent: bool, node_conn: i32) -> String
    {
        let mut sb = String::new();
        for i in 0..batch_len
        {
            let id1 = 1000 + (i * 10) + 1;
            let id2 = 1000 + (i * 10);
            sb.push_str(&format!("{}{}{}", "  \"", id1, "\": {\n"));
            sb.push_str("    \"inputs\": {\n");
            sb.push_str(&format!("{}{}{}", "      \"batch_index\": ", i, ",\n"));
            sb.push_str("      \"length\": 1,\n");
            if latent
            {
                sb.push_str("      \"samples\": [\n");
            }
            else
            {
                sb.push_str("      \"image\": [\n");
            }
            sb.push_str("        \"999\",\n");
            sb.push_str(&format!("{}{}{}", "        ", node_conn, "\n"));
            sb.push_str("      ]\n");
            sb.push_str("    },\n");
            if latent
            {
                sb.push_str("    \"class_type\": \"LatentFromBatch\",\n");
            }
            else
            {
                sb.push_str("    \"class_type\": \"ImageFromBatch\",\n");
            }
            sb.push_str("    \"_meta\": {\n");
            if latent
            {
                sb.push_str("      \"title\": \"Latent From Batch\"\n");
            }
            else
            {
                sb.push_str("      \"title\": \"ImageFromBatch\"\n");
            }
            sb.push_str("    }\n");
            sb.push_str("  },\n");
            sb.push_str(&format!("{}{}{}", "  \"", id2, "\": {\n"));
            sb.push_str("    \"inputs\": {\n");
            sb.push_str(&format!("{}{}{}{}", "      \"filename_prefix\": \"", file_name, i, "\",\n"));
            if latent
            {
                sb.push_str("      \"samples\": [\n");
            }
            else
            {
                sb.push_str("      \"images\": [\n");
            }
            sb.push_str(&format!("{}{}{}", "        \"", id1, "\",\n"));
            sb.push_str("        0\n");
            sb.push_str("      ]\n");
            sb.push_str("    },\n");
            if latent
            {
                sb.push_str("    \"class_type\": \"SaveLatent\",\n");
            }
            else
            {
                sb.push_str("    \"class_type\": \"SaveImage\",\n");
            }
            sb.push_str("    \"_meta\": {\n");
            if latent
            {
                sb.push_str("      \"title\": \"SaveLatent\"\n");
            }
            else
            {
                sb.push_str("      \"title\": \"Save Image\"\n");
            }
            sb.push_str("    }\n");
            sb.push_str("  },\n");
        }
        if latent
        {
            return GenerateStage::json_templ(json, "##SAVE_LATENT##".to_string(), sb);
        }
        else
        {
            return GenerateStage::json_templ(json, "##SAVE_BITMAP##".to_string(), sb);
        }
    }

    pub fn json_repeat(json: String, batch_len: i32, current_stage: &GenerateStage) -> String
    {
        let repeat_tag_1 = "\"##REPEAT_BEGIN##\":\"\",";
        let repeat_tag_2 = "\"##REPEAT_END##\":\"\",";
        let repeat_1: isize = string_index_of_s(&json, &repeat_tag_1);
        let repeat_2: isize = string_index_of_s(&json, &repeat_tag_2);

        let json1 = substring(&json, 0, repeat_1 as usize);
        let json2 = substring_(&json, repeat_2 as usize + repeat_tag_2.len());
        let jsonx0 = substring(&json, repeat_1 as usize + repeat_tag_1.len(), repeat_2 as usize);

        let mut sb = String::new();
        sb.push_str(&json1);
        for i in 0..batch_len
        {
            let mut jsonx = jsonx0.clone();
            jsonx = GenerateStage::json_tag(jsonx, "##idx0##".to_string(), format!("{}", ((i * 10 + 0) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx1##".to_string(), format!("{}", ((i * 10 + 1) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx2##".to_string(), format!("{}", ((i * 10 + 2) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx3##".to_string(), format!("{}", ((i * 10 + 3) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx4##".to_string(), format!("{}", ((i * 10 + 4) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx5##".to_string(), format!("{}", ((i * 10 + 5) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx6##".to_string(), format!("{}", ((i * 10 + 6) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx7##".to_string(), format!("{}", ((i * 10 + 7) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx8##".to_string(), format!("{}", ((i * 10 + 8) + 1000)));
            jsonx = GenerateStage::json_tag(jsonx, "##idx9##".to_string(), format!("{}", ((i * 10 + 9) + 1000)));

            jsonx = GenerateStage::json_tag_(jsonx, "##Seed##".to_string(), format!("{}", tools::int_to_unsigned_long(current_stage.get_batch_seed(i))));

            jsonx = GenerateStage::json_tag(jsonx, "##AddNoise##".to_string(), if current_stage.undo_redo_curr.process_begin { "enable" } else { "disable" }.to_string());
            jsonx = GenerateStage::json_tag(jsonx, "##ReturnWithNoise##".to_string(), if current_stage.undo_redo_curr.process_begin { "disable" } else { "enable" }.to_string());

            if current_stage.get_batch_steps_total(i) > 0
            {
                jsonx = GenerateStage::json_tag_i(jsonx, "##StepStart##".to_string(), current_stage.get_batch_steps_begin(i));
                jsonx = GenerateStage::json_tag_i(jsonx, "##StepEnd##".to_string(), current_stage.get_batch_steps_end(i));
                jsonx = GenerateStage::json_tag_i(jsonx, "##StepCount##".to_string(), current_stage.get_batch_steps_total(i));
            }
            else
            {
                jsonx = GenerateStage::json_tag_i(jsonx, "##StepStart##".to_string(), 0);
                jsonx = GenerateStage::json_tag_i(jsonx, "##StepEnd##".to_string(), 0);
                jsonx = GenerateStage::json_tag_i(jsonx, "##StepCount##".to_string(), 0);
            }
            jsonx = GenerateStage::json_tag_(jsonx, "##PromptCfg##".to_string(), tools::int_to_str(current_stage.get_batch_cfg(i), 1));

            jsonx = GenerateStage::json_tag(jsonx, "##FileNameI##".to_string(), format!("{}{}.latent", current_stage.temp_file_name_work, i));
            jsonx = GenerateStage::json_tag(jsonx, "##FileNameP##".to_string(), format!("{}{}x.latent", current_stage.temp_file_name_work, i));
            jsonx = GenerateStage::json_tag(jsonx, "##FileNameM##".to_string(), format!("{}{}x.png", current_stage.temp_file_name_work, i));
            jsonx = GenerateStage::json_tag(jsonx, "##FileNameO##".to_string(), format!("{}{}", current_stage.temp_file_name_work, i));
            sb.push_str(&jsonx);
        }
        sb.push_str(&json2);
        sb
    }

    pub fn json_variant(json: String, tag_name: String, use_x: bool) -> String
    {
        let option_tag_1 = format!("\"##OPTION_{}_BEGIN##\":\"\",", tag_name);
        let option_tag_2 = format!("\"##OPTION_{}_END##\":\"\",", tag_name);
        let mut option_1: isize = string_index_of_s(&json, &option_tag_1);
        let mut option_2: isize = string_index_of_s(&json, &option_tag_2);

        let mut json_x = json;
        while (option_1 > 0) && (option_2 > option_1)
        {
            let json1 = substring(&json_x, 0, option_1 as usize);
            let json2 = substring_(&json_x, option_2 as usize + option_tag_2.len());
            let jsonx = substring(&json_x, option_1 as usize + option_tag_1.len(), option_2 as usize);
            if use_x
            {
                json_x = format!("{}{}{}", json1, jsonx, json2);
            }
            else
            {
                json_x = format!("{}{}", json1, json2);
            }

            option_1 = string_index_of_s(&json_x, &option_tag_1);
            option_2 = string_index_of_s(&json_x, &option_tag_2);
        }
        json_x
    }

    pub fn json_excerpt(json: String, tag_name: String) -> String
    {
        let option_tag_1 = format!("\"##OPTION_{}_BEGIN##\":\"\",", tag_name);
        let option_tag_2 = format!("\"##OPTION_{}_END##\":\"\",", tag_name);
        let option_1: isize = string_index_of_s(&json, &option_tag_1);
        let option_2: isize = string_index_of_s(&json, &option_tag_2);

        let json_x = json;
        if (option_1 > 0) && (option_2 > option_1)
        {
            let jsonx = substring(&json_x, option_1 as usize + option_tag_1.len(), option_2 as usize);

            return jsonx;
        }
        json_x
    }

    pub fn json_insert(json: String, tag_name: String, contents_x: String, contents_tag: String) -> String
    {
        let mut contents = String::new();
        if contents_x.len() > 0
        {
            contents = get_resource(&contents_x).to_string();
            contents = GenerateStage::json_excerpt(contents, contents_tag);
        }

        let insert_tag = format!("\"##INSERT_{}##\":\"\",", tag_name);
        let mut insert0 = string_index_of_s(&json, &insert_tag);
        let mut json_x = json;

        while insert0 > 0
        {
            let json1 = substring(&json_x, 0, insert0 as usize);
            let json2 = substring_(&json_x, insert0 as usize + insert_tag.len());

            json_x = format!("{}{}{}", json1, contents, json2);

            insert0 = string_index_of_s(&json_x, &insert_tag);
        }

        json_x
    }

    /*pub fn json_clean(json: String) -> String
    {
        let dummy_tag = "\"dummy\":\"dummy\"";
        let mut json_x = json;
        let mut dummy_pos = string_index_of_s(&json_x, dummy_tag);
        while dummy_pos > 0
        {
            let mut json1 = format!("!{}", substring(&json_x, 0, dummy_pos as usize));
            json1 = substring_(json1.trim(), 1);
            if json1.ends_with(",")
            {
                json1 = substring(&json1, 0, json1.chars().count() - 1);
            }
            let json2 = substring_(&json_x, dummy_pos as usize + dummy_tag.len());
            json_x = format!("{}{}", json1, json2);
            dummy_pos = string_index_of_s(&json_x, dummy_tag);
        }
        json_x
    }*/

    pub fn json_clean(json: String) -> String {
        let dummy_tag = "\"dummy\":\"dummy\"";
        let mut json_x = json;

        while let Some(dummy_pos) = json_x.find(dummy_tag)
        {
            let mut left_side = json_x[..dummy_pos].to_string();
            left_side = left_side.trim_end().to_string();

            if left_side.ends_with(',')
            {
                left_side.pop();
            }

            let right_side = &json_x[dummy_pos + dummy_tag.len()..];

            json_x = format!("{}{}", left_side, right_side);
        }

        json_x
    }
}