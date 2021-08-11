const fetch = require('node-fetch');
const {Storage} = require('@google-cloud/storage');
const {CloudTasksClient} = require('@google-cloud/tasks');
const storage = new Storage();
const puppeteer = require("puppeteer");
const _gm = require("gm");
const gm = _gm.subClass({imageMagick: true});
const validateUuid = require('uuid-validate');

const BROWSER_WIDTH = 1024;
const BROWSER_HEIGHT = 576;

const THUMB_HEIGHT = 168;
const THUMB_WIDTH = BROWSER_WIDTH * (THUMB_HEIGHT / BROWSER_HEIGHT);

require('dotenv').config();

//TEST URL: http://localhost:8081/?jig=d46ca2d2-eeef-11eb-8c76-77f818ce2b73&module=d9f674c6-eeef-11eb-b38c-e75f7ef16f01&kind=tapping-board

exports.showScreenshotRelease = makeShowScreenshot("https://jigzi.org");
exports.showScreenshotSandbox = makeShowScreenshot("https://sandbox.jigzi.org");
exports.saveScreenshotRelease = makeSaveScreenshot("https://jigzi.org", "ji-cloud-uploads-origin-eu-001");
exports.saveScreenshotSandbox = makeSaveScreenshot("https://sandbox.jigzi.org", "ji-cloud-sandbox-uploads-origin-eu-001");
exports.queueScreenshotRelease = queueScreenshot("us-central1", "ji-cloud", "https://europe-west1-ji-cloud.cloudfunctions.net", "saveScreenshotRelease", "https://uploads.jicloud.org");
exports.queueScreenshotSandbox = queueScreenshot("europe-west1", "ji-cloud-developer-sandbox", "https://europe-west1-ji-cloud-developer-sandbox.cloudfunctions.net", "saveScreenshotSandbox", "https://uploads.sandbox.jicloud.org");

/*** Factory functions for release vs. sandbox ***/

let _tasksClient;

//The task location unfortunately MUST be in the app-engine region, and the app-engine region can't be changed
//We aren't even using an app engine instance at this point... not sure why the release has it set to us-central1
//But it's only for queing tasks, the actual heavy lifting of generating and writing the screenshot is all in europe-west1
//i.e. same region as storage (and cloud run etc.)
//So no biggie, just a slight inconvenience
function queueScreenshot(location, project, baseUrl, endpoint, finalUrl) {
    return wrapCors((req, res) => {
        const {respondError, respondJson} = makeResponders(res);

        return parseQuery(req.query)
            .then(({jig, module, kind}) => {
                if(_tasksClient == undefined) {
                    _tasksClient = new CloudTasksClient();
                }

                const client = _tasksClient;

                const QUEUE = "screenshot";
                const parent = client.queuePath(project, location, QUEUE);
                
                const url = `${baseUrl}/${endpoint}?jig=${jig}&module=${module}&kind=${kind}`;

                const task = {
                    httpRequest: {
                        httpMethod: 'POST',
                        url 
                    },
                };

                //task.httpRequest.body = Buffer.from(payload).toString('base64');
                //task.scheduleTime = { seconds: inSeconds + Date.now() / 1000, };

                const request = {parent, task};
                return client.createTask(request)
                    .then(([response]) => {
                        return {
                            jpg: `${finalUrl}/screenshot/${jig}/${module}/full.jpg`,
                            taskUrl: url,
                            taskName: response.name
                        }
                    });
            })
            .then(
                resp => {
                    respondJson(resp);
                },
                err => {
                    respondError(err);
                }
            );
    });
}

function makeShowScreenshot(baseUrl) {
    return wrapCors((req, res) => {
        const {respondError, respondBuffer} = makeResponders(res);
        
        getScreenshotUrl(req, baseUrl)
            .then(url => doScreenshot(url))
            .then(
                ({fullBuffer, thumbBuffer}) => {
                    respondBuffer({contentType: "image/jpeg", data: thumbBuffer});
                },
                err => {
                    respondError(err);
                }
            )
    });
}

function makeSaveScreenshot(baseUrl, bucketName) {
    return wrapCors((req, res) => {
        const {respondError, respondJson} = makeResponders(res);
             
        getScreenshotUrl(req, baseUrl)
            .then(url => doScreenshot(url))
            .then(({fullBuffer, thumbBuffer}) => {
                return parseQuery(req.query)
                    .then(({jig, module, kind}) => {
                        const bucket = new Storage().bucket(bucketName);
                        const fullFile = bucket.file(`screenshot/${jig}/${module}/full.jpg`);
                        const thumbFile = bucket.file(`screenshot/${jig}/${module}/thumb.jpg`);

                        return writeJpegToFile({file: fullFile, data: fullBuffer})
                            .then(() => writeJpegToFile({file: thumbFile, data: thumbBuffer}));
                    })
            })
            .then(
                () => {
                    respondJson({saved: true})
                },
                err => {
                    respondError(err);
                }
            )
            .catch(err => {
            });
    });
}

/*** Heavy Lifting ***/

//Take a screenshot
function doScreenshot(url) {
    const DEBUG_BROWSER_OPEN = false;

    return puppeteer.launch({
        headless: !DEBUG_BROWSER_OPEN,
        args: ['--no-sandbox', '--disable-setuid-sandbox', '--font-render-hinting=none'],
        defaultViewport: {
            width: BROWSER_WIDTH,
            height: BROWSER_HEIGHT 
        }
    })
    .then(async browser => {

        const page = await browser.newPage();
        await page.goto(url, { waitUntil: "networkidle0" });
        //removing this seems to be okay: 
        //await page.waitFor(5000);
        await page.evaluateHandle('document.fonts.ready');
        const imageBuffer = await page.screenshot({
            type: "jpeg",
            quality: 90
        });
        if(!DEBUG_BROWSER_OPEN) {
            await browser.close();
        }
        return imageBuffer
    })
    .then(fullBuffer => {
        return gmToBuffer(gm(fullBuffer).resize(THUMB_WIDTH, THUMB_HEIGHT))
            .then(thumbBuffer => {
                return {fullBuffer, thumbBuffer}
            })
    });
}

// Cloud file writers
function writeToFileWithMeta({meta, file, data}) {
    return new Promise((resolve, reject) => {
        //NOTE: Setting resumable to true breaks things...
        file.save(data, {resumable: false}, err => {
            if(err) {
                reject(err);
            } else {
                resolve(null);
            }
        })
    })
    .then(() => new Promise((resolve, reject) => {
        file.setMetadata(meta, null, err => {
            if(err) {
                reject(err);
            } else {
                resolve(file);
            }
        })
    }))
}

function writeJsonToFile({file, data}) {
    return writeToFileWithMeta({
        meta: {contentType: "application/json"},
        file,
        data: JSON.stringify(data)
    });
}

function writeJpegToFile({file, data}) {
    return writeToFileWithMeta({
        meta: {contentType: "image/jpeg"},
        file,
        data
    });
}

function writePngToFile({file, data}) {
    return writeToFileWithMeta({
        meta: {contentType: "image/png"},
        file,
        data
    });
}

/*** utils ***/

//Convert a gm stream to buffer
function gmToBuffer(data) {
  return new Promise((resolve, reject) => {
    data.stream((err, stdout, stderr) => {
      if (err) { return reject(err) }
      const chunks = []
      stdout.on('data', (chunk) => { chunks.push(chunk) })
      // these are 'once' because they can and do fire multiple times for multiple errors,
      // but this is a promise so you'll have to deal with them one at a time
      stdout.once('end', () => { resolve(Buffer.concat(chunks)) })
      stderr.once('data', (data) => { reject(String(data)) })
    })
  });
}

function getScreenshotUrl(req, baseUrl) {
    return parseQuery(req.query)
        .then(({jig, module, kind}) => {
            return `${baseUrl}/module/${kind}/play/${jig}/${module}?screenshot=true`
        });
}
function parseQuery(query) {
    return new Promise((resolve, reject) => {
        const {jig, module, kind} = query;
        if(!jig || !module || !kind || jig == "" || module == "" || kind == "") {
            reject("not enough data!");
        } else if(!validateUuid(jig) || !validateUuid(module)) {
            reject("invalid uuid");
        } else {
            resolve({jig, module, kind})
        }
    })
}

//Allow CORS
function wrapCors(fn) {
    return (req, res) => {
      // Set CORS headers for preflight requests
      // Allows GETs from any origin with the Content-Type header
      // and caches preflight response for 3600s

      res.set('Access-Control-Allow-Origin', '*');

      if (req.method === 'OPTIONS') {
        // Send response to OPTIONS requests
        res.set('Access-Control-Allow-Methods', 'GET');
        res.set('Access-Control-Allow-Headers', 'Content-Type');
        res.set('Access-Control-Max-Age', '3600');
        res.status(204).send('');
      } else {
        return fn(req, res);
      }
    }
}

//Response handlers
function makeResponders(res) {
    return {
        respondError: error => {
            console.error(error);
            res.status(400);
            res.json({error: true}); 
            res.end();
        },

        respondJson: data => {
            res.json(data);
        },

        respondBuffer: ({contentType, data}) => { 
            res.contentType(contentType);
            res.status(200);
            res.send(data);
            res.end();
        },
        respondBufferSave: ({contentType, data, filename}) => {
            res.contentType(contentType);
            //res.setHeader('Content-Disposition', `attachment; filename="${filename}"`);
            res.setHeader('Content-Disposition', `attachment; filename="${filename}"`);
            res.status(200);
            res.send(data);
            res.end();
        }

    }
}
