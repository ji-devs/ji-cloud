const fetch = require('node-fetch');
const {Storage} = require('@google-cloud/storage');
const storage = new Storage();

const makePurger = (FASTLY_PUBLIC_BASEURL) => async (obj, context) => {
  const baseUrl = FASTLY_PUBLIC_BASEURL.replace(/\/+$/, '');
  const fileName = obj.name.replace(/^\/+/, '');
  const completeObjectUrl = `${baseUrl}/${fileName}`;

  storage.bucket(obj.bucket).file(obj.name).setMetadata({
    cacheControl: 'max-age=0, s-maxage=86400',
  })
  .then(() => fetch(completeObjectUrl, { method: 'PURGE'}))
  .then(resp => {
    if (!resp.ok) throw new Error('Unexpected status ' + resp.status);
  })
  .catch(err => {
    console.error("got error in purge!");
    console.error(err);
  });

};

exports.purgeDocs = makePurger("https://docs.jicloud.org");
exports.purgeFrontendRelease = makePurger("https://frontend.jicloud.org");
exports.purgeStorybookRelease = makePurger("https://storybook.jicloud.org");


exports.purgeFrontendSandbox = makePurger("https://frontend.sandbox.jicloud.org");
exports.purgeStorybookSandbox = makePurger("https://storybook.sandbox.jicloud.org");