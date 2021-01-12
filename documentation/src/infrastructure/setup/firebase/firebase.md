# Firebase

The firebase side should be connected to the google cloud, i.e. both release and sandbox projects

Make sure to follow the firebase guidelines - e.g. to allow the domains for oauth


## Auth config 

Edit `frontend/apps/crates/entry/user/js/firebase.js` and put in the config for both sandbox and release

These aren't secrets, it's okay to be checked into the repo and it will be publically viewable through the browser anyway
