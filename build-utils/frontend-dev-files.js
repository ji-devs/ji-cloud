const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs-extra');

if(process.argv.length < 3) {
    console.error("usage is `npm run frontend-dev-files -- [dirname]`");
    process.exit(1);
}

const SPA = process.argv[2];
switch(SPA) {
	case "user":
	case "admin":
	case "jig":
	case "module/memory-game/edit":
		break;
	default: {
		console.error("supply valid APP (and note the space) for frontend-dev-files -- [APP]");
		process.exit(1);
	}
}

//CSS
execSync(`npm run _bundle-prod`, {cwd: path.resolve(`../frontend/css`)});
fs.copy(path.resolve(`../frontend/css/dist/styles.min.css`), path.resolve(`../frontend/app/${SPA}/dist/css/styles.min.css`))
  .catch(err => console.error(err));

//HTML
const src = path.resolve("../frontend/core/devfiles/index.html")
const dest = path.resolve(`../frontend/app/${SPA}/dist/index.html`);

fs.readFile(src, 'utf-8')
    .then(html => html.replace("{{SPA}}", SPA))
    .then(html => fs.writeFile(dest, html))
    .catch(err => console.error(err));

