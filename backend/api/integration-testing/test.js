const { default: test } = require('ava');

const got = require('got');
getPort = require('get-port');
const { execSync, exec, spawnSync } = require('child_process');
const { argv, stdout } = require('process');
const { copyFile, copyFileSync, mkdirSync } = require('fs');
const path = require('path');

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

    const env = { LOCAL_API_PORT: port, DATABASE_URL: `${db_url}`, PGUSER: "postgres", JWT_SECRET: "abc123", INTER_SERVER_SECRET: "aaa", LOCAL_PAGES_PORT: 0 };

    t.context.port = port;
    t.context.server = exec("bin/ji-cloud-api", { env: env, encoding: 'utf8' });
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

test.todo("register user")
test.todo("auth fail")
test.todo("user profile");

test.todo("create category");
test.todo("delete category");
test.todo("get categories");
test.todo("update category");
