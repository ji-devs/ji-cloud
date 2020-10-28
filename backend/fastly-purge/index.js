const configPath = process.env["CONFIG_PATH"] ? process.env["CONFIG_PATH"] : "./config";

const fetch = require('node-fetch');
const {Storage} = require('@google-cloud/storage');
const storage = new Storage();
const CONFIG = require(configPath);

const hasExtension = ext => target => {
  const idx = target.lastIndexOf('.');
  if(idx === -1 || idx === target.length-1) {
    return false;
  }

  const str = target.substr(idx + 1);

  return str.toLowerCase() === ext.toLowerCase();
}


const hasWasmExtension = hasExtension("wasm");

const getMetadata = filename => {

    //these don't get dynamic filenames and do require immediate changes in the browser
    //they are still cached on the fastly side
    const noBrowserCache = ["html", "css", "js"].some(ext => hasExtension(ext) (filename));

    if(noBrowserCache) {
        console.log(`not caching ${filename} in browser`);
    } else {
        console.log(`caching ${filename} in browser`);
    }

    let metaData = noBrowserCache
        ?   {
                cacheControl: "no-cache, no-store, max-age=0, must-revalidate",
                //doesn't work
                //surrogateControl: "max-age=2628000",
            }
        :   {
                cacheControl: "public, max-age=3600",
                //doesn't work
                //surrogateControl: "max-age=2628000",
            };

    if(hasWasmExtension(filename)) {
        console.log(`${filename} is wasm, so changing contentType`);
        metaData.contentType = 'application/wasm';
    }

    return metaData;

}
const makePurger = FASTLY_PUBLIC_BASEURL => async (obj, context) => {
    const baseUrl = FASTLY_PUBLIC_BASEURL.replace(/\/+$/, '');
    const filename = obj.name.replace(/^\/+/, '');
    const completeObjectUrl = `${baseUrl}/${filename}`;

    console.log(`got purge request for object: ${obj.name} in bucket ${obj.bucket} filename: ${filename}`);
    const file = storage.bucket(obj.bucket).file(obj.name);

    try {
        const existsData = await file.exists();
        const exists = existsData[0];

        if(!exists) {
            console.warn(`${filename} doesn't exist in storage (kinda weird), so not setting metadata`);
        } else {
            console.log(`${filename} exists, so setting metadata`);
            await storage.bucket(obj.bucket).file(obj.name).setMetadata(getMetadata(filename));
        }
        console.log(`making purge request for ${completeObjectUrl}`);

        const resp = await fetch(completeObjectUrl, { method: 'PURGE'});

        if (!resp.ok) {
            throw new Error('Unexpected status ' + resp.status);
        }
         
        const data = await resp.json();

        console.log(`Purged ${filename}, ID ${data.id}`);
    } catch(err) {
            console.error("got error in purge!");
            console.error(err);
    }
};

exports.purgeMedia = makePurger(CONFIG.URL_MEDIA);

exports.purgeFrontendRelease = makePurger(CONFIG.URL_FRONTEND_RELEASE);
exports.purgeFrontendSandbox = makePurger(CONFIG.URL_FRONTEND_SANDBOX);
exports.purgeUploadsRelease = makePurger(CONFIG.URL_UPLOADS_RELEASE);
exports.purgeUploadsSandbox = makePurger(CONFIG.URL_UPLOADS_SANDBOX);
