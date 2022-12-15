#!/bin/sh

gsutil -m rsync -d -r /home/david/archive/legacy-cdn gs://ji-cloud-legacy-eu-001


gsutil -m cp -r /Users/dakom/Downloads/jigzi-data/games/* gs://ji-cloud-legacy-eu-001/transcode/games


gsutil -m cp -r gs://ji-cloud-legacy-eu-001/* /home/david/archive/legacy-cdn
