require('dotenv').config();
const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const firebase = require("firebase-admin");

const SHARED_SERVER_SECRET = process.env["SHARED_SERVER_SECRET"];

if(SHARED_SERVER_SECRET == null || SHARED_SERVER_SECRET === "") {
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

    if(req.header("SHARED_SERVER_SECRET") !== SHARED_SERVER_SECRET) {
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

/*
const mysql = require('mysql');
const express = require('express');
const app = express();
const bodyParser = require('body-parser');
app.use(bodyParser.json());

const port = process.env.PORT || 8080;
app.listen(port, () => {
  console.log('Ji Cloud API listening on port', port);
});

app.get('/:id', async (req, res) => {
  const id = parseInt(req.params.id);
  let retVal;
  if (id) {
    try {
      const appointment = await getAppointment(id);
      if (appointment) {
        retVal = {status: 'success', data: {appointment: appointment}};
      }
      else {
        res.status(404);
        retVal = {status: 'fail', data: {title: `Appointment ${id} not found`}};
      }
    }
    catch (ex) {
      res.status(500);
      retVal = {status: 'error', message: ex};
    }
  }
  else {
    res.status(400);
    retVal = {status: 'fail', data: {title: `Appointment id must be integer`}};
  }
  res.setHeader('content-type', 'application/json');
  res.end(JSON.stringify(retVal));
})

app.post('/search', async (req, res) => {
  const user_id = req.body.user_id;
  let retVal;
  if (user_id) {
    try {
      const appointments = await searchAppointments(user_id);
      retVal = {status: 'success', data: {appointments: appointments}};
    }
    catch (ex) {
      res.status(500);
      retVal = {status: 'error', message: ex};
    }
  }
  else {
    res.status(400);
    retVal = {status: 'fail', data: {title: `Request must contain user_id`}};
  }
  res.setHeader('content-type', 'application/json');
  res.end(JSON.stringify(retVal));
})

app.post('/', async (req, res) => {
  let retVal;
  try {
    const id = await createAppointment(req.body);
    const appointment = await getAppointment(id);
    retVal = {status: 'success', data: {appointment: appointment}};
  }
  catch (ex) {
    res.status(500);
    retVal = {status: 'error', message: ex};
  }
  res.setHeader('content-type', 'application/json');
  res.end(JSON.stringify(retVal));
})

app.delete('/:id', async (req, res) => {
  const id = parseInt(req.params.id);
  let retVal;
  if (id) {
    try {
      const success = await deleteAppointment(id);
      if (success) {
        retVal = {status: 'success', data: {title: `Appointment ${id} deleted`}};
      }
      else {
        res.status(404);
        retVal = {status: 'fail', data: {title: `Appointment ${id} not found`}};
      }
    }
    catch (ex) {
      res.status(500);
      retVal = {status: 'error', message: ex};
    }
  }
  else {
    res.status(400);
    retVal = {status: 'fail', data: {title: `Appointment id must be integer`}};
  }
  res.setHeader('content-type', 'application/json');
  res.end(JSON.stringify(retVal));
})

async function getAppointment(id) {
  return new Promise(function(resolve, reject) {
    if (!id) resolve();
    const sql = 'SELECT * FROM appointments where id=?';
    getDbPool().query(sql, [id], (err, results) => {
      if (err) {
        console.error(err);
        reject(getErrorMessage(err));
      }
      else {
        if (results.length > 0) {
          resolve(results[0]);
        }
        else {
          resolve();
        }
      }
    })
  })
}

function getErrorMessage(err) {
  return err.sqlMessage || err.code || err;
}

async function createAppointment(fields) {
  return new Promise(function(resolve, reject) {
    const sql = 'INSERT INTO appointments SET ?';
    getDbPool().query(sql, fields, (err, results) => {
      if (err) {
        console.error(err);
        reject(getErrorMessage(err));
      }
      else {
        resolve(results.insertId);
      }
    })
  })
}

async function searchAppointments(user_id) {
  return new Promise(function(resolve, reject) {
    const sql = 'SELECT * FROM appointments where user_id=?';
    getDbPool().query(sql, [user_id], (err, results) => {
      if (err) {
        console.error(err);
        reject(getErrorMessage(err));
      }
      else {
        resolve(results);
      }
    })
  })
}

async function deleteAppointment(id) {
  return new Promise(function(resolve, reject) {
    const sql = 'DELETE FROM appointments WHERE id=?';
    getDbPool().query(sql, [id], (err, results) => {
      if (err) {
        console.error(err);
        reject(getErrorMessage(err));
      }
      else {
        resolve(results.affectedRows);
      }
    })
  })
}

let cachedDbPool;

function getDbPool() {
  if (!cachedDbPool) {
    cachedDbPool = mysql.createPool({
      connectionLimit: 1,
      user: process.env.SQL_USER,
      password: process.env.SQL_PASSWORD,
      database: process.env.SQL_NAME,
      socketPath: `/cloudsql/${process.env.INST_CON_NAME}`
    });
  }
  return cachedDbPool;
}

*/