let screenDisp = 0;
let screenZoom = 100;
let screenOffsetX = 0;
let screenOffsetY = 0;

let screenDispIdx = 0;

let screenDiv = document.getElementById("screenWindow");
let screenCanvas = document.getElementById("screen");
let screenCtx = screenCanvas.getContext("2d");
let screenPicW = 1;
let screenPicH = 1;
let screenPics = [];

let screenExtWindow = null;



function screenNewWindow(isWin)
{
    if (screenExtWindow && !screenExtWindow.closed)
    {
        screenExtWindow.focus();
        return;
    }

    let windowFeatures = `width=640,height=480,menubar=no,toolbar=no,location=no,status=no,scrollbars=no,resizable=yes`;

    if (isWin)
    {
        screenExtWindow = window.open("", "_blank", windowFeatures);
    }
    else
    {
        screenExtWindow = window.open("", "_blank");
    }

    screenExtWindow.document.write(`
        <!DOCTYPE html>
        <html>
        <head>
            <title>AILatent screen</title>
            <style>
                body { margin: 0; background-color: #000000; overflow: hidden; }
                #screenWindow { width: 100vw; height: 100vh; display: flex; justify-content: center; align-items: center; }
            </style>
        </head>
        <body>
            <div id="screenWindow"><canvas id="screen"></canvas></div>
        </body>
        </html>
    `);
    screenExtWindow.document.close();

    document.getElementById("screenContainer").style.display = "none";

    screenObserver.disconnect();

    screenDiv = screenExtWindow.document.getElementById("screenWindow");
    screenCanvas = screenExtWindow.document.getElementById("screen");
    screenCtx = screenCanvas.getContext("2d");

    screenCanvas.addEventListener('mousedown', handleInteraction1);
    screenCanvas.addEventListener('mousemove', handleInteraction2);
    screenCanvas.addEventListener('mouseup', handleInteraction3);
    screenCanvas.addEventListener('mouseleave', handleInteraction3);

    screenCanvas.addEventListener('touchstart', handleInteraction1, { passive: false });
    screenCanvas.addEventListener('touchmove', handleInteraction2, { passive: false });
    screenCanvas.addEventListener('touchend', handleInteraction3, { passive: false });
    screenCanvas.addEventListener('touchcancel', handleInteraction3, { passive: false });



//    screenObserver.observe(screenDiv);
    screenExtWindow.addEventListener("resize", screenPrepare);

    screenPrepare();

    screenExtWindow.addEventListener("unload", function() {
        document.getElementById("screenContainer").style.display = "block";

        screenDiv = document.getElementById("screenWindow");
        screenCanvas = document.getElementById("screen");
        screenCtx = screenCanvas.getContext("2d");

        screenCanvas.addEventListener('mousedown', handleInteraction1);
        screenCanvas.addEventListener('mousemove', handleInteraction2);
        screenCanvas.addEventListener('mouseup', handleInteraction3);
        screenCanvas.addEventListener('mouseleave', handleInteraction3);

        screenCanvas.addEventListener('touchstart', handleInteraction1, { passive: false });
        screenCanvas.addEventListener('touchmove', handleInteraction2, { passive: false });
        screenCanvas.addEventListener('touchend', handleInteraction3, { passive: false });
        screenCanvas.addEventListener('touchcancel', handleInteraction3, { passive: false });

        screenObserver.observe(screenDiv);
 
        screenPrepare();
        screenExtWindow = null;
    });
}





function screenRepaintAll()
{
    const screenZoomVal = parseInt(document.getElementById("screenZoom").value);
    if (screenZoomVal != 0)
    {
        screenZoom = Math.round(Math.pow(10, Math.abs(screenZoomVal) * 0.1)) * Math.sign(screenZoomVal);
    }
    else
    {
        screenZoom = 0;
    }
    screenOffsetX = parseInt(document.getElementById("screenOffsetX").value);
    screenOffsetY = parseInt(document.getElementById("screenOffsetY").value);
    
    const ScreenSizeVal = parseInt(document.getElementById("screenSize").value);
    
    screenWindow.style.height = ScreenSizeVal + "vh";
    
    document.getElementById("screenH").innerHTML = ((screenOffsetX > 0) ? "+" : "") + screenOffsetX;
    document.getElementById("screenV").innerHTML = ((screenOffsetY > 0) ? "+" : "") + screenOffsetY;
    
    document.getElementById("screenZ").innerHTML = ((screenZoom > 0) ? "+" : "") + screenZoom;
    document.getElementById("screenS").innerHTML = ScreenSizeVal;
    
    for (let i = 0; i < (guiBatchW * guiBatchH); i++)
    {
        screenRepaint(i);
    }
}


function screenRepaintAllPics()
{
    for (let i = 0; i < (guiBatchW * guiBatchH); i++)
    {
        screenRepaint(i);
    }
}

function screenRepaint(idx)
{
    let idxX = (idx % guiBatchW) * screenPicW;
    let idxY = ((idx - (idx % guiBatchW)) / guiBatchW) * screenPicH;
    let idxW = screenPicW;
    let idxH = screenPicH;

    if (screenDispIdx > 0)
    {
        if (screenDispIdx == (idx + 1))
        {
            idxX = 0;
            idxY = 0;
            idxW = screenPicW * guiBatchW;
            idxH = screenPicH * guiBatchH;
        }
        else
        {
            return;
        }
    }


    screenCtx.fillStyle = "black";
    screenCtx.fillRect(idxX, idxY, idxW, idxH);
    
    //screenCtx.fillStyle = "green";
    //screenCtx.fillRect(idxX + 2, idxY + 2, idxW - 4, idxH - 4);
    
    if (screenPics[idx])
    {
        const imgParam = graphPrepareImg(screenPics[idx], 0, 0, 0, idxX, idxY, idxW, idxH, screenZoom, screenOffsetX, screenOffsetY, screenCanvas, screenCtx);
        maskPaint(screenCtx, imgParam, idxX, idxY, idxW, idxH, 0);
    }
}

function screenSetPicture(idx, data)
{
    if (data.length > 0)
    {
        const img = new Image();
        img.onload = function() {
            screenPics[idx] = img;
            screenRepaint(idx);
        };
        img.onerror = function() {
            screenPics[idx] = null;
            screenRepaint(idx);
        };
        img.src = data;
    }
    else
    {
        screenPics[idx] = null;
        screenRepaint(idx);
    }
}


function screenPrepare()
{
    screenDims = screenDiv.getBoundingClientRect();

    screenPicW = Math.floor(screenDims.width / guiBatchW);
    screenPicH = Math.floor(screenDims.height / guiBatchH);
    const scrW = screenPicW * guiBatchW;
    const scrH = screenPicH * guiBatchH;
    screenCanvas.width = scrW;
    screenCanvas.height = scrH;

    if (screenPics.length != (guiBatchW * guiBatchH))
    {
        screenPics = [];
        for (let i = 0; i < (guiBatchW * guiBatchH); i++)
        {
            screenPics.push(null);
        }
    }
    screenRepaintAll();
}

function screenPrepareWidthHeight(w, h)
{
    if ((guiBatchW != w) || (guiBatchH != h))
    {
        guiBatchW = w;
        guiBatchH = h;
        screenPrepare();
    }
}

const screenObserver = new ResizeObserver(entries => {
    for (let entry of entries)
    {
        if (entry.target === screenDiv)
        {
            screenPrepare();
        }
    }
});

screenObserver.observe(screenDiv);







function getCanvasCoordinates(event) {
    const rect = screenCanvas.getBoundingClientRect();

    let clientX, clientY;

    if (event.touches && event.touches.length > 0) {
        clientX = event.touches[0].clientX;
        clientY = event.touches[0].clientY;
    } else {
        clientX = event.clientX;
        clientY = event.clientY;
    }

    const x = (clientX - rect.left) * (screenCanvas.width / rect.width);
    const y = (clientY - rect.top) * (screenCanvas.height / rect.height);

    return {
        x: Math.round(x),
        y: Math.round(y)
    };
}

let handleInteractionBtn = false;

function handleInteraction1(event)
{
    event.preventDefault();
    handleInteractionBtn = true;
    
    const coords = getCanvasCoordinates(event);
    if (maskState > 0)
    {
        maskMouse(coords.x, coords.y, 1);
        return;
    }

    const idxX = Math.floor(coords.x / screenPicW);
    const idxY = Math.floor(coords.y / screenPicH);
    
    if (screenDispIdx > 0)
    {
        let x = coords.x;
        let y = coords.y;
        let t = 0;
        
        const cnvW = (screenPicW * guiBatchW) / 2;
        const cnvH = (screenPicH * guiBatchH) / 2;
       
        if (x < cnvW)
        {
            if (y < cnvH)
            {
                if ((x / y) > (cnvW / cnvH)) { t = 1; } else { t = 4; }
            }
            else
            {
                if ((x / (cnvH + cnvH - y)) > (cnvW / cnvH)) { t = 2; } else { t = 4; }
            }
        }
        else
        {
            if (y < cnvH)
            {
                if (((cnvW + cnvW - x) / y) > (cnvW / cnvH)) { t = 1; } else { t = 3; }
            }
            else
            {
                if (((cnvW + cnvW - x) / (cnvH + cnvH - y)) > (cnvW / cnvH)) { t = 2; } else { t = 3; }
            }
        }



        const middlePercent = 50;
        if ((x >= (middlePercent * cnvW / 100)) && (x <= ((200 - middlePercent) * cnvW / 100)))
        {
            if ((y >= (middlePercent * cnvH / 100)) && (y <= ((200 - middlePercent) * cnvH / 100)))
            {
                t = 5;
            }
        }

        switch (t)
        {
            case 1:
                screenDispIdx = screenDispIdx - guiBatchW;
                if (screenDispIdx <= 0)
                {
                    if (screenDispIdx == (1 - guiBatchW))
                    {
                        screenDispIdx += guiBatchW;
                    }
                    screenDispIdx += (guiBatchW * guiBatchH)
                    screenDispIdx--;
                }
                break;
            case 2:
                screenDispIdx = screenDispIdx + guiBatchW;
                if (screenDispIdx > (guiBatchW * guiBatchH))
                {
                    if (screenDispIdx == ((guiBatchW * guiBatchH) + guiBatchW))
                    {
                        screenDispIdx -= guiBatchW;
                    }
                    screenDispIdx -= (guiBatchW * guiBatchH)
                    screenDispIdx++;
                }
                break;
            case 3:
                screenDispIdx = screenDispIdx + 1;
                if (screenDispIdx > (guiBatchW * guiBatchH))
                {
                    screenDispIdx -= (guiBatchW * guiBatchH)
                }
                break;
            case 4:
                screenDispIdx = screenDispIdx - 1;
                if (screenDispIdx <= 0)
                {
                    screenDispIdx += (guiBatchW * guiBatchH)
                }
                break;
            case 5:
                screenDispIdx = 0;
                break;
        }
    }
    else
    {
        screenDispIdx = (guiBatchW * idxY + idxX) + 1;
    }
    screenRepaintAll();
}

function handleInteraction2(event)
{
    event.preventDefault();
    if (handleInteractionBtn)
    {
        const coords = getCanvasCoordinates(event);
        if (maskState > 0)
        {
            maskMouse(coords.x, coords.y, 2);
        }
    }
}

function handleInteraction3(event)
{
    event.preventDefault();
    if (handleInteractionBtn)
    {
        handleInteractionBtn = false;
        const coords = getCanvasCoordinates(event);
        if (maskState > 0)
        {
            maskMouse(coords.x, coords.y, 3);
        }
    }
}

screenCanvas.addEventListener('mousedown', handleInteraction1);
screenCanvas.addEventListener('mousemove', handleInteraction2);
screenCanvas.addEventListener('mouseup', handleInteraction3);
screenCanvas.addEventListener('mouseleave', handleInteraction3);

screenCanvas.addEventListener('touchstart', handleInteraction1, { passive: false });
screenCanvas.addEventListener('touchmove', handleInteraction2, { passive: false });
screenCanvas.addEventListener('touchend', handleInteraction3, { passive: false });
screenCanvas.addEventListener('touchcancel', handleInteraction3, { passive: false });



window.addEventListener('unload', function()
{
    if (screenExtWindow && !screenExtWindow.closed)
    {
        screenExtWindow.close();
    }
});

