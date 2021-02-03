const { default: test } = require('ava');

const got = require('got');
const getPort = require('get-port');
const path = require('path');
const tough = require('tough-cookie');
const spawnAsync = require('@expo/spawn-async');
const qs = require('qs');

// to whom it might concern, this JWT is made of the following header, payload:
// {"alg": "HS256", "typ": "JWT"}
// {"sub": "SGkgdGhpcyBpcyBhIHRlc3QgdG9rZW4K", "iat": 1597096685, "auth_time": 1597096686 }
// The secret used is `aaaaa`
const TEST_JWT = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJTR2tnZEdocGN5QnBjeUJoSUhSbGMzUWdkRzlyWlc0SyIsImlhdCI6MTU5NzA5NjY4NSwiYXV0aF90aW1lIjoxNTk3MDk2Njg2fQ.BNpCIBuNq0bhgXuAEqrAfPpIein0Y54hj352d2ke1sI';

// this jwt is used for register *erroring*
// {"alg": "HS256", "typ": "JWT"}
// {"sub": "JmaZTj2b5X9ksf5oFSXnCdVuLJkMROCS", "iat": 1597096685, "auth_time": 1597096686 }
// The secret used is `aaaaa`
const REGISTER_ERR_JWT = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJKbWFaVGoyYjVYOWtzZjVvRlNYbkNkVnVMSmtNUk9DUyIsImlhdCI6MTU5NzA5NjY4NSwiYXV0aF90aW1lIjoxNTk3MDk2Njg2fQ.fxdYc2Dhr6RVrWFkh8xh-ROz6fYKI9yW7WKJuaEfdqs';

// login cookie
const COOKIE = 'X-JWT=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.'
    + 'eyJpZCI6IjFmMjQxZTFiLWI1MzctNDkzZi1hMjMwLTA3NWNiMTYzMTViZSIsImNzcmYiOiJSdVF1WmI1QW9HU2R4SUdBIn0.osvyaIW4Mt-3Em4kkuvO4wXAsCVA9gZwkqXlQvQETAs; '
    + 'Max-Age=1209600; Path=/v1; HttpOnly; SameSite=Lax; hostOnly=true; aAge=2ms; cAge=6ms; hostOnly=true; aAge=1ms; cAge=1ms';

// login csrf
const CSRF = 'RuQuZb5AoGSdxIGA';

const fixtures = {
    user: '1_user.sql',
    metaKinds: '2_meta_kinds.sql',
    categoryOrdering: '3_category_ordering.sql',
    categoryNesting: '4_category_nesting.sql',
    image: '5_image.sql',
    userNoPerms: '6_user_no_perms.sql',
};

const DB_NAMES = new Set();

function createDbName(length) {
    length = length || 16;

    const generateDbName = () => {
        let name = '';
        while (name.length < length) {
            name += Math.random().toString(36).replace(/[^a-z]+/g, '');
            name = name.slice(0, length);
        }
        return name;
    };

    let name = '';

    do {
        name = generateDbName();
    } while (DB_NAMES.has(name));

    return name;
}

function hookServerStarted(server) {
    return new Promise((resolve) => {
        server.stderr.on('data', (data) => {
            if (data.toString().includes('Starting "actix-web-service-')) {
                resolve();
            }
        });
    });
}

async function runFixtures(files, dbUrl, dir) {
    const args = [dbUrl];

    // eslint-disable-next-line no-restricted-syntax
    for (const file of files) {
        args.push('-f', path.resolve(dir, file));
    }

    await spawnAsync('/usr/bin/psql', args, { env: { PGUSER: 'postgres' }, encoding: 'utf8' });
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
    t.context.BIN_FILE = process.env.BIN_FILE || '../target/debug/ji-cloud-api';
    t.context.FIXTURES_DIR = process.env.FIXTURES_DIR || path.resolve(t.context.parentDir, 'fixtures');
    t.context.isNative = process.env.USE_PG_TMP || false;
    if (t.context.isNative) {
        t.context.PG_TMP = process.env.PG_TMP || '../../script/ephemeralpg/pg_tmp.sh';
        t.context.baseDbUrl = await spawnAsync(t.context.PG_TMP, { encoding: 'utf8' }).then((it) => it.stdout);
        t.context.getDbUrl = (name) => t.context.baseDbUrl.replace('test', name);
    } else {
        t.context.baseDbUrl = process.env.DATABASE_URL;
        t.context.getDbUrl = (name) => `${t.context.baseDbUrl}/${name}`;
    }

});

test.beforeEach(async (t) => {
    let port = getPort();
    t.context.dbName = createDbName();
    t.context.dbUrl = t.context.getDbUrl(t.context.dbName);

    await spawnAsync('/usr/bin/psql', [t.context.baseDbUrl, '-U', 'postgres', '-c', `create database "${t.context.dbName}"`], { encoding: 'utf8', env: { PGPASSWORD: 'password' } });

    port = await port;

    const env = {
        LOCAL_API_PORT: port,
        DATABASE_URL: t.context.dbUrl,
        PGUSER: 'postgres',
        JWT_SECRET: 'abc123',
        LOCAL_PAGES_PORT: 0,
        LOCAL_NO_FIREBASE_AUTH: true,
        S3_LOCAL_DISABLE_CLIENT: true,
        S3_ENDPOINT: '',
        S3_BUCKET: '',
        GOOGLE_S3_ACCESS_KEY: '',
        GOOGLE_S3_ACCESS_SECRET: '',
        DISABLE_GOOGLE_CLOUD: true,
        PROJECT_ID: '',
        ALGOLIA_PROJECT_ID: '',
        ALGOLIA_KEY: '',
        ALGOLIA_LOCAL_DISABLE_CLIENT: true,
        RUST_LOG: "warning,actix_server::builder=info",
    };

    t.context.port = port;
    t.context.server = spawnAsync(t.context.BIN_FILE, { env, encoding: 'utf8' });

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

test.afterEach.always('kill server', async (t) => {
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
            timezone: 'US/Pacific-New',
            organization: 'test organization',
        },
        responseType: 'json',
        headers: {
            authorization: `Bearer ${TEST_JWT}`,
        },
    });

    t.not(body.csrf, null);
});

async function registerDuplicateUserError(t, args) {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.FIXTURES_DIR);

    const cookieJar = new tough.CookieJar();

    const error = await t.throwsAsync(got.post('http://0.0.0.0/v1/user', {
        cookieJar,
        port: t.context.port,
        json: {
            username: 'test2',
            email: 'test2@test.test',
            given_name: 'Bobby',
            family_name: 'Tables',
            language: 'en_US',
            locale: 'en_US',
            opt_into_edu_resources: true,
            over_18: true,
            timezone: 'US/Pacific-New',
            organization: 'test organization',
            [args.key]: args.value,
        },
        responseType: 'json',
        headers: {
            authorization: `Bearer ${args.jwt}`,
        },
    }));

    t.is(error.response.statusCode, 422);
    t.snapshot(error.response.body);
}

registerDuplicateUserError.title = (providedTitle = 'register duplicate user error', args) => `${providedTitle} (${args.key !== '' ? args.key : 'id'})`;

test(registerDuplicateUserError, { jwt: TEST_JWT, key: '', value: '' });
test(registerDuplicateUserError, { jwt: REGISTER_ERR_JWT, key: 'username', value: 'test' });
test(registerDuplicateUserError, { jwt: REGISTER_ERR_JWT, key: 'email', value: 'test@test.test' });

test('login user', async (t) => {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.FIXTURES_DIR);

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
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.FIXTURES_DIR);

    const profile = await got.get('http://0.0.0.0/v1/user/me/profile', t.context.loggedInReqBase);

    t.snapshot(profile.body);
});

test('create category', async (t) => {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.FIXTURES_DIR);

    const category = await got.post('http://0.0.0.0/v1/category', {
        ...t.context.loggedInReqBase,
        json: {
            name: 'One',
        },
    });

    t.deepEqual(typeof (category.body.id), 'string');
    t.deepEqual(typeof (category.body.index), 'number');
});

function assertCategoryUpdatedAt(t, categories) {
    categories.forEach((it) => {
        if (it.children) {
            assertCategoryUpdatedAt(t, it.children);
        }

        t.true(it.updated_at === null || typeof (it.updated_at) === 'string');
        delete it.updated_at;
    });
}

test('delete category', async (t) => {
    await runFixtures([fixtures.user, fixtures.categoryOrdering], t.context.dbUrl, t.context.FIXTURES_DIR);
    await t.notThrowsAsync(got.delete(
        'http://0.0.0.0/v1/category/7fe19326-e883-11ea-93f0-5343493c17c4',
        t.context.loggedInReqBase,
    ));

    const { body: resp } = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    assertCategoryUpdatedAt(t, resp.categories);

    t.snapshot(resp);
});

async function updateCategoryFactory(t, args) {
    await runFixtures([fixtures.user, fixtures.categoryOrdering], t.context.dbUrl, t.context.FIXTURES_DIR);

    await t.notThrowsAsync(got.patch(`http://0.0.0.0/v1/category/${args.category}`, {
        ...t.context.loggedInReqBase,
        json: args.json,
    }));

    const { body: resp } = await got.get('http://0.0.0.0/v1/category?scope=Decendants', t.context.loggedInReqBase);

    assertCategoryUpdatedAt(t, resp.categories);

    t.snapshot(resp);
}

updateCategoryFactory.title = (providedTitle = 'update category', args) => `${providedTitle} - (id: ${args.category}, ${JSON.stringify(args.json)})`;

// todo: combine the following several tests into a test factory.

test(updateCategoryFactory, { category: '7fe19326-e883-11ea-93f0-5343493c17c4', json: { parent_id: '81c4796a-e883-11ea-93f0-df2484ab6b11' } });
test(updateCategoryFactory, { category: '7fe19326-e883-11ea-93f0-5343493c17c4', json: { parent_id: null, index: 0 } });
test(updateCategoryFactory, { category: '81c4796a-e883-11ea-93f0-df2484ab6b11', json: { index: 1 } });
test(updateCategoryFactory, { category: '81c4796a-e883-11ea-93f0-df2484ab6b11', json: { name: 'abc123' } });

test('get categories', async (t) => {
    await runFixtures([fixtures.user, fixtures.categoryOrdering], t.context.dbUrl, t.context.FIXTURES_DIR);

    const categories = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    t.snapshot(categories.body);
});

async function getNestedCategories(t, options) {
    await runFixtures([fixtures.user, fixtures.categoryNesting], t.context.dbUrl, t.context.FIXTURES_DIR);

    const categories = await got.get(`http://0.0.0.0/v1/category?${qs.stringify(options, { arrayFormat: 'comma', encodeValuesOnly: true })}`, t.context.loggedInReqBase);

    t.snapshot(categories.body);
}

getNestedCategories.title = (providedTitle = '', meta) => {
    const {
        scope, ids,
    } = {
        scope: null, ids: [], ...meta,
    };

    let title;

    if (providedTitle !== '') {
        title = `get categories nested - ${providedTitle}`;
    } else {
        title = 'get categories nested';
    }

    return `${title} - scope=${scope || ''}, ids=${ids}`;
};

test('top level', getNestedCategories);
test('whole tree', getNestedCategories, {
    scope: 'Decendants',
});

test('tree overlapping', getNestedCategories, {
    scope: 'Decendants',
    ids: ['afbce03c-e90f-11ea-8281-cfde02f6b582', 'e315d3b2-e90f-11ea-8281-73cd69c14821'],
});

test('ancestors', getNestedCategories, {
    scope: 'Ancestors',
    ids: ['afbce03c-e90f-11ea-8281-cfde02f6b582', 'e315d3b2-e90f-11ea-8281-73cd69c14821'],
});

test('exact', getNestedCategories, {
    ids: ['afbce03c-e90f-11ea-8281-cfde02f6b582', '01cff7d8-e910-11ea-8281-7f86c625a156'],
});

test('update category ordering', async (t) => {
    const categoryThree = '81c4796a-e883-11ea-93f0-df2484ab6b11';
    await runFixtures([fixtures.user, fixtures.categoryOrdering], t.context.dbUrl, t.context.FIXTURES_DIR);

    const updateResp = await got.patch(`http://0.0.0.0/v1/category/${categoryThree}`, {
        ...t.context.loggedInReqBase,
        json: {
            index: 0,
        },
    });

    t.deepEqual(updateResp.statusCode, 204);

    let categories = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    // ignore updated at (but make sure it exists, and is a string)
    categories.body.categories.forEach((it) => {
        t.deepEqual(typeof (it.updated_at), 'string');
        delete it.updated_at;
    });

    t.snapshot(categories.body);

    const revertResp = await got.patch(`http://0.0.0.0/v1/category/${categoryThree}`, {
        ...t.context.loggedInReqBase,
        json: {
            index: 2,
        },
    });

    t.deepEqual(revertResp.statusCode, 204);

    categories = await got.get('http://0.0.0.0/v1/category', t.context.loggedInReqBase);

    // ignore updated at (but make sure it exists, and is a string)
    categories.body.categories.forEach((it) => {
        t.deepEqual(typeof (it.updated_at), 'string');
        delete it.updated_at;
    });

    t.snapshot(categories.body);
});

test('GET metadata', async (t) => {
    await runFixtures([fixtures.user, fixtures.metaKinds], t.context.dbUrl, t.context.FIXTURES_DIR);

    const meta = await got.get('http://0.0.0.0/v1/metadata', t.context.loggedInReqBase);

    t.snapshot(meta.body);
});

async function createImage(t, meta) {
    await runFixtures([fixtures.user, fixtures.metaKinds], t.context.dbUrl, t.context.FIXTURES_DIR);

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
            kind: 'Canvas',
            ...meta,
        },
    });

    t.deepEqual(typeof (image.body.id), 'string');
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
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.FIXTURES_DIR);
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
            kind: 'Canvas',
            [args.kind]: [args.id],
        },
    }));

    t.deepEqual(error.response.statusCode, 422);

    t.deepEqual(error.response.body.id, args.id);
    t.deepEqual(error.response.body.kind, args.kindName);
}

createImageError.title = (providedTitle = 'create image error', args) => `${providedTitle} (${args.kindName})`;

test(createImage);
test(createImage, { styles: ['6389eaa0-de76-11ea-b7ab-0399bcf84df2', '6389ff7c-de76-11ea-b7ab-9b5661dd4f70'] });
test(createImage, {
    styles: ['6389eaa0-de76-11ea-b7ab-0399bcf84df2'],
    affiliations: ['c0cd4446-de76-11ea-b7ab-93987e8aa112'],
    age_ranges: ['f3722790-de76-11ea-b7ab-77b45e9af3ef'],
});

test(createImageError, { kind: 'styles', kindName: 'Style', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });
test(createImageError, { kind: 'affiliations', kindName: 'Affiliation', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });
test(createImageError, { kind: 'age_ranges', kindName: 'AgeRange', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });
test(createImageError, { kind: 'categories', kindName: 'Category', id: '6389eaa0-de76-11ea-b7ab-0399bcf84df2' });

test('GET image', async (t) => {
    await runFixtures([fixtures.user, fixtures.metaKinds, fixtures.image], t.context.dbUrl, t.context.FIXTURES_DIR);

    const resp = await got.get('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);

    t.snapshot(resp.body.metadata);
});

test.todo('GET images');

test('DELETE image', async (t) => {
    await runFixtures([fixtures.user, fixtures.metaKinds, fixtures.image], t.context.dbUrl, t.context.FIXTURES_DIR);

    await t.notThrowsAsync(got.delete('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase));
});

test.todo('PATCH image/raw (upload image)');

// todo: test builder
test('update image - empty', async (t) => {
    await runFixtures([fixtures.user, fixtures.metaKinds, fixtures.image], t.context.dbUrl, t.context.FIXTURES_DIR);

    await got.patch('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);

    const resp = await got.get('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);

    t.snapshot(resp.body.metadata);
});

test('update image - is_premium', async (t) => {
    await runFixtures([fixtures.user, fixtures.metaKinds, fixtures.image], t.context.dbUrl, t.context.FIXTURES_DIR);

    await t.notThrowsAsync(got.patch('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', { ...t.context.loggedInReqBase, json: { is_premium: true } }));

    const resp = await got.get('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);
    const { metadata } = resp.body;

    // can't snapshot update timestamps for obvious reasons.
    t.deepEqual(typeof (metadata.updated_at), 'string');
    delete metadata.updated_at;

    t.snapshot(metadata);
});

test('update image - two styles', async (t) => {
    await runFixtures([fixtures.user, fixtures.metaKinds, fixtures.image], t.context.dbUrl, t.context.FIXTURES_DIR);

    await t.notThrowsAsync(got.patch('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', { ...t.context.loggedInReqBase, json: { styles: ['6389eaa0-de76-11ea-b7ab-0399bcf84df2', '6389ff7c-de76-11ea-b7ab-9b5661dd4f70'] } }));

    const resp = await got.get('http://0.0.0.0/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f', t.context.loggedInReqBase);
    const { metadata } = resp.body;

    // can't snapshot update timestamps for obvious reasons.
    // t.deepEqual(typeof (metadata.updated_at), 'string');
    delete metadata.updated_at;

    t.snapshot(metadata);
});

// 500s, but for some reason diagnosis is being difficult 
test.skip('create jig - default', async (t) => {
    await runFixtures([fixtures.user], t.context.dbUrl, t.context.FIXTURES_DIR);

    const jig = await got.post('http://0.0.0.0/v1/jig', {
        ...t.context.loggedInReqBase,
    });

    t.deepEqual(typeof (jig.body.id), 'string');
});

test.todo("create jig - params");
test.todo("delete jig");
test.todo("get jig");
test.todo("update jig");

async function authFail(t, data) {
    await runFixtures([fixtures.userNoPerms], t.context.dbUrl, t.context.FIXTURES_DIR);

    let base = t.context.loggedInReqBase;
    if (data.body.method === 'GET') {
        base = { ...base, headers: {} };
    }

    const response = await got(data.route, {
        ...base,
        prefixUrl: 'http://0.0.0.0/',
        ...data.body,
        throwHttpErrors: false,
    });

    if (data.body.method === 'GET') {
        t.deepEqual(response.statusCode, 401);
    } else {
        t.deepEqual(response.statusCode, 403);
    }
}

authFail.title = (providedTitle = 'authfail', data) => {
    let actionName;

    switch (data.body.method) {
        case 'GET': actionName = 'get'; break;
        case 'PATCH': actionName = 'update'; break;
        case 'DELETE': actionName = 'delete'; break;
        case 'POST': actionName = 'create'; break;
        default: actionName = '?';
    }

    return `${providedTitle} - ${data.kind} - ${actionName}`;
};

test(authFail, {
    kind: 'category',
    route: 'v1/category',
    body: {
        method: 'POST',
        json: {
            name: ''
        },
    }
});

test(authFail, {
    kind: 'category',
    route: 'v1/category/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'PATCH',
    }
});

test(authFail, {
    kind: 'category',
    route: 'v1/category?ids=00000000-0000-0000-0000-000000000000',
    body: {
        method: 'GET',
    }
});

test(authFail, {
    kind: 'category',
    route: 'v1/category/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'DELETE',
    }
});

test(authFail, {
    kind: 'image',
    route: 'v1/image',
    body: {
        method: 'POST',
        json: {
            name: 'test',
            description: 'testest',
            is_premium: false,
            publish_at: null,
            styles: [],
            age_ranges: [],
            affiliations: [],
            categories: [],
            kind: 'Canvas',
        },
    }
});

test(authFail, {
    kind: 'image',
    route: 'v1/image/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'PATCH',
    }
});

test(authFail, {
    kind: 'image',
    route: 'v1/image/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'GET',
    }
});

test(authFail, {
    kind: 'image',
    route: 'v1/image/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'DELETE',
    }
});

test(authFail, {
    kind: 'jig',
    route: 'v1/jig',
    body: {
        method: 'POST',
    }
});

test(authFail, {
    kind: 'jig',
    route: 'v1/jig/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'PATCH',
    }
});

test(authFail, {
    kind: 'jig',
    route: 'v1/jig/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'GET',
    }
});


test(authFail, {
    kind: 'jig',
    route: 'v1/jig/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'DELETE',
    }
});

test(authFail, {
    kind: 'module',
    route: 'v1/module',
    body: {
        method: 'POST',
    }
});

test(authFail, {
    kind: 'module',
    route: 'v1/module/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'PATCH',
    }
});

test(authFail, {
    kind: 'module',
    route: 'v1/module/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'GET',
    }
});

test(authFail, {
    kind: 'module',
    route: 'v1/module/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'DELETE',
    }
});


test(authFail, {
    kind: 'animation',
    route: 'v1/animation',
    body: {
        method: 'POST',
        json: {
            name: 'test',
            description: 'testest',
            is_premium: false,
            is_looping: false,
            publish_at: null,
            variant: 'Gif',
        },
    }
});

// route doesn't exist yet.
test.skip(authFail, {
    kind: 'animation',
    route: 'v1/animation/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'PATCH',
    }
});

test(authFail, {
    kind: 'animation',
    route: 'v1/animation/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'GET',
    }
});

test(authFail, {
    kind: 'animation',
    route: 'v1/animation/00000000-0000-0000-0000-000000000000',
    body: {
        method: 'DELETE',
    }
});
