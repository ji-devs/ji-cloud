const fetch = require('node-fetch');

const makePurger = (FASTLY_PUBLIC_BASEURL) => async (obj, context) => {
  const baseUrl = FASTLY_PUBLIC_BASEURL.replace(/\/+$/, '');
  const fileName = obj.name.replace(/^\/+/, '');
  const completeObjectUrl = `${baseUrl}/${fileName}`;

  const resp = await fetch(completeObjectUrl, { method: 'PURGE'})
  if (!resp.ok) throw new Error('Unexpected status ' + resp.status);

  const data = await resp.json();
  console.log(`Job complete for ${fileName}, purge ID ${data.id}`);
};

exports.purgeDocs = makePurger("https://docs.jicloud.org");
exports.purgeFrontendRelease = makePurger("https://frontend.jicloud.org");
exports.purgeStorybookRelease = makePurger("https://storybook.jicloud.org");


exports.purgeFrontendSandbox = makePurger("https://frontend.sandbox.jicloud.org");
exports.purgeStorybookSandbox = makePurger("https://storybook.sandbox.jicloud.org");