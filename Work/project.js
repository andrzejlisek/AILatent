function btoa_utf8(text)
{
    //return btoa(unescape(encodeURIComponent(text)));
    const bytes = new TextEncoder().encode(text);
    const binString = Array.from(bytes, (byte) => String.fromCodePoint(byte)).join("");
    return btoa(binString);
}

function atob_utf8(base64)
{
    //return decodeURIComponent(escape(atob(base64)));
    const binString = atob(base64);
    const bytes = Uint8Array.from(binString, (char) => char.codePointAt(0));
    return new TextDecoder().decode(bytes);
}



function projectStageListRefresh()
{
    let stageNum = parseInt(window.asm_env.data("project_stage", -1, "", "", ""));
    
    let stageList = document.getElementById("stages");
    guiListClear(stageList);
    for (let i = 0; i < stageNum; i++)
    {
        guiListAdd(stageList, window.asm_env.data("project_stage", i, "", "", ""));
    }
    if (stageNum > 0)
    {
        stageList.selectedIndex = 0;
        projectStageShow();
    }
}

function projectLoad()
{
    let fileName = fileSelectedName();
    if (fileName)
    {
        fileLoad(fileName, projectLoadFinish);
    }
}

function projectLoadFinish(n, contents)
{
    let contentsRaw = contents.substring(contents.indexOf(',') + 1);
    contentsRaw = "" + atob_utf8(contentsRaw) + "";
    window.asm_env.data("project_load", 0, contentsRaw, "", "");
    
    projectStageListRefresh();
}

function projectSave()
{
    let contents = window.asm_env.data("project_save", 0, "", "", "");
    
    if (contents.length == 0) return;
    
    let contentsRaw = "data:text/plain;base64," + btoa_utf8(contents)
    
    let fileName = prompt("File name", fileSelectedName())
    {
        if (fileName)
        {
            filesWork = fileName;
            fileSave(fileName, contentsRaw, projectSaveFinish);
        }
    }
}

function projectSaveFinish()
{
    fileListDisp();
}

function projectStart(execType)
{
    let stageList = document.getElementById("stages");
    let indices = "" + (stageList.length + 2) + "";
    for (let i = 0; i < stageList.length; i++)
    {
        if (stageList[i].selected)
        {
            indices = indices + "," + i;
        }
    }
    const guiBatchW0 = parseInt(document.getElementById("projectBatchW").value);
    const guiBatchH0 = parseInt(document.getElementById("projectBatchH").value);
    window.asm_env.startproc(guiBatchW0, guiBatchH0, indices, execType);
}


function projectAdd()
{
    let stageList = document.getElementById("stages");
    let idx = parseInt(window.asm_env.data("stage_list", stageList.selectedIndex, "add", "", ""));
    if (idx >= 0)
    {
        guiListAdd(stageList, window.asm_env.data("project_stage", idx, "", "", ""));
        stageList.selectedIndex = idx;
        projectStageShow();
    }
}

function projectRem()
{
    let stageList = document.getElementById("stages");
    let idx = parseInt(window.asm_env.data("stage_list", stageList.selectedIndex, "rem", "", ""));
    if (idx >= 0)
    {
        guiListRem(stageList, stageList.selectedIndex);
        stageList.selectedIndex = idx;
        projectStageShow();
    }
}

function projectMvUp()
{
    let stageList = document.getElementById("stages");
    let idx = parseInt(window.asm_env.data("stage_list", stageList.selectedIndex, "mv_up", "", ""));
    if (idx >= 0)
    {
        guiStageRefreshInfo(idx);
        guiStageRefreshInfo(idx + 1);
        stageList.selectedIndex = idx;
        projectStageShow();
    }
}

function projectMvDn()
{
    let stageList = document.getElementById("stages");
    let idx = parseInt(window.asm_env.data("stage_list", stageList.selectedIndex, "mv_dn", "", ""));
    if (idx >= 0)
    {
        guiStageRefreshInfo(idx);
        guiStageRefreshInfo(idx - 1);
        stageList.selectedIndex = idx;
        projectStageShow();
    }
}


function projectConfig()
{
    fetch("config.txt")
    .then(_1 => _1.text())
    .then(_2 => projectConfigFinish(_2));
}

function projectConfigFinish(x)
{
    let engines = window.asm_env.data("config", 0, x, "", "").split('\n');
    
    engineList = document.getElementById("stage_process_model");
    guiListClear(engineList);
    for (i in engines)
    {
        guiListAdd(engineList, engines[i]);
    }
    projectStageListRefresh();
    maskInit();
    guiExecStop();
}


function projectStageShow()
{
    let stageList = document.getElementById("stages");
    if (stageList.selectedIndex >= 0)
    {
        let stage = stageList.selectedIndex;


        document.getElementById("stage_stage_type").selectedIndex = window.asm_env.data("project_value_get", stage, "stage_type", "", "");

        document.getElementById("stage_input_file").value = window.asm_env.data("project_value_get", stage, "input_file", "", "");
        document.getElementById("stage_input_row").value = window.asm_env.data("project_value_get", stage, "input_row", "", "");
        document.getElementById("stage_input_col").value = window.asm_env.data("project_value_get", stage, "input_col", "", "");
        document.getElementById("stage_input_width").value = window.asm_env.data("project_value_get", stage, "input_width", "", "");
        document.getElementById("stage_input_height").value = window.asm_env.data("project_value_get", stage, "input_height", "", "");
        document.getElementById("stage_input_zoom").value = window.asm_env.data("project_value_get", stage, "input_zoom", "", "");
        document.getElementById("stage_input_offsetx").value = window.asm_env.data("project_value_get", stage, "input_offsetx", "", "");
        document.getElementById("stage_input_offsety").value = window.asm_env.data("project_value_get", stage, "input_offsety", "", "");
        document.getElementById("stage_input_color_r").value = window.asm_env.data("project_value_get", stage, "input_color_r", "", "");
        document.getElementById("stage_input_color_g").value = window.asm_env.data("project_value_get", stage, "input_color_g", "", "");
        document.getElementById("stage_input_color_b").value = window.asm_env.data("project_value_get", stage, "input_color_b", "", "");
        guiInputValueColor(1, "stage_input_color", "stage_input_color_r", "stage_input_color_g", "stage_input_color_b");
        
        document.getElementById("stage_process_source").value = window.asm_env.data("project_value_get", stage, "process_source", "", "");
        document.getElementById("stage_process_source_type").selectedIndex = window.asm_env.data("project_value_get", stage, "process_source_type", "", "");
        document.getElementById("stage_process_type").selectedIndex = window.asm_env.data("project_value_get", stage, "process_type", "", "");
        guiListSetByText(document.getElementById("stage_process_model"), window.asm_env.data("project_value_get", stage, "process_model", "", ""));
        guiListSetByText(document.getElementById("stage_process_sampler"), window.asm_env.data("project_value_get", stage, "process_sampler", "", ""));
        guiListSetByText(document.getElementById("stage_process_scheduler"), window.asm_env.data("project_value_get", stage, "process_scheduler", "", ""));
        document.getElementById("stage_process_seed_v").value = window.asm_env.data("project_value_get", stage, "process_seed_v", "", "");
        document.getElementById("stage_process_seed_i").value = window.asm_env.data("project_value_get", stage, "process_seed_i", "", "");
        document.getElementById("stage_process_seed_d").selectedIndex = window.asm_env.data("project_value_get", stage, "process_seed_d", "", "");
        document.getElementById("stage_process_cfg_v").value = window.asm_env.data("project_value_get", stage, "process_cfg_v", "", "");
        document.getElementById("stage_process_cfg_i").value = window.asm_env.data("project_value_get", stage, "process_cfg_i", "", "");
        document.getElementById("stage_process_cfg_d").selectedIndex = window.asm_env.data("project_value_get", stage, "process_cfg_d", "", "");
        document.getElementById("stage_process_step_t_v").value = window.asm_env.data("project_value_get", stage, "process_step_t_v", "", "");
        document.getElementById("stage_process_step_t_i").value = window.asm_env.data("project_value_get", stage, "process_step_t_i", "", "");
        document.getElementById("stage_process_step_t_d").selectedIndex = window.asm_env.data("project_value_get", stage, "process_step_t_d", "", "");
        document.getElementById("stage_process_step_b_v").value = window.asm_env.data("project_value_get", stage, "process_step_b_v", "", "");
        document.getElementById("stage_process_step_b_i").value = window.asm_env.data("project_value_get", stage, "process_step_b_i", "", "");
        document.getElementById("stage_process_step_b_d").selectedIndex = window.asm_env.data("project_value_get", stage, "process_step_b_d", "", "");
        document.getElementById("stage_process_step_e_v").value = window.asm_env.data("project_value_get", stage, "process_step_e_v", "", "");
        document.getElementById("stage_process_step_e_i").value = window.asm_env.data("project_value_get", stage, "process_step_e_i", "", "");
        document.getElementById("stage_process_step_e_d").selectedIndex = window.asm_env.data("project_value_get", stage, "process_step_e_d", "", "");
        document.getElementById("stage_process_step_o_v").value = window.asm_env.data("project_value_get", stage, "process_step_o_v", "", "");
        document.getElementById("stage_process_step_o_i").value = window.asm_env.data("project_value_get", stage, "process_step_o_i", "", "");
        document.getElementById("stage_process_step_o_d").selectedIndex = window.asm_env.data("project_value_get", stage, "process_step_o_d", "", "");

        document.getElementById("stage_process_prompt_posi").value = window.asm_env.data("project_value_get", stage, "process_prompt_posi", "", "");
        document.getElementById("stage_process_prompt_nega").value = window.asm_env.data("project_value_get", stage, "process_prompt_nega", "", "");

        maskCtrlLoad();

        guiStageRefresh();

        projectStageShowPics();
    }
}


function projectStageShowPics()
{
    let stageList = document.getElementById("stages");
    if (stageList.selectedIndex >= 0)
    {
        let stage = stageList.selectedIndex;

        let screenData = window.asm_env.data("project_stage_data", stage, "", "", "").split(",");
        screenPrepareWidthHeight(parseInt(screenData[0]), parseInt(screenData[1]));
        for (let i = 0; i < (guiBatchW * guiBatchH); i++)
        {
            guiDispPic(stage, i);
        }
    }
}



function projectStageSet(ctrl, t)
{
    let stageList = document.getElementById("stages");
    if (stageList.selectedIndex >= 0)
    {
        let stage = stageList.selectedIndex;

        let ctrlObj = document.getElementById(ctrl);

        if (t == 1)
        {
            const v = "" + ctrlObj.value + "";
            window.asm_env.data("project_value_set", stage, ctrl.substring(6), v, "")
        }

        if (t == 2)
        {
            const v = "" + ctrlObj.options[ctrlObj.selectedIndex].value + "";
            window.asm_env.data("project_value_set", stage, ctrl.substring(6), v, "")
        }

        if (t == 3)
        {
            const v = "" + ctrlObj.selectedIndex + "";
            window.asm_env.data("project_value_set", stage, ctrl.substring(6), v, "")
        }

        if (t == 4)
        {
            const v_r = "" + document.getElementById(ctrl + "_r").value + "";
            const v_g = "" + document.getElementById(ctrl + "_g").value + "";
            const v_b = "" + document.getElementById(ctrl + "_b").value + "";
            window.asm_env.data("project_value_set", stage, (ctrl + "_r").substring(6), v_r, "")
            window.asm_env.data("project_value_set", stage, (ctrl + "_g").substring(6), v_g, "")
            window.asm_env.data("project_value_set", stage, (ctrl + "_b").substring(6), v_b, "")
        }
    }
}

function projectStageTestPass(addr, txt)
{
    alert(addr + "\nTest passed")
}

function projectStageTestFail(addr, txt)
{
    alert(addr + "\nTest failed\n" + txt)
}

function projectStageTest()
{
    let stageList = document.getElementById("stages");
    if (stageList.selectedIndex >= 0)
    {
        let stage = stageList.selectedIndex;

        let serverAddr = window.asm_env.data("project_stage_data", stage, "", "", "").split(",")[2];
        let serverType = window.asm_env.data("project_stage_data", stage, "", "", "").split(",")[3];

        if (serverType == "comfyui")
        {
            //serverAddr = serverAddr + "/object_info";
            serverAddr = serverAddr + "/system_stats";
        }
        if (serverType == "a1111")
        {
            serverAddr = serverAddr + "/config";
            //serverAddr = serverAddr + "/info";
        }
        let serverInfo = serverType + "\n" + serverAddr;

        fetch(serverAddr)
        .then(_1 => {
            if (!_1.ok)
            {
                throw new Error("Server error: " + _1.status);
            }
            return _1.text();
        })
        .then(_2 => projectStageTestPass(serverInfo, _2)).catch(_2 => projectStageTestFail(serverInfo, _2.message));
    }
}

function projectSource()
{
    const ctrl1 = document.getElementById("stage_stage_type");
    ctrl1.selectedIndex = 0;
    ctrl1.dispatchEvent(new Event('input', { bubbles: true }));
    const ctrl2 = document.getElementById("stage_input_file");
    ctrl2.value = fileSelectedName();
    ctrl2.dispatchEvent(new Event('input', { bubbles: true }));
}

