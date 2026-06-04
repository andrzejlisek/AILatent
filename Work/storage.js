const filePrefix = "AILatent_File_";
const dbName = "AILatentDB";
const storeName = "files";
let dbInstance = null;

let fileGetDBPersist = false;

function fileGetDB(callback)
{
    if (fileGetDBPersist)
    {
        fileGetDB_base(callback);
    }
    else
    {
        fileGetDBPersist = true;
        if (navigator.storage && navigator.storage.persist)
        {
            try
            {
                navigator.storage.persist()
                    .then(() => fileGetDB_base(callback))
                    .catch(() => fileGetDB_base(callback));
            }
            catch (error)
            {
                fileGetDB_base(callback);
            }
        }
        else
        {
            fileGetDB_base(callback);
        }
    }
}

function fileGetDB_base(callback)
{
    if (dbInstance)
    {
        callback(dbInstance);
        return;
    }
    const request = indexedDB.open(dbName, 1);

    request.onupgradeneeded = function(event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains(storeName))
        {
            db.createObjectStore(storeName);
        }
    };

    request.onsuccess = function(event) {
        dbInstance = event.target.result;
        callback(dbInstance);
    };

    request.onerror = function(event) {
        console.error("Błąd otwierania IndexedDB:", event.target.error);
    };
}

function fileList_ls(answer)
{
    let fileName = [];
    for (var item in localStorage)
    {
        if (item.substr(0, filePrefix.length) == filePrefix)
        {
            fileName.push(item.substr(filePrefix.length));
        }
    }
    fileName.sort();
    setTimeout(answer, 1, fileName);
}

function fileList(answer)
{
    fileGetDB(function(db) {
        const transaction = db.transaction([storeName], "readonly");
        const store = transaction.objectStore(storeName);
        const request = store.getAllKeys();

        request.onsuccess = function() {
            let fileName = request.result;
            fileName.sort();
            answer(fileName);
        };
    });
}

function fileLoad_ls(name, answer)
{
    if ((name) && localStorage[filePrefix + name])
    {
        const data = localStorage.getItem(filePrefix + name);
        setTimeout(answer, 1, name, data);
    }
    else
    {
        setTimeout(answer, 1, name, "");
    }
}

function fileLoad(name, answer)
{
    if (!name)
    {
        answer(name, "");
        return;
    }

    fileGetDB(function(db) {
        const transaction = db.transaction([storeName], "readonly");
        const store = transaction.objectStore(storeName);
        const request = store.get(name);

        request.onsuccess = function() {
            const data = request.result;
            if (data !== undefined)
            {
                answer(name, data);
            }
            else
            {
                answer(name, "");
            }
        };

        request.onerror = function() {
            answer(name, "");
        };
    });
}

function fileSave_ls(name, data, answer)
{
    if ((data.length > 0) && (name))
    {
        try
        {
            localStorage.setItem(filePrefix + name, data);
        }
        catch (error)
        {
            console.warn("File save error [" + name + "]: " + error)
            localStorage.removeItem(filePrefix + name);
        }
        setTimeout(answer, 1, name);
    }
    else
    {
        localStorage.removeItem(filePrefix + name);
        setTimeout(answer, 1, name);
    }
}

function fileSave(name, data, answer)
{
    if (!name)
    {
        answer(name);
        return;
    }

    fileGetDB(function(db) {
        const transaction = db.transaction([storeName], "readwrite");
        const store = transaction.objectStore(storeName);

        if (data && data.length > 0)
        {
            const request = store.put(data, name);

            request.onsuccess = function() {
                answer(name);
            };

            request.onerror = function(event) {
                console.warn("File save error [" + name + "]: " + event.target.error);
                store.delete(name);
                answer(name);
            };
        }
        else
        {
            const request = store.delete(name);
            request.onsuccess = function() {
                answer(name);
            };
        }
    });
}



function migrate()
{
    fileList_ls(migrate1);
}

function migrate1(files)
{
    for (fileItem in files)
    {
        const n = files[fileItem];
        fileLoad_ls(n, migrate2);
    }
}

function migrate2(name, data)
{
    fileSave(name, data, migrate3);
}

function migrate3(name)
{
}

//migrate();

