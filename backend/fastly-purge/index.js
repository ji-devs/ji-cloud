const fetch = require('node-fetch');
const {Storage} = require('@google-cloud/storage');
const storage = new Storage();

const makePurger = (FASTLY_PUBLIC_BASEURL) => async (obj, context) => {
  const baseUrl = FASTLY_PUBLIC_BASEURL.replace(/\/+$/, '');
  const fileName = obj.name.replace(/^\/+/, '');
  const completeObjectUrl = `${baseUrl}/${fileName}`;

  const resp = await fetch(completeObjectUrl, { method: 'PURGE'})
  if (!resp.ok) throw new Error('Unexpected status ' + resp.status);
  //const data = await resp.json();

  const metaResp = await storage.bucket(obj.bucket).file(obj.name).setMetadata({
    cacheControl: 'Cache-Control: max-age=0, s-maxage=86400',
  });
};

exports.purgeDocs = makePurger("https://docs.jicloud.org");
exports.purgeFrontendRelease = makePurger("https://frontend.jicloud.org");
exports.purgeStorybookRelease = makePurger("https://storybook.jicloud.org");


exports.purgeFrontendSandbox = makePurger("https://frontend.sandbox.jicloud.org");
exports.purgeStorybookSandbox = makePurger("https://storybook.sandbox.jicloud.org");