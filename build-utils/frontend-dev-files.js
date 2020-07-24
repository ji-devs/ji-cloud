if(process.argv.length < 3) {
    console.error("usage is `npm run frontend-dev-files -- [dirname]`");
    process.exit(1);
}

switch(process.argv[2]) {
	case "user":
		break;
	default: {
		console.error("supply valid APP (and note the space) for frontend-dev-files -- [APP]");
		process.exit(1);
	}
}

const path = require('path');
const fs = require('fs-extra');

const srcPath = path.resolve("../frontend/_core/devfiles");
const targetPath = `../frontend/${process.argv[2]}/app/dist`;


fs.copy(srcPath, targetPath)
  .catch(err => console.error(err))
