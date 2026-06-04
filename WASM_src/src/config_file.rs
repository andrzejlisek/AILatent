use std::collections::HashMap;

use crate::tools;

pub struct ConfigFile
{
    pub raw: HashMap<String, String>,
    pub raw_file: Vec<String>,
    pub case_mode: i32,
    pub save_state: i32,
}

impl ConfigFile
{
    pub fn case_cm(s: String, cm: i32) -> String
    {
        if cm > 0
        {
            return s.to_uppercase();
        }
        if cm < 0
        {
            return s.to_lowercase();
        }
        return s;
    }

    fn case(&self, s: String) -> String
    {
        if self.case_mode > 0
        {
            return s.to_uppercase();
        }
        if self.case_mode < 0
        {
            return s.to_lowercase();
        }
        return s;
    }

    pub fn new() -> ConfigFile
    {
        let self_ = ConfigFile
        {
            raw: HashMap::new(),
            raw_file: vec![],
            case_mode: 0,
            save_state: 0,
        };
        self_
    }

    pub fn file_load_start(&mut self)
    {
        self.raw.clear();
        self.raw_file.clear();
    }

    pub fn file_load_s(&mut self, raw_line: &str)
    {
        self.file_load(format!("{}", raw_line));
    }

    pub fn file_load(&mut self, raw_line: String)
    {
        let i = tools::string_index_of(&raw_line, '=');
        if i >= 0
        {
            let i_usize = i as usize;
            let k = self.case((&raw_line[0..i_usize]).to_string());
            if !self.raw.contains_key(&k)
            {
                let val = if raw_line.len() > i as usize + 1
                {
                    raw_line[i_usize + 1..].to_string()
                }
                else
                {
                    String::new()
                };
                self.raw.insert((&k).to_string(), val);
                self.raw_file.push(format!("{}=", k));
            }
        }
        else
        {
            self.raw_file.push(raw_line);
        }
    }

    pub fn file_save_start(&mut self)
    {
        self.save_state = 0;
    }

    pub fn file_save(&mut self) -> String
    {
        self.save_state = self.save_state + 1;
        let mut i = 0;
        let cm = self.case_mode;
        for (key, val) in self.raw.iter()
        {
            i = i + 1;
            if i == self.save_state
            {
                return (format!("{}={}", &ConfigFile::case_cm(key.to_string(), cm), val)).to_string();
            }
        }
        String::new()
    }


    pub fn print(&self) -> String
    {
        let mut s = String::new();
        let cm = self.case_mode;
        for (key, val) in self.raw.iter()
        {
            s.push_str(&ConfigFile::case_cm(key.to_string(), cm));
            s.push('=');
            s.push_str(val);
            s.push_str("\n");
        }
        s
    }

    pub fn param_clear(&mut self)
    {
        self.raw.clear();
        self.raw_file.clear();
    }

    pub fn param_remove(&mut self, name: String)
    {
        let k = self.case(name);
        if let std::collections::hash_map::Entry::Occupied(entry) = self.raw.entry(k)
        {
            entry.remove();
        }
    }

    pub fn param_set_s(&mut self, name: String, value: String)
    {
        let k = self.case(name);
        if self.raw.contains_key(&k)
        {
            self.raw.insert((&k).to_string(), value);
        }
        else
        {
            self.raw.insert((&k).to_string(), value);
            self.raw_file.push(format!("{}=", k));
        }
    }

    pub fn param_set_i(&mut self, name: String, value: i32)
    {
        self.param_set_s(name, format!("{}", value));
    }

    pub fn param_set_l(&mut self, name: String, value: i64)
    {
        self.param_set_s(name, format!("{}", value));
    }

    pub fn param_set_b(&mut self, name: String, value: bool)
    {
        if value
        {
            self.param_set_s(name, format!("1"));
        }
        else
        {
            self.param_set_s(name, format!("0"));
        }
    }

    pub fn param_get_s_x(&self, name: String, x: String) -> String
    {
        let k = self.case(name);
        self.raw.get(&k).cloned().unwrap_or(x)
    }

    pub fn param_get_s(&self, name: String) -> String
    {
        let k = self.case(name);
        self.raw.get(&k).cloned().unwrap_or("".to_string())
    }

    pub fn param_get_i_x(&self, name: String, x: i32) -> i32
    {
        let k = self.case(name);
        let v = self.raw.get(&k).cloned().unwrap_or("0".to_string());
        v.parse::<i32>().unwrap_or_else(|_| x)
    }

    pub fn param_get_i(&self, name: String) -> i32
    {
        let k = self.case(name);
        let v = self.raw.get(&k).cloned().unwrap_or("0".to_string());
        v.parse::<i32>().unwrap_or_else(|_| 0)
    }

    pub fn param_get_l_x(&self, name: String, x: i64) -> i64
    {
        let k = self.case(name);
        let v = self.raw.get(&k).cloned().unwrap_or("0".to_string());
        v.parse::<i64>().unwrap_or_else(|_| x)
    }

    pub fn param_get_l(&self, name: String) -> i64
    {
        let k = self.case(name);
        let v = self.raw.get(&k).cloned().unwrap_or("0".to_string());
        v.parse::<i64>().unwrap_or_else(|_| 0)
    }

    pub fn param_get_b_x(&self, name: String, x: bool) -> bool
    {
        let k = self.case(name);
        let v = self.raw.get(&k).cloned().unwrap_or("X".to_string());
        match v.to_uppercase().as_str()
        {
            "1" | "TRUE" | "YES" | "T" | "Y" => true,
            "0" | "FALSE" | "NO" | "F" | "N" => false,
            _ => x,
        }
    }

    pub fn param_get_b(&self, name: String) -> bool
    {
        let k = self.case(name);
        let v = self.raw.get(&k).cloned().unwrap_or("X".to_string());
        match v.to_uppercase().as_str()
        {
            "1" | "TRUE" | "YES" | "T" | "Y" => true,
            "0" | "FALSE" | "NO" | "F" | "N" => false,
            _ => false,
        }
    }

    pub fn param_exists(&self, name: String) -> bool
    {
        let k = self.case(name);
        self.raw.contains_key(&k)
    }
}