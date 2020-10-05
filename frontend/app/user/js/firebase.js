/*
    The firebase token is only for authentication - not authorization
    Essentially we use it as proof to the service that the user is who they claim
    But that isn't enough to do anything, and this proof is a one-time bearer token

    That token is, however, used by the server to set the real authorization token
    Which is used as a cookie and automatically sent when hitting the various endpoints
*/

export function init_firebase(dev) {

    const firebaseDevConfig = {
        apiKey: "AIzaSyALsii1P1nKENhgszj1tz8pRqCXct3eck0",
        authDomain: "ji-cloud-developer-sandbox.firebaseapp.com",
        databaseURL: "https://ji-cloud-developer-sandbox.firebaseio.com",
        projectId: "ji-cloud-developer-sandbox",
        storageBucket: "ji-cloud-developer-sandbox.appspot.com",
        messagingSenderId: "735837525944",
        appId: "1:735837525944:web:10e1fc18d5d10f04c3614d"
    };

    const firebaseProdConfig = {
        apiKey: "AIzaSyB1aDTWI5nez8SJe6oGp-o2LErxAEDSktQ",
        authDomain: "ji-cloud.firebaseapp.com",
        databaseURL: "https://ji-cloud.firebaseio.com",
        projectId: "ji-cloud",
        storageBucket: "ji-cloud.appspot.com",
        messagingSenderId: "516631917755",
        appId: "1:516631917755:web:842b4c92c60041dd5ca59e",
        measurementId: "G-4V46KRQZPB"
    };


    firebase.initializeApp(dev ? firebaseDevConfig : firebaseProdConfig);
    firebase.auth().setPersistence(firebase.auth.Auth.Persistence.NONE);
}


export function firebase_register_google() {

    const provider = new firebase.auth.GoogleAuthProvider();
    provider.addScope('profile');
    provider.addScope('email');
    return complete_register(firebase.auth().signInWithPopup(provider));
}
export function firebase_register_email(email, password) {
    return complete_register(firebase.auth().createUserWithEmailAndPassword(email, password));
}

function complete_register(promise) {
    return promise
        .then(({user}) => 
            user.getIdToken()
                .then(token => ({token, name: user.displayName, email: user.email, avatar: user.photoURL}))
        )
        .then(result => {
            firebase.auth().signOut();
            return result; 
        });
}

export function firebase_signin_google() {

    const provider = new firebase.auth.GoogleAuthProvider();
    provider.addScope('profile');
    provider.addScope('email');
    return complete_signin(firebase.auth().signInWithPopup(provider));
}

export function firebase_signin_email(email, password) {
    return complete_signin(firebase.auth().signInWithEmailAndPassword(email, password));
}

function complete_signin(promise) {
    return promise
        .then(({user}) => 
            user.emailVerified 
            ? user 
            : Promise.reject({
                code: "internal/confirm-email"
            })
        )
        .then(user => user.getIdToken())
        .then(idToken => {
            firebase.auth().signOut();
            return idToken
        });
}

export function firebase_forgot_password(email) {
    return firebase
        .auth()
        .sendPasswordResetEmail(email)
}

function getCookie(name) {
  var v = document.cookie.match('(^|;) ?' + name + '=([^;]*)(;|$)');
  return v ? v[2] : null;
}
