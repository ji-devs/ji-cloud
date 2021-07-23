import { initializeApp } from 'firebase/app';
import { getAnalytics } from "firebase/analytics";
import { getFirestore, doc, onSnapshot } from "firebase/firestore";

let app;
let analytics;
let db;

export function _init(target) {
    if(app != undefined) {
        return;
    }

    const config = target == "release" 
        ? {
              apiKey: "AIzaSyB1aDTWI5nez8SJe6oGp-o2LErxAEDSktQ",
              authDomain: "ji-cloud.firebaseapp.com",
              databaseURL: "https://ji-cloud.firebaseio.com",
              projectId: "ji-cloud",
              storageBucket: "ji-cloud.appspot.com",
              messagingSenderId: "516631917755",
              appId: "1:516631917755:web:842b4c92c60041dd5ca59e",
              measurementId: "G-4V46KRQZPB"
          }
        : {
          apiKey: "AIzaSyALsii1P1nKENhgszj1tz8pRqCXct3eck0",
          authDomain: "ji-cloud-developer-sandbox.firebaseapp.com",
          databaseURL: "https://ji-cloud-developer-sandbox.firebaseio.com",
          projectId: "ji-cloud-developer-sandbox",
          storageBucket: "ji-cloud-developer-sandbox.appspot.com",
          messagingSenderId: "735837525944",
          appId: "1:735837525944:web:10e1fc18d5d10f04c3614d"
        };    

    app = initializeApp(config);
    analytics = getAnalytics(app);
    db = getFirestore(app);

    console.log("firebase initialized! target:", target);
}

let uploadListeners = new Map();
let uploadListenerCounter = 0;

export function addUploadListener(mediaId, listener) {

    const listenerId = uploadListenerCounter++;

    uploadListeners.set(listenerId, listener);

    const ref = doc(db, "uploads", mediaId);

    onSnapshot(ref, doc => {
        if(uploadListeners.has(listenerId)) {
            const onStatus = uploadListeners.get(listenerId);

            const data = doc.data();

            if(data == null) {
                onStatus({
                    ready: false,
                    processing: false
                });
            } else {
                onStatus({
                    ready: data.ready === true,
                    processing: data.processing === true
                });
            }
        }
    });

    return listenerId;
}

export function removeUploadListener(listenerId) {
    if(uploadListeners.has(listenerId)) {
        uploadListeners.delete(listenerId);
    } 
}
