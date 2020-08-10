const { default: test } = require('ava');

const got = require('got');
getPort = require('get-port');
const { execSync, exec, spawnSync } = require('child_process');
const { argv, stdout } = require('process');
const { copyFile, copyFileSync, mkdirSync } = require('fs');
const path = require('path');
const tough = require('tough-cookie');
const { TLSSocket } = require('tls');
function hookServerStarted(server) {
    return new Promise(resolve => {
        server.stdout.on('data', function (data) {
            if (data.toString().includes(`Starting "actix-web-service-`)) {
                resolve();
            }
        });
    })
}

test.before(async t => {
    t.context.parentDir = path.resolve(process.cwd(), '..');

    execSync("cargo build --manifest-path ../Cargo.toml");

    try {
        mkdirSync("bin")
    }
    catch (e) {
        if (e && e.code === 'EEXIST') {
            // ignore it.
        } else {
            throw e
        }
    }
    copyFileSync("../target/debug/ji-cloud-api", "bin/ji-cloud-api");
})

test.beforeEach(async t => {
    const port = await getPort();

    var parentDir = t.context.parentDir;
    const dbUrl = execSync("../../script/ephemeralpg/pg_tmp.sh", { encoding: 'utf8' });

    t.context.dbUrl = dbUrl;

    execSync("cargo sqlx migrate run", { cwd: parentDir, env: { DATABASE_URL: `${dbUrl}`, PGUSER: "postgres" }, encoding: 'utf8' });

    const env = {
        LOCAL_API_PORT: port,
        DATABASE_URL: `${dbUrl}`,
        PGUSER: "postgres",
        JWT_SECRET: "abc123",
        INTER_SERVER_SECRET: "aaa",
        LOCAL_PAGES_PORT: 0,
        LOCAL_NO_FIREBASE_AUTH: true
    };

    t.context.port = port;
    t.context.server = exec("bin/ji-cloud-api", { env: env, encoding: 'utf8' });

    await hookServerStarted(t.context.server);
})

test.afterEach.always(t => {
    if (t.context.server) {
        t.context.server.kill('SIGTERM');
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
    const parentDir = t.context.parentDir;
    const dbUrl = t.context.dbUrl;

    execSync(`/usr/bin/psql -f fixtures/1_user.sql ${dbUrl}`, { cwd: parentDir, env: { PGUSER: "postgres" }, encoding: 'utf8' });

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
    const parentDir = t.context.parentDir;
    const dbUrl = t.context.dbUrl;

    execSync(`/usr/bin/psql -f fixtures/1_user.sql ${dbUrl}`, { cwd: parentDir, env: { PGUSER: "postgres" }, encoding: 'utf8' });

    const cookieJar = new tough.CookieJar();

    const login = await got.post('http://0.0.0.0/v1/login', {
        cookieJar,
        port: t.context.port,
        responseType: 'json',
        headers: {
            authorization: "Bearer " + TEST_JWT,
        }
    });

    const profile = await got.get('http://0.0.0.0/v1/user/me/profile', {
        cookieJar,
        port: t.context.port,
        responseType: 'json',
        headers: {
            "X-CSRF": login.body.csrf,
        }
    });

    t.deepEqual(profile.body.display_name, "test");
    t.deepEqual(profile.body.email, "test@test.test");
    t.deepEqual(profile.body.id, "1f241e1b-b537-493f-a230-075cb16315be");
    t.deepEqual(profile.body.scopes, []);
})

test.todo("create category");
test.todo("delete category");
test.todo("get categories");
test.todo("update category");
