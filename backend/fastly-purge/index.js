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

  return str === ext;
}

const hasWasmExtension = hasExtension("wasm");

const getMetadata = (cacheInBrowser) => {
    return cacheInBrowser
        ?   {
                cacheControl: "max-age=3600",
            }
        :   {
                cacheControl: "no-store, must-revalidate",
                surrogateControl: "max-age=3600",
            };

}
const makePurger = (FASTLY_PUBLIC_BASEURL, cacheInBrowser) => async (obj, context) => {
    const baseUrl = FASTLY_PUBLIC_BASEURL.replace(/\/+$/, '');
    const fileName = obj.name.replace(/^\/+/, '');
    const completeObjectUrl = `${baseUrl}/${fileName}`;

    console.log(`got purge request for object: ${obj.name} in bucket ${obj.bucket} filename: ${fileName}`);
    const file = storage.bucket(obj.bucket).file(obj.name);

    try {
        const existsData = await file.exists();
        const exists = existsData[0];

        if(!exists) {
            console.warn(`${fileName} doesn't exist in storage (kinda weird), so not setting metadata`);
        } else {
            console.log(`${fileName} exists, so setting metadata`);
            let metaData = getMetadata(cacheInBrowser); 

            if(hasWasmExtension(fileName)) {
                console.log(`${fileName} is wasm, so changing contentType`);
                metaData.contentType = 'application/wasm';
            }
            await storage.bucket(obj.bucket).file(obj.name).setMetadata(metaData);
        }
        console.log(`making purge request for ${completeObjectUrl}`);

        const resp = await fetch(completeObjectUrl, { method: 'PURGE'});

        if (!resp.ok) {
            throw new Error('Unexpected status ' + resp.status);
        }
         
        const data = await resp.json();

        console.log(`Purged ${fileName}, ID ${data.id}`);
    } catch(err) {
            console.error("got error in purge!");
            console.error(err);
    }
};

exports.purgeDocs = makePurger(CONFIG.URL_DOCS, false);
exports.purgeMedia = makePurger(CONFIG.URL_MEDIA, true);

exports.purgeFrontendRelease = makePurger(CONFIG.URL_FRONTEND_RELEASE, false);
exports.purgeStorybookRelease = makePurger(CONFIG.URL_STORYBOOK_RELEASE, false);

exports.purgeFrontendSandbox = makePurger(CONFIG.URL_FRONTEND_SANDBOX, false);
exports.purgeStorybookSandbox = makePurger(CONFIG.URL_STORYBOOK_SANDBOX, false);
