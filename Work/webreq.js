function webRequest_base64ToBlob(base64, mimeType)
{
    mimeType = mimeType || 'application/octet-stream';
    const byteCharacters = atob(base64);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++)
    {
        byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    return new Blob([byteArray], { type: mimeType });
}

function webRequest_arrayBufferToBase64(buffer)
{
    return new Promise(function(resolve)
    {
        const reader = new FileReader();
        reader.onload = function()
        {
            resolve(reader.result.split(',')[1]);
        };
        reader.readAsDataURL(new Blob([buffer]));
    });
}

function webRequest(urlAddr, headerParameters, requestBody, answerFunction, answerInfo)
{
    try
    {
        let binaryI = false;
        let binaryO = false;
        let uploadFileName = "";
        let HttpCustMethod = "";

        const headers = new Headers();

        if (headerParameters && headerParameters.length > 0)
        {
            const headerSplitter = headerParameters.charAt(headerParameters.length - 1);
            let headerStep = 0;

            while (headerStep < headerParameters.length)
            {
                const header1 = headerParameters.indexOf(headerSplitter, headerStep);
                const header2 = headerParameters.indexOf(headerSplitter, header1 + 1);

                if (header1 > headerStep && header2 > header1)
                {
                    const headerKey = headerParameters.substring(headerStep, header1);
                    const headerVal = headerParameters.substring(header1 + 1, header2);
                    let std = true;

                    if (headerKey === "X_BINARYI")
                    {
                        std = false;
                        binaryI = true;
                        uploadFileName = headerVal;
                    }
                    else
                    {
                        if (headerKey === "X_BINARYO")
                        {
                            std = false;
                            binaryO = true;
                        }
                        else
                        {
                            if (headerKey === "X_METHOD")
                            {
                                std = false;
                                HttpCustMethod = headerVal;
                            }
                        }
                    }

                    if (std)
                    {
                        headers.append(headerKey, headerVal);
                    }
                    headerStep = header2 + 1;
                }
                else
                {
                    headerStep = headerParameters.length;
                }
            }
        }

        const fetchOptions = {
            headers: headers
        };

        if (binaryI)
        {
            fetchOptions.method = 'POST';
            const formData = new FormData();
            const fileBlob = webRequest_base64ToBlob(requestBody);

            formData.append('image', fileBlob, uploadFileName);
            formData.append('overwrite', 'true');

            fetchOptions.body = formData;
        }
        else
        {
            headers.append("Content-Type", "application/json");

            if (!HttpCustMethod)
            {
                HttpCustMethod = requestBody ? "POST" : "GET";
            }

            fetchOptions.method = HttpCustMethod;

            if ((HttpCustMethod === "POST" || HttpCustMethod === "PUT") && requestBody)
            {
                fetchOptions.body = requestBody;
            }
        }

        fetch(urlAddr, fetchOptions)
            .then(function(response) {
                if (binaryO)
                {
                    response.arrayBuffer()
                        .then(function(buffer) {
                            return webRequest_arrayBufferToBase64(buffer);
                        })
                        .then(function(base64Response) {
                            if (response.status === 200)
                            {
                                answerFunction(base64Response, answerInfo);
                            }
                            else
                            {
                                answerFunction("ERROR\n" + response.status + "\n" + base64Response, answerInfo);
                            }
                        })
                        .catch(function(err) {
                            answerFunction("ERROR\n" + err.message, answerInfo);
                        });
                } else {
                    response.text()
                        .then(function(textResponse) {
                            if (response.status === 200)
                            {
                                answerFunction(textResponse, answerInfo);
                            }
                            else
                            {
                                let formattedText = textResponse;
                                try
                                {
                                    formattedText = JSON.stringify(JSON.parse(textResponse), null, 2);
                                }
                                catch (e)
                                {
                                }

                                answerFunction("ERROR\n" + response.status + "\n" + formattedText, answerInfo);
                            }
                        })
                        .catch(function(err) {
                            answerFunction("ERROR\n" + err.message, answerInfo);
                        });
                }
            })
            .catch(function(error) {
                answerFunction("ERROR\n" + error.message, answerInfo);
            });

    }
    catch (error)
    {
        answerFunction("ERROR\n" + error.message, answerInfo);
    }
}

