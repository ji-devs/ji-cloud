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

//Abortable Promise example: https://codepen.io/dakom/pen/LYyOvwV?editors=1111
export function waitForUploadReady(mediaId, libId, abortController) {

    return new Promise((resolve, reject) => {
        const ref = doc(db, "uploads", "media", libId, mediaId);
        
        if(abortController != null) {
            abortController.signal.onabort = () => {
                reject();
            }
        }

        let hasBeenNotReady = false;

        onSnapshot(ref, doc => {
            if(abortController == null || !abortController.signal.aborted) {
                const data = doc.data();
                const status = data == null 
                    ?  {
                        ready: false,
                        processing: false
                    }
                    : {
                        ready: data.ready === true,
                        processing: data.processing === true
                    };

                console.log(status);

                if(status.ready) {
                    if(hasBeenNotReady) {
                        resolve();
                    } else {
                        console.log("technically ready but never wasn't, waiting for next ready");
                    }
                } else {
                    hasBeenNotReady = true;
                }
            }
        });
    });
}
