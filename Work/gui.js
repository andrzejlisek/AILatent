let guiBatchW = 1;
let guiBatchH = 1;



function guiExecStart()
{
    document.getElementById("waiting").style.display = "";
}

function guiExecStop()
{
    document.getElementById("waiting").style.display = "none";
}

function guiExec(idx, eType)
{
    switch(eType)
    {
        case 0:
            setTimeout(projectStageShowPics, 1);
            guiExecStop();
            break;
        case 1:
            guiExecStart();
            break;
    }
    setTimeout(guiStageRefreshInfo, 1, idx);
}

function guiDispPic(stage, batchN)
{
    let contents = window.asm_env.data("get_picture", stage, "" + batchN, "1", "");
    screenSetPicture(batchN, graphPrefix(contents));
}




function guiNumber(e)
{
        e.preventDefault();

        let value = parseInt(this.value, 10) || 0;
        let step = parseInt(this.step, 10) || 1;
        let min = parseInt(this.min, 10);
        let max = parseInt(this.max, 10);

        if (e.deltaY < 0)
        {
            value += step;
        }
        else
        {
            value -= step;
        }

        if (!isNaN(min) && value < min) value = min;
        if (!isNaN(max) && value > max) value = max;

        this.value = value;

        this.dispatchEvent(new Event('input'));
}

function guiInputValue(e)
{
    const id = e.target.id;
    //console.log(e.target.id + " = " + e.target.value);
    
    let isStageChange = 0;
    
    if ((id == "stage_input_row") || (id == "stage_input_col") || (id == "stage_process_source")) { isStageChange = 1; }
    if ((id == "stage_input_file") || (id == "stage_input_width") || (id == "stage_input_height")) { isStageChange = 1; }
    if ((id == "stage_input_zoom") || (id == "stage_input_offsetx") || (id == "stage_input_offsety")) { isStageChange = 1; }

    if ((id == "stage_process_model") || (id == "stage_process_sampler") || (id == "stage_process_scheduler")) { isStageChange = 2; }
    if ((id == "stage_process_seed_v") || (id == "stage_process_seed_i")) { isStageChange = 1; }
    if ((id == "stage_process_cfg_v") || (id == "stage_process_cfg_i")) { isStageChange = 1; }
    if ((id == "stage_process_step_t_v") || (id == "stage_process_step_t_i")) { isStageChange = 1; }
    if ((id == "stage_process_step_b_v") || (id == "stage_process_step_b_i")) { isStageChange = 1; }
    if ((id == "stage_process_step_e_v") || (id == "stage_process_step_e_i")) { isStageChange = 1; }

    if ((id == "stage_stage_type") || (id == "stage_process_source_type")) { isStageChange = 3; }
    if ((id == "stage_process_type") || (id == "stage_process_seed_d") || (id == "stage_process_cfg_d")) { isStageChange = 3; }
    if ((id == "stage_process_step_t_d") || (id == "stage_process_step_b_d") || (id == "stage_process_step_e_d")) { isStageChange = 3; }


    if ((id == "stage_process_prompt_posi") || (id == "stage_process_prompt_nega")) { isStageChange = 1; }
    
    if (isStageChange > 0)
    {
        projectStageSet(id, isStageChange);
        guiStageRefresh();
    }
    
    if (id == "stages")
    {
        projectStageShow();
    }
}


function guiValueEngine(n)
{
    if (n < 0)
    {
        return "-" + guiValueEngine(0 - n);
    }

    if ((n >= 1000) && (n < 10000)) return (n / 1000).toFixed(2) + "k";
    if ((n >= 10000) && (n < 100000)) return (n / 1000).toFixed(1) + "k";
    if ((n >= 100000) && (n < 1000000)) return (n / 1000).toFixed(0) + "k";

    if ((n >= 1000000) && (n < 10000000)) return (n / 1000000).toFixed(2) + "M";
    if ((n >= 10000000) && (n < 100000000)) return (n / 1000000).toFixed(1) + "M";
    if ((n >= 100000000) && (n < 1000000000)) return (n / 1000000).toFixed(0) + "M";

    if ((n >= 1000000000)) return (n / 10000000).toFixed(2) + "G";
    if ((n >= 10000000000)) return (n / 100000000).toFixed(1) + "G";
    if ((n >= 100000000000)) return (n / 1000000000).toFixed(0) + "G";

    return "" + n;
}

function guiListClear(list)
{
    list.options.length = 0;
}

function guiListAdd(list, txt)
{
    let item = document.createElement("option");
    item.value = txt;
    item.textContent = txt;
    list.appendChild(item);
}

function guiListRem(list, idx)
{
    let item = list.remove(idx);
}

function guiListUpdate(list, idx, txt)
{
    let item = list.children[idx];
    item.value = txt;
    item.textContent = txt;
}

function guiListSetByText(list, txt)
{
    for (let i = 0; i < list.options.length; i++)
    {
        if (list.options[i].value == txt)
        {
            list.selectedIndex = i;
            return;
        }
    }
    list.selectedIndex = -1;
}


function guiStageRefreshInfo(idx)
{
    const stageList = document.getElementById("stages");
    const stage = (idx >= 0) ? idx : stageList.selectedIndex;
    if (stage >= 0)
    {
        guiListUpdate(stageList, stage, window.asm_env.data("project_stage", stage, "", "", ""));
    }
    return stage;
}

function guiStageRefresh()
{
    const stage = guiStageRefreshInfo(-1);

    const idx = document.getElementById("stage_stage_type").selectedIndex;
    document.getElementById("stage_type_0").style.display = (idx == 0) ? "" : "none";
    document.getElementById("stage_type_1").style.display = (idx == 1) ? "" : "none";
    document.getElementById("stage_type_2").style.display = (idx == 2) ? "" : "none";

    switch (idx)
    {
        case 0:
            {
                const w = parseInt(document.getElementById("stage_input_width").value);
                const h = parseInt(document.getElementById("stage_input_height").value);
                if (h > 0)
                {
                    document.getElementById("stage_info_aspect").innerHTML = (w / h).toFixed(3);
                }
                else
                {
                    document.getElementById("stage_info_aspect").innerHTML = "0.000";
                }
                document.getElementById("stage_info_area").innerHTML = guiValueEngine(w * h);
            }
            break;
        case 1:
            {
                document.getElementById("stage_process_recom").innerHTML = window.asm_env.data("project_value_get", stage, "process_recom", "", "");
                document.getElementById("stage_process_steps").innerHTML = window.asm_env.data("project_value_get", stage, "process_steps", "", "");
                document.getElementById("stage_process_denoise").innerHTML = window.asm_env.data("project_value_get", stage, "process_denoise", "", "");
            }
            break;
    }
}



function guiControlPrepareTxt(id)
{
    document.getElementById(id).style.width = "100%";
    document.getElementById(id).style["min-width"] = "50px";
    document.getElementById(id).addEventListener('input', guiInputValue, { passive: false });
}

function guiControlPrepareNum(id)
{
    document.getElementById(id).style.width = "100%";
    document.getElementById(id).style["min-width"] = "50px";
    document.getElementById(id).addEventListener('wheel', guiNumber, { passive: false });
    document.getElementById(id).addEventListener('input', guiInputValue, { passive: false });
}

function guiControlPrepareOpt(id, opts)
{
    const list = document.getElementById(id);
    list.options.length = 0;
    for (i in opts)
    {
        let item = document.createElement("option");
        if (opts[i] == filesWork)
        {
            idx = i;
        }
        item.value = opts[i];
        item.textContent = opts[i];
        list.appendChild(item);
    }
    list.selectedIndex = 0;
    list.style.width = "100%";
    list.style["min-width"] = "50px";
    document.getElementById(id).addEventListener('input', guiInputValue, { passive: false });
}


guiControlPrepareNum("projectBatchW");
guiControlPrepareNum("projectBatchH");

guiControlPrepareTxt("stage_input_file");
guiControlPrepareNum("stage_input_width", ["xxx"]);
guiControlPrepareNum("stage_input_height", ["xxx"]);
guiControlPrepareNum("stage_input_row");
guiControlPrepareNum("stage_input_col");
guiControlPrepareNum("stage_input_zoom");
guiControlPrepareNum("stage_input_offsetx");
guiControlPrepareNum("stage_input_offsety");

guiControlPrepareNum("stage_process_source");
guiControlPrepareOpt("stage_process_source_type", ["Bitmap","Latent"]);
guiControlPrepareOpt("stage_process_type", ["Full","Begin","Middle","End"]);
guiControlPrepareOpt("stage_process_model", ["xxx"]);
guiControlPrepareOpt("stage_process_sampler", ["Euler", "DPM++ 2M", "LCM"]);
guiControlPrepareOpt("stage_process_scheduler", ["Karras", "Simple", "SGM Uniform", "Normal", "Exponential"]);
guiControlPrepareNum("stage_process_seed_v");
guiControlPrepareNum("stage_process_seed_i");
guiControlPrepareOpt("stage_process_seed_d", ["HV", "VH", "H", "V"]);

guiControlPrepareNum("stage_process_cfg_v");
guiControlPrepareNum("stage_process_cfg_i");
guiControlPrepareOpt("stage_process_cfg_d", ["HV", "VH", "H", "V"]);
guiControlPrepareNum("stage_process_step_t_v");
guiControlPrepareNum("stage_process_step_t_i");
guiControlPrepareOpt("stage_process_step_t_d", ["HV", "VH", "H", "V"]);
guiControlPrepareNum("stage_process_step_b_v");
guiControlPrepareNum("stage_process_step_b_i");
guiControlPrepareOpt("stage_process_step_b_d", ["HV", "VH", "H", "V"]);
guiControlPrepareNum("stage_process_step_e_v");
guiControlPrepareNum("stage_process_step_e_i");
guiControlPrepareOpt("stage_process_step_e_d", ["HV", "VH", "H", "V"]);

guiControlPrepareTxt("stage_process_prompt_posi");
guiControlPrepareTxt("stage_process_prompt_nega");

guiControlPrepareOpt("stage_stage_type", ["Image", "Process"]);


guiControlPrepareOpt("stages", []);

guiStageRefresh();



function guiItemPrev()
{

    const stageList = document.getElementById("stages");
    const stage = stageList.selectedIndex;
    if (stage >= 1)
    {
        stageList.selectedIndex = stageList.selectedIndex - 1;
        projectStageShow();
    }
}

function guiItemNext()
{
    const stageList = document.getElementById("stages");
    const stage = stageList.selectedIndex;
    if (stage < (stageList.length - 1))
    {
        stageList.selectedIndex = stageList.selectedIndex + 1;
        projectStageShow();
    }
}
