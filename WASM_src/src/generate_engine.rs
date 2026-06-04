use crate::config_file::ConfigFile;

pub struct GenerateEngine
{
    pub name: String,
    pub model: String,
    pub model_text: String,
    pub model_diffusion: String,
    pub model_vae: String,
    pub model_ckpt: bool,
    pub steps: i32,
    pub cfg: i32,
    pub sampler: String,
    pub scheduler: String,
    pub img_type: i32,
}

impl GenerateEngine
{
    pub fn new_blank() -> GenerateEngine
    {
        let self_ = GenerateEngine
        {
            name: String::new(),
            model: String::new(),
            model_text: String::new(),
            model_diffusion: String::new(),
            model_vae: String::new(),
            steps: 0,
            cfg: 0,
            sampler: String::new(),
            scheduler: String::new(),
            img_type: 0,
            model_ckpt: true,
        };
        self_
    }

    pub fn new_clone(eng: &GenerateEngine) -> GenerateEngine
    {
        let self_ = GenerateEngine
        {
            name: eng.name.to_string(),
            model: eng.model.to_string(),
            model_text: eng.model_text.to_string(),
            model_diffusion: eng.model_diffusion.to_string(),
            model_vae: eng.model_vae.to_string(),
            steps: eng.steps,
            cfg: eng.cfg,
            sampler: eng.sampler.to_string(),
            scheduler: eng.scheduler.to_string(),
            img_type: eng.img_type,
            model_ckpt: eng.model_ckpt,
        };
        self_
    }

    pub fn new(cf: &ConfigFile, idx: i32) -> GenerateEngine
    {
        let mut self_ = GenerateEngine
        {
            name: cf.param_get_s(format!("Engine{}Name", idx)),
            model: cf.param_get_s(format!("Engine{}Model", idx)),
            model_text: cf.param_get_s(format!("Engine{}ModelText", idx)),
            model_diffusion: cf.param_get_s(format!("Engine{}ModelDiffusion", idx)),
            model_vae: cf.param_get_s(format!("Engine{}ModelVae", idx)),
            steps: cf.param_get_i(format!("Engine{}Steps", idx)),
            cfg: cf.param_get_i(format!("Engine{}Cfg", idx)),
            sampler: cf.param_get_s(format!("Engine{}Sampler", idx)),
            scheduler: cf.param_get_s(format!("Engine{}Scheduler", idx)),
            img_type: cf.param_get_i(format!("Engine{}Type", idx)),
            model_ckpt: true,
        };
        if (self_.model_text.len() > 0) || (self_.model_diffusion.len() > 0) || (self_.model_vae.len() > 0)
        {
            self_.model_ckpt = false;
        }
        if self_.model_text.len() == 0
        {
            self_.model_text = self_.model.to_string();
        }
        if self_.model_diffusion.len() == 0
        {
            self_.model_diffusion = self_.model.to_string();
        }
        if self_.model_vae.len() == 0
        {
            self_.model_vae = self_.model.to_string();
        }
        self_
    }
}