const { default: test } = require('ava');

const got = require('got');
getPort = require('get-port');
const { execSync, exec, spawnSync } = require('child_process');
const { argv, stdout } = require('process');
const { copyFile, copyFileSync, mkdirSync } = require('fs');
const path = require('path');
const tough = require('tough-cookie');
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

    var parentDir = path.resolve(process.cwd(), '..');
    const db_url = execSync("../../script/ephemeralpg/pg_tmp.sh", { encoding: 'utf8' });

    execSync("cargo sqlx migrate run", { cwd: parentDir, env: { DATABASE_URL: `${db_url}`, PGUSER: "postgres" }, encoding: 'utf8' })

    const env = {
        LOCAL_API_PORT: port,
        DATABASE_URL: `${db_url}`,
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
// {"sub": "SGkgdGhpcyBpcyBhIHRlc3QgdG9rZW4K"}
// The secret used is `aaaaa`
const TEST_JWT = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJTR2tnZEdocGN5QnBjeUJoSUhSbGMzUWdkRzlyWlc0SyJ9.GjY_3h8RAe5cH4cDGwPNpVP72MRZLGPTYdnZU7y4VMI";


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

test.todo("auth fail")
test.todo("user profile");

test.todo("create category");
test.todo("delete category");
test.todo("get categories");
test.todo("update category");
