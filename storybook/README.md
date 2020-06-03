[![Build Status](https://github.com/jewish-interactive/ji-cloud-storybook/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/jewish-interactive/ji-cloud-storybook/actions)

# [https://storybook.jicloud.org](https://storybook.jicloud.org/)

# [styles.min.css](https://storybook.jicloud.org/dist/styles.min.css)

See [storybook-for-dominator-boilerplate](https://github.com/dakom/storybook-for-dominator-boilerplate) for dev notes.

Because it deploys to firebase (not github) and notifies slack, it needs these CI secrets (and not GH_PAT):

* FIREBASE_TOKEN (generated via firebase-tools `firebase login:ci`) 
* SLACK_BOT_TOKEN (the one that begins "xoxb-")

Also, because we use a genuine remote for media storage, the "copy-media-directory" and associated steps are gone