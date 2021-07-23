import { initializeApp } from 'firebase/app';
import { getAnalytics } from "firebase/analytics";
import { getDatabase, ref, onValue} from "firebase/database";

let hasInit = false;

let analytics;
let db;

export function _init(target) {
    if(hasInit) {
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

    initializeApp(config);

    analytics = getAnalytics();
    db = getDatabase();

    console.log("firebase initialized! target:", target);

    hasInit = true;
}

export function listenForUploadImage(id) {
    console.log("listening for", 'media-upload-state/image/' + id + '/state');

    const imageStateRef = ref(db, 'media-upload-state/image/' + id + '/state');

    onValue(imageStateRef, (snapshot) => {
        const data = snapshot.val();
        console.log("Image state:", data);
    });    

}
