# Media

Media is synced via Dropbox, in order to allow graphics, audio, and other media artists to contribute their work without going through git.

The local dev configuration happens like everything else in [config](../config/config.md)

In order to have media updates automatically propogate all the way up to the CDN, a bit of manual setup work needs to be done first:

1. Locally install [rclone](https://rclone.org/). Not on the server, but on a dev machine
2. Use it to create the authenticated dropbox remote
3. Same for cloud storage (though no need for authentication - that will happen via automatic credentials)
4. Create the bucket like all the others (and setup w/ fastly, etc.)
5. Create a small Compute Engine instance (i.e. e2 micro) and give its service account owner access to the bucket
6. Install rclone on that instance
7. Copy the local config file (*which will contain the config keys*) to the remote VM (run `rclone config file` on the vm to get path)
8. Write a small shell script and chmod u+x it with the following, replacing the remote and bucket names as needed:

```
#!/bin/sh
rclone sync dropbox:/ji-cloud-media media-storage:/ji-cloud-media-origin-eu-001/
```

9. Setup a cron task to run rclone and sync at some interval. For example, every 2 minutes:

```
*/2 * * * * /home/david/sync.sh
```



