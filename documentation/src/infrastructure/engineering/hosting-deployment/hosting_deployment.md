<p align="center">
    <img alt="Hosting and Deployment High Level View" src="./hosting-deployment.drawio.svg">
</p>

## Notes:

* The release/sandbox split is consistent. A sandbox frontend will hit sandbox server which hits sandbox sql.
* There are several static hosting endpoints. It's primarily split to keep frontend weight down.

