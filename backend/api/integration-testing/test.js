const { default: test } = require('ava');

const got = require('got');
const getPort = require('get-port');
const { mkdir, copyFile } = require('fs').promises;
const path = require('path');
const tough = require('tough-cookie');
const spawnAsync = require('@expo/spawn-async');

function hookServerStarted(server) {
    return new Promise(resolve => {
        resolved = false;
        server.stdout.on('data', function (data) {
            if (data.toString().includes('Starting "actix-web-service-')) {
                resolve();
            }
        });
    })
}

async function runFixtures(files, dbUrl, parentDir) {
    for (let file of files) {
        await spawnAsync("/usr/bin/psql", ["-f", file, dbUrl], { cwd: parentDir, env: { PGUSER: "postgres" }, encoding: 'utf8' });
    }
}

test.before(async t => {
    t.context.parentDir = path.resolve(process.cwd(), '..');

    await spawnAsync("cargo", ["build", "--manifest-path", "../Cargo.toml"]);

    try {
        await mkdir("bin");
    }
    catch (e) {
        if (e && e.code === 'EEXIST') {
            // ignore it.
        } else {
            throw e
        }
    }
    await copyFile("../target/debug/ji-cloud-api", "bin/ji-cloud-api");
})

test.beforeEach(async t => {
    let port = getPort();

    var parentDir = t.context.parentDir;
    const dbUrl = await spawnAsync("../../script/ephemeralpg/pg_tmp.sh", { encoding: 'utf8' }).then(it => it.stdout);

    t.context.dbUrl = dbUrl;

    let id = Math.random();

    await spawnAsync("cargo", ["sqlx", "migrate", "run"], { cwd: parentDir, env: { DATABASE_URL: dbUrl, PGUSER: "postgres" }, encoding: 'utf8' });
    port = await port;

    const env = {
        LOCAL_API_PORT: port,
        DATABASE_URL: dbUrl,
        PGUSER: "postgres",
        JWT_SECRET: "abc123",
        INTER_SERVER_SECRET: "aaa",
        LOCAL_PAGES_PORT: 0,
        LOCAL_NO_FIREBASE_AUTH: true
    };

    t.context.port = port;
    t.context.server = spawnAsync("bin/ji-cloud-api", { env: env, encoding: 'utf8' });

    await hookServerStarted(t.context.server.child);
})

test.afterEach.always(async t => {
    if (t.context.server) {
        try {
            t.context.server.child.kill('SIGKILL');
            await t.context.server;
        } catch (e) {
            if (e && e.signal === 'SIGKILL') {
                // ignore it.
            } else {
                throw e
            }
        }
    }
})

test("pass", async t => {
    let e = await t.throwsAsync(got('http://0.0.0.0', { port: t.context.port }));
    t.is(e.response.statusCode, 404)
})

// to whom it might concern, this JWT is made of the following header, payload:
// {"alg": "HS256", "typ": "JWT"}
// {"sub": "SGkgdGhpcyBpcyBhIHRlc3QgdG9rZW4K", "iat": 1597096685, "auth_time": 1597096686 }
// The secret used is `aaaaa`
const TEST_JWT = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJTR2tnZEdocGN5QnBjeUJoSUhSbGMzUWdkRzlyWlc0SyIsImlhdCI6MTU5NzA5NjY4NSwiYXV0aF90aW1lIjoxNTk3MDk2Njg2fQ.BNpCIBuNq0bhgXuAEqrAfPpIein0Y54hj352d2ke1sI";

// login cookie
const COOKIE = "X-JWT=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9." +
    "eyJpZCI6IjFmMjQxZTFiLWI1MzctNDkzZi1hMjMwLTA3NWNiMTYzMTViZSIsImNzcmYiOiJSdVF1WmI1QW9HU2R4SUdBIn0.osvyaIW4Mt-3Em4kkuvO4wXAsCVA9gZwkqXlQvQETAs; " +
    "Max-Age=1209600; Path=/v1; HttpOnly; SameSite=Lax; hostOnly=true; aAge=2ms; cAge=6ms; hostOnly=true; aAge=1ms; cAge=1ms";

// login csrf
const CSRF = "RuQuZb5AoGSdxIGA";

test("missing auth (firebase)", async t => {
    const e = await t.throwsAsync(got.post('http://0.0.0.0/v1/user', {
        port: t.context.port,
        // As you can see, we properly have the body, so the only thing that should cause this to fail is...
        json: {
            display_name: "test",
            email: "test@test.test"
        },
        responseType: 'json',
        headers: {
            // ... the fact that we're skipping out on authorization
            // authorization: "Bearer " + TEST_JWT,
        }
    }));

    t.is(e.response.statusCode, 401);
})

test("register user", async t => {
    const cookieJar = new tough.CookieJar();

    const { body } = await got.post('http://0.0.0.0/v1/user', {
        cookieJar,
        port: t.context.port,
        json: {
            display_name: "test",
            email: "test@test.test"
        },
        responseType: 'json',
        headers: {
            authorization: "Bearer " + TEST_JWT,
        }
    })

    t.not(body.csrf, null);
})

test("login user", async t => {
    await runFixtures(['fixtures/1_user.sql'], t.context.dbUrl, t.context.parentDir);

    const cookieJar = new tough.CookieJar();

    const { body } = await got.post('http://0.0.0.0/v1/login', {
        cookieJar,
        port: t.context.port,
        responseType: 'json',
        headers: {
            authorization: "Bearer " + TEST_JWT,
        }
    });

    t.not(body.csrf, null);
})

test("user profile", async t => {
    await runFixtures(['fixtures/1_user.sql'], t.context.dbUrl, t.context.parentDir);

    const cookieJar = new tough.CookieJar();
    await cookieJar.setCookie(COOKIE, 'http://0.0.0.0/v1/login')

    const profile = await got.get('http://0.0.0.0/v1/user/me/profile', {
        cookieJar,
        port: t.context.port,
        responseType: 'json',
        headers: {
            "X-CSRF": CSRF,
        }
    });

    t.snapshot(profile.body);
})

test.todo("create category");
test.todo("delete category");
test.todo("get categories");
test.todo("update category");

test.todo("metadata/affliation");
test.todo("metadata/age-range");
test.todo("metadata/affliations");

test("create image meta", async t => {
    await runFixtures(['fixtures/1_user.sql'], t.context.dbUrl, t.context.parentDir);

    const cookieJar = new tough.CookieJar();
    await cookieJar.setCookie(COOKIE, 'http://0.0.0.0/v1/login')

    const image = await got.post('http://0.0.0.0/v1/image', {
        cookieJar,
        port: t.context.port,
        responseType: 'json',
        headers: {
            "X-CSRF": CSRF,
        },
        json: {
            name: "test",
            description: "testest",
            is_premium: false,
            publish_at: null,
            styles: [],
            age_ranges: [],
            affiliations: [],
            categories: []
        }
    });

    t.deepEqual(typeof (image.body.id), "string");
    t.deepEqual(typeof (image.body.upload_url), "string");
})
