<p align="center">
    <img alt="Hosting and Deployment High Level View" src="./hosting-deployment.drawio.svg">
</p>

# CDN Purging (and cloud functions)

* TODO: https://github.com/fastly/purge-fastly-gcs-trigger

## Notes:

* The release/sandbox split is consistent. A sandbox frontend will hit sandbox server which hits sandbox sql.
* There are several static hosting endpoints. It's primarily split to keep frontend weight down.
* The servers are exactly one per type (i.e. there is only one template server even if it serves content from different static firebase endpoints.). Cloud-run will scale horizontally as needed

