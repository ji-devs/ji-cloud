require('dotenv').config();
const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const firebase = require("firebase-admin");
const {getSecret} = require("./secrets");
const settings = require("./settings");

(async () => {

    settings.init();

    const INTER_SERVER_SECRET = await getSecret("INTER_SERVER");

    if(INTER_SERVER_SECRET == null || INTER_SERVER_SECRET === "") {
      console.error("MISSING SHARED SERVER SECRET!");
    } else {
      firebase.initializeApp();

      app.use(bodyParser.json());

      const port = process.env.PORT || 8082;

      app.listen(port, () => {
        console.log('Ji Cloud API listening on port', port);
      });


      //TODO - checking for shared secret should be middleware that returns bad statuscode
      app.get('/validate-firebase-token/:token', async (req, res) => {
        res.setHeader('content-type', 'application/json');

        if(req.header("INTER_SERVER_SECRET") !== INTER_SERVER_SECRET) {
          res.end(JSON.stringify({valid: false}));
        } else {
          try {
            const token = await firebase.auth().verifyIdToken(req.params.token);
            res.end(JSON.stringify({valid: true}));
          } catch(e) {
            res.end(JSON.stringify({valid: false}));
          }
        }
      });
    }
})();



