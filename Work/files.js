let filesWork = "";

function fileSelectedName()
{
    const list = document.getElementById("filelist");
    if (list.selectedIndex < 0) return "";
    return list.options[list.selectedIndex].value;
}

function fileUpload()
{
    let input = document.createElement("input");
    input.type = "file";

    input.onchange = function(event) {
        let file = event.target.files[0];
        if (!file) return;

        let reader = new FileReader();

        reader.onload = function(e) {
            name = prompt("File name", file.name);
            filesWork = name;
            fileSave(name, e.target.result, fileUploadFinish);
        };

        reader.readAsDataURL(file);
    };

    input.click();
}

function fileUploadFinish()
{
    fileListDisp();
}


function fileDownload()
{
    const list = document.getElementById("filelist");
    if (list.selectedIndex < 0) return;
    const name = list.options[list.selectedIndex].value;
    filesWork = name;
    fileLoad(name, fileDownloadFinish);
}

function fileDownloadFinish(name, contents)
{
    if ((name) && (contents))
    {
        let a = document.createElement("a");
        a.href = contents;
        a.download = name;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    }
}

function fileRemove()
{
    filesWork = "";
    const list = document.getElementById("filelist");
    if (list.selectedIndex < 0) return;
    const name = list.options[list.selectedIndex].value;
    if (confirm("Are you sure you want to remove the " + name))
    {
        fileSave(name, "", fileRemoveFinish);
    }
}

function fileRemoveFinish()
{
    fileListDisp();
}

function fileRename()
{
    filesWork = "";
    const list = document.getElementById("filelist");
    if (list.selectedIndex < 0) return;
    const name = list.options[list.selectedIndex].value;
    fileLoad(name, fileRenameWork);
}

function fileRenameWork(name, contents)
{
    let namex = prompt("File name", name);

    if ((name) && (namex) && (name != namex))
    {
        filesWork = namex;
        fileRenameFinishX = 0;
        fileSave(name, "", fileRenameFinish1);
        fileSave(namex, contents, fileRenameFinish2);
    }
}

let fileRenameFinishX;

function fileRenameFinish1(name, contents)
{
    fileRenameFinishX = fileRenameFinishX + 1;
    if (fileRenameFinishX == 2) fileRenameFinish();
}

function fileRenameFinish2(name, contents)
{
    fileRenameFinishX = fileRenameFinishX + 1;
    if (fileRenameFinishX == 2) fileRenameFinish();
}

function fileRenameFinish()
{
    fileListDisp();
}



function fileType()
{
    const list = document.getElementById("filelist");
    if (list.selectedIndex < 0) return;
    const name = list.options[list.selectedIndex].value;
    fileLoad(name, fileTypeFinish);
}

function fileTypeFinish(name, contents)
{
    const idx = contents.indexOf(",");
    if (idx > 0)
    {
        alert(contents.substring(0, idx));
    }
}


function fileListDisp()
{
    fileList(fileListDispResult);
}

function fileListDispResult(fl)
{
    const list = document.getElementById("filelist");
    list.options.length = 0;
    let idx = -1;
    for (i in fl)
    {
        let item = document.createElement("option");
        if (fl[i] == filesWork)
        {
            idx = i;
        }
        item.value = fl[i];
        item.textContent = fl[i];
        list.appendChild(item);
    }
    list.selectedIndex = idx;
}

fileListDisp();

