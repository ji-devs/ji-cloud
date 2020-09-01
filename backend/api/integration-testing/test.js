const { default: test } = require('ava');

const got = require('got');
const getPort = require('get-port');
const { mkdir, copyFile } = require('fs').promises;
const path = require('path');
const tough = require('tough-cookie');
const spawnAsync = require('@expo/spawn-async');
const qs = require('qs');

// to whom it might concern, this JWT is made of the following header, payload:
// {"alg": "HS256", "typ": "JWT"}
// {"sub": "SGkgdGhpcyBpcyBhIHRlc3QgdG9rZW4K", "iat": 1597096685, "auth_time": 1597096686 }
// The secret used is `aaaaa`
const TEST_JWT = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJTR2tnZEdocGN5QnBjeUJoSUhSbGMzUWdkRzlyWlc0SyIsImlhdCI6MTU5NzA5NjY4NSwiYXV0aF90aW1lIjoxNTk3MDk2Njg2fQ.BNpCIBuNq0bhgXuAEqrAfPpIein0Y54hj352d2ke1sI';

// login cookie
const COOKIE = 'X-JWT=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.'
    + 'eyJpZCI6IjFmMjQxZTFiLWI1MzctNDkzZi1hMjMwLTA3NWNiMTYzMTViZSIsImNzcmYiOiJSdVF1WmI1QW9HU2R4SUdBIn0.osvyaIW4Mt-3Em4kkuvO4wXAsCVA9gZwkqXlQvQETAs; '
    + 'Max-Age=1209600; Path=/v1; HttpOnly; SameSite=Lax; hostOnly=true; aAge=2ms; cAge=6ms; hostOnly=true; aAge=1ms; cAge=1ms';

// login csrf
const CSRF = 'RuQuZb5AoGSdxIGA';

const fixtures = {
    user: 'fixtures/1_user.sql',
    imageMetaKinds: 'fixtures/2_image_meta_kinds.sql',
    categoryOrdering: 'fixtures/3_category_ordering.sql',
    categoryNesting: 'fixtures/4_category_nesting.sql',
    image: 'fixtures/5_image.sql',
};

function hookServerStarted(server) {
    return new Promise((resolve) => {
        server.stdout.on('data', (data) => {
            if (data.toString().includes('Starting "actix-web-service-')) {
                resolve();
            }
        });
    });
}

async function runFixtures(files, dbUrl, parentDir) {
    const args = [dbUrl];

    // eslint-disable-next-line no-restricted-syntax
    for (const file of files) {
        args.push('-f', file);
    }

    await spawnAsync('/usr/bin/psql', args, { cwd: parentDir, env: { PGUSER: 'postgres' }, encoding: 'utf8' });
}

async function login() {
    const cookieJar = new tough.CookieJar();
    await cookieJar.setCookie(COOKIE, 'http://0.0.0.0/v1/login');

    return {
        cookieJar,
        headers: {
            'X-CSRF': CSRF,
        },
    };
}

test.before(async (t) => {
    t.context.parentDir = path.resolve(process.cwd(), '..');

    try {
        await mkdir('bin');
    } catch (e) {
        if (e && e.code === 'EEXIST') {
            // ignore it.
        } else {
            throw e;
        }
    }
    await copyFile('../target/debug/ji-cloud-api', 'bin/ji-cloud-api');
});

test.beforeEach(async (t) => {
    let port = getPort();

    const dbUrl = await spawnAsync('../../script/ephemeralpg/pg_tmp.sh', { encoding: 'utf8' }).then((it) => it.stdout);

    port = await port;

    t.context.dbUrl = dbUrl;

    const env = {
        cwd: t.context.parentDir,
        LOCAL_API_PORT: port,
        DATABASE_URL: dbUrl,
        PGUSER: 'postgres',
        JWT_SECRET: 'abc123',
        LOCAL_PAGES_PORT: 0,
        LOCAL_NO_FIREBASE_AUTH: true,
        S3_LOCAL_DISABLE_CLIENT: true,
        DISABLE_GOOGLE_CLOUD: true,
        PROJECT_ID: "",
        ALGOLIA_APPLICATION_ID: "",
        ALGOLIA_KEY: "",
        ALGOLIA_LOCAL_DISABLE_CLIENT: true,
    };

    t.context.port = port;
    t.context.server = spawnAsync('bin/ji-cloud-api', { env, encoding: 'utf8' });
    t.context.loggedInReqBase = {
        ...await login(),
        port,
        responseType: 'json',
    };

    await hookServerStarted(t.context.server.child);

    t.context.server.child.stderr.on('data', (data) => {
        console.warn(data.toString());
    });
});

test.afterEach.always("kill server", async (t) => {
    if (t.context.server) {
        try {
            t.context.server.child.kill('SIGKILL');
            await t.context.server;
        } catch (e) {
            if (e && e.signal === 'SIGKILL') {
                // ignore it.
            } else {
                throw e;
            }
        }
    }
});

test('pass', async (t) => {
    const e = await t.throwsAsync(got('http://0.0.0.0', { port: t.context.port }));
    t.is(e.response.statusCode, 404);
});

test('missing auth (firebase)', async (t) => {
    const e = await t.throwsAsync(got.post('http://0.0.0.0/v1/user', {
        port: t.context.port,
        // As you can see, we properly have the body,
        // so the only thing that should cause this to fail is...
        json: {
            display_name: 'test',
            email: 'test@test.test',
        },
        responseType: 'json',
        headers: {
            // ... the fact that we're skipping out on authorization
            // authorization: "Bearer " + TEST_JWT,
        },
    }));

    t.is(e.response.statusCode, 401);
});

test('register user', async (t) => {
    const cookieJar = new tough.CookieJar();

    const { body } = await got.post('http://0.0.0.0/v1/user', {
        cookieJar,
        port: t.context.port,
        json: {
            username: 'test',
            email: 'test@test.test',
            given_name: 'Bobby',
            family_name: 'Tables',
            language: 'en_US',
            locale: 'en_US',
            opt_into_edu_resources: true,
            over_18: true,
            timezone: 'US/Pacific-New'
        },
        responseType: 'json',
        headers: {
            authorization: `Bearer ${TEST_JWT}`,
        },
    });

    t.not(body.csrf, null);
});

test('login user', async (t) => {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.parentDir);

    const cookieJar = new tough.CookieJar();

    const { body } = await got.post('http://0.0.0.0/v1/login', {
        cookieJar,
        port: t.context.port,
        responseType: 'json',
        headers: {
            authorization: `Bearer ${TEST_JWT}`,
        },
    });

    t.not(body.csrf, null);
});

test('user profile', async (t) => {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.parentDir);

    const profile = await got.get('http://0.0.0.0/v1/user/me/profile', t.context.loggedInReqBase);

    t.snapshot(profile.body);
});

test('create category', async (t) => {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.parentDir);

    const category = await got.post('http://0.0.0.0/v1/category', {
        ...t.context.loggedInReqBase,
        json: {
            name: 'One',
        },
    });

    t.deepEqual(typeof (category.body.id), 'string');
    t.deepEqual(typeof (category.body.index), 'number');
})

test.todo('delete category');
test.todo('update category');

test('get categories', async (t) => {
    await runFixtures([fixtures.user, fixtures.categoryOrdering], t.context.dbUrl, t.context.parentDir);

    const categories = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    t.snapshot(categories.body);
});

async function getNestedCategories(t, options) {
    await runFixtures([fixtures.user, fixtures.categoryNesting], t.context.dbUrl, t.context.parentDir);

    const categories = await got.get(`http://0.0.0.0/v1/category?${qs.stringify(options, { arrayFormat: 'brackets', encodeValuesOnly: true })}`, t.context.loggedInReqBase);

    t.snapshot(categories.body);
}

getNestedCategories.title = (providedTitle = '', meta) => {
    const {
        scope, ids
    } = {
        scope: null, ids: [], ...meta,
    };

    let title;

    if (providedTitle != '') {
        title = `get categories nested - ${providedTitle}`;
    } else {
        title = 'get categories nested'
    }

    return `${title} - scope=${scope || ''}, ids=${ids}`;
};

test('top level', getNestedCategories);
test('whole tree', getNestedCategories, {
    scope: "Decendants",
});

test('tree overlapping', getNestedCategories, {
    scope: "Decendants",
    ids: ['afbce03c-e90f-11ea-8281-cfde02f6b582', 'e315d3b2-e90f-11ea-8281-73cd69c14821'],
});

test('ancestors', getNestedCategories, {
    scope: "Ancestors",
    ids: ['afbce03c-e90f-11ea-8281-cfde02f6b582', 'e315d3b2-e90f-11ea-8281-73cd69c14821'],
});

test('exact', getNestedCategories, {
    ids: ['afbce03c-e90f-11ea-8281-cfde02f6b582', '01cff7d8-e910-11ea-8281-7f86c625a156'],
});

test('update category ordering', async (t) => {
    const categoryThree = '81c4796a-e883-11ea-93f0-df2484ab6b11';
    await runFixtures([fixtures.user, fixtures.categoryOrdering], t.context.dbUrl, t.context.parentDir);

    const updateResp = await got.patch(`http://0.0.0.0/v1/category/${categoryThree}`, {
        ...t.context.loggedInReqBase,
        json: {
            index: 0,
        }
    });

    t.deepEqual(updateResp.statusCode, 204);

    let categories = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    // ignore updated at (but make sure it exists, and is a string)
    categories.body.categories.forEach(it => {
        t.deepEqual(typeof (it.updated_at), 'string');
        delete it.updated_at;
    });

    t.snapshot(categories.body);

    const revertResp = await got.patch(`http://0.0.0.0/v1/category/${categoryThree}`, {
        ...t.context.loggedInReqBase,
        json: {
            index: 2,
        }
    });

    t.deepEqual(revertResp.statusCode, 204);

    categories = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    // ignore updated at (but make sure it exists, and is a string)
    categories.body.categories.forEach(it => {
        t.deepEqual(typeof (it.updated_at), 'string');
        delete it.updated_at;
    });

    t.snapshot(categories.body);
})

test('GET metadata', async (t) => {
    await runFixtures([fixtures.user, fixtures.imageMetaKinds], t.context.dbUrl, t.context.parentDir);

    const meta = await got.get('http://0.0.0.0/v1/image/metadata', t.context.loggedInReqBase);

    t.snapshot(meta.body);
});

async function createImage(t, meta) {
    await runFixtures([fixtures.user, fixtures.imageMetaKinds], t.context.dbUrl, t.context.parentDir);

    const image = await got.post('http://0.0.0.0/v1/image', {
        ...t.context.loggedInReqBase,
        json: {
            name: 'test',
            description: 'testest',
            is_premium: false,
            publish_at: null,
            styles: [],
            age_ranges: [],
            affiliations: [],
            categories: [],
            ...meta,
        },
    });

    t.deepEqual(typeof (image.body.id), 'string');
    t.deepEqual(typeof (image.body.upload_url), 'string');
}

createImage.title = (providedTitle = 'create image meta', meta) => {
    const {
        // eslint-disable-next-line camelcase
        styles, age_ranges, affiliations, categories,
    } = {
        styles: [], age_ranges: [], affiliations: [], categories: [], ...meta,
    };
    return `${providedTitle} +${styles.length} styles, +${age_ranges.length} age_ranges, +${affiliations.length} affiliations, +${categories.length} categories`;
};

async function createImageError(t, args) {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.parentDir);
    const error = await t.throwsAsync(got.post('http://0.0.0.0/v1/image', {
        ...t.context.loggedInReqBase,
        json: {
            name: 'test',
            description: 'testest',
            is_premium: false,
            publish_at: null,
            styles: [],
            age_ranges: [],
            affiliations: [],
            categories: [],
            [args.kind]: [args.id],
        },
    }));

    t.deepEqual(error.response.statusCode, 422);

    const { MissingMetadata } = error.response.body;

    t.deepEqual(MissingMetadata.id, args.id);
    t.deepEqual(MissingMetadata.kind, args.kindName);
}

createImageError.title = (providedTitle = 'create image error', args) => `${providedTitle} (${args.kindName})`;

test(createImage);
test(createImage, {
    styles: ['6389eaa0-de76-11ea-b7ab-0399bcf84df2'],
    affiliations: ['c0cd4446-de76-11ea-b7ab-93987e8aa112'],
    age_ranges: ['f3722790-de76-11ea-b7ab-77b45e9af3ef'],
});

test(createImageError, { kind: 'styles', kindName: 'Style', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });
test(createImageError, { kind: 'affiliations', kindName: 'Affiliation', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });
test(createImageError, { kind: 'age_ranges', kindName: 'AgeRange', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });
test(createImageError, { kind: 'categories', kindName: 'Category', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });

test.todo('GET image');
test.todo('GET images');
test.todo('DELETE image');

// todo: test builder
test('update image - empty', async t => {
    await runFixtures([fixtures.user, fixtures.imageMetaKinds, fixtures.image], t.context.dbUrl, t.context.parentDir);

    await got.patch('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);

    const resp = await got.get('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);

    t.snapshot(resp.body.metadata);
});

test('update image - is_premium', async t => {
    await runFixtures([fixtures.user, fixtures.imageMetaKinds, fixtures.image], t.context.dbUrl, t.context.parentDir);

    await t.notThrowsAsync(got.patch('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', { ...t.context.loggedInReqBase, json: { is_premium: true } }));

    const resp = await got.get('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);
    const metadata = resp.body.metadata;

    // can't snapshot update timestamps for obvious reasons.
    t.deepEqual(typeof (metadata.updated_at), 'string');
    delete metadata.updated_at;

    t.snapshot(metadata);
});
