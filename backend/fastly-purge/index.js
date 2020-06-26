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
    console.log(`Purged ${fileName}, ID ${data.id}`);
  })
  .catch(err => {
    console.error("got error in purge!");
    console.error(err);
  });

};

//is actually on sandbox (since it's for devs)
exports.purgeDocs = makePurger("https://docs.jicloud.org");

//is actually on release (since it's for public)
exports.purgeMedia = makePurger("https://media.jicloud.org");

exports.purgeFrontendRelease = makePurger("https://frontend.jicloud.org");
exports.purgeStorybookRelease = makePurger("https://storybook.jicloud.org");

exports.purgeFrontendSandbox = makePurger("https://frontend.sandbox.jicloud.org");
exports.purgeStorybookSandbox = makePurger("https://storybook.sandbox.jicloud.org");