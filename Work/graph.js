let graphCurrentParams = [];

function graphPrefix(raw)
{
    return "data:image/png;base64," + raw;
}

function graphPrepare(raw, answer)
{
    let prepareW = parseInt(graphCurrentParams[0]);
    let prepareH = parseInt(graphCurrentParams[1]);
    let prepareZoom = parseInt(graphCurrentParams[2]);
    let prepareX = parseInt(graphCurrentParams[3]);
    let prepareY = parseInt(graphCurrentParams[4]);
    let prepareR = parseInt(graphCurrentParams[5]);
    let prepareG = parseInt(graphCurrentParams[6]);
    let prepareB = parseInt(graphCurrentParams[7]);
    
    const img = new Image();
    img.onload = function() {

        if (prepareW <= 0) { prepareW = img.naturalWidth; }
        if (prepareH <= 0) { prepareH = img.naturalHeight; }

        const tempCnv = document.createElement("canvas");
        tempCnv.width = prepareW;
        tempCnv.height = prepareH;
        const tempCtx = tempCnv.getContext("2d");

        graphPrepareImg(img, prepareR, prepareG, prepareB, 0, 0, prepareW, prepareH, prepareZoom, prepareX, prepareY, tempCnv, tempCtx);


        const result = tempCnv.toDataURL("image/png");
        answer(result);
    };
    img.onerror = function() {
        answer("");
    };
    img.src = raw;
}

function graphPrepareImg(img, marginR, marginG, marginB, idxX, idxY, idxW, idxH, xZoom, xOffsetX, xOffsetY, tempCnv, tempCtx)
{
    tempCtx.fillStyle = "rgb(" + marginR + "," + marginG + "," + marginB + ")";
    //tempCtx.fillStyle = "black";
    tempCtx.fillRect(idxX, idxY, idxW, idxH);

    //screenCtx.fillStyle = "green";
    //screenCtx.fillRect(idxX + 2, idxY + 2, idxW - 4, idxH - 4);

    if (!img)
    {
        return;
    }

    const imgW = img.naturalWidth; 
    const imgH = img.naturalHeight;

    let paintW = idxW;
    let paintH = idxH;

    if (xZoom > 0)
    {
        if ((idxW / idxH) > (imgW / imgH))
        {
            paintH = idxH * xZoom / 100;
            paintW = imgW * paintH / imgH;
        }
        else
        {
            paintW = idxW * xZoom / 100;
            paintH = imgH * paintW / imgW;
        }
    }
    if (xZoom < 0)
    {
        if ((idxW / idxH) > (imgW / imgH))
        {
            paintW = idxW * (0 - xZoom) / 100;
            paintH = imgH * paintW / imgW;
        }
        else
        {
            paintH = idxH * (0 - xZoom) / 100;
            paintW = imgW * paintH / imgH;
        }
    }


    let imgOffsetX = (paintW * xOffsetX / 200);
    let imgOffsetY = (paintH * xOffsetY / 200);

    let srcX = 0;
    let srcY = 0;
    let srcW = imgW;
    let srcH = imgH;
    let dstX = Math.round((idxW - paintW) / 2 + imgOffsetX);
    let dstY = Math.round((idxH - paintH) / 2 + imgOffsetY);
    let dstW = Math.round(paintW);
    let dstH = Math.round(paintH);

    let imgBounds = [imgW, imgH, dstX, dstY, dstW, dstH];
    
    if (dstX < 0)
    {
        srcX = Math.round((0 - dstX) * srcW / dstW);
        dstX = 0;
    }
    if (dstY < 0)
    {
        srcY = Math.round((0 - dstY) * srcH / dstH);
        dstY = 0;
    }
    if (dstW > (idxW - dstX))
    {
        srcW = Math.round(srcW - (dstW + dstX - idxW) * srcW / dstW);
        dstW = idxW - dstX;
    }
    if (dstH > (idxH - dstY))
    {
        srcH = Math.round(srcH - (dstH + dstY - idxH) * srcH / dstH);
        dstH = idxH - dstY;
    }

    tempCtx.drawImage(img, srcX, srcY, srcW, srcH, idxX + dstX, idxY + dstY, dstW, dstH);
    return imgBounds;
}



function timestamp()
{
    const now = new Date();

    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');

    const hours = String(now.getHours()).padStart(2, '0');
    const minutes = String(now.getMinutes()).padStart(2, '0');
    const seconds = String(now.getSeconds()).padStart(2, '0');

    return `${year}${month}${day}${hours}${minutes}${seconds}`;
}

function numPad(num, maxVal)
{
    const numLen = String(maxVal).length;
    return String(num).padStart(numLen, '0');
}



function graphDownloadCollage()
{
    let imgW = 0;
    let imgH = 0;
    for (let i = 0; i < screenPics.length; i++)
    {
        if (screenPics[i])
        {
            let img = screenPics[i];
            imgW = Math.max(imgW, img.naturalWidth);
            imgH = Math.max(imgH, img.naturalHeight);
        }
    }
    
    if (imgW == 0) return;
    if (imgH == 0) return;

    const tempCnv = document.createElement("canvas");
    tempCnv.width = imgW * guiBatchW;
    tempCnv.height = imgH * guiBatchH;
    const tempCtx = tempCnv.getContext("2d");

    let idx = 0;
    for (let y = 0; y < guiBatchH; y++)
    {
        for (let x = 0; x < guiBatchW; x++)
        {
            graphPrepareImg(screenPics[idx], 0, 0, 0, x * imgW, y * imgH, imgW, imgH, 100, 0, 0, tempCnv, tempCtx);
            idx++;
        }
    }

    let raw = tempCnv.toDataURL("image/png");
    graphDownloadFile("ailatent_" + timestamp() + ".png", raw);
}

function graphDownloadImages()
{
    for (let i = 0; i < screenPics.length; i++)
    {
        if ((screenDispIdx == 0) || (screenDispIdx == (i + 1)))
        {
            if (screenPics[i])
            {
                let img = screenPics[i];

                let imgW = img.naturalWidth;
                let imgH = img.naturalHeight;

                const tempCnv = document.createElement("canvas");
                tempCnv.width = imgW;
                tempCnv.height = imgH;
                const tempCtx = tempCnv.getContext("2d");

                graphPrepareImg(img, 0, 0, 0, 0, 0, imgW, imgH, 0, 0, 0, tempCnv, tempCtx);
                let raw = tempCnv.toDataURL("image/png");
                
                graphDownloadFile("ailatent_" + timestamp() + "_" + numPad(i + 1, screenPics.length) + ".png", raw);
            }
        }
    }
}

function graphDownloadFile(name, contents)
{
    fileDownloadFinish(name, contents);
    return;
    
    if ((name) && (contents))
    {
        try
        {
            fetch(contents).then(_1 => _1.blob()).then(_2 => graphDownloadFileFinish(_2));
        }
        catch
        {
        }
    }    
}

function graphDownloadFileFinish(blob)
{
    const blobUrl = URL.createObjectURL(blob);
    window.open(blobUrl, '_blank');
    setTimeout(() => URL.revokeObjectURL(blobUrl), 1000);    
}

function graphCanvasPreview(cnv)
{
    const newTab = window.open("", "_blank");
    cnv.toBlob(function(blob) {
        const url = URL.createObjectURL(blob);
        newTab.location.href = url;
        setTimeout(() => URL.revokeObjectURL(url), 10000);
    }, "image/png");

}

function graphCanvasToFile(cnv)
{
    let temp = cnv.toDataURL("image/png");
    let idx = temp.indexOf(',');
    if (idx > 0)
    {
        return temp.substr(idx + 1);
    }
    return temp;
}
