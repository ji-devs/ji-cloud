# Images

There is never a need to use `<img>` directly. We have precisely two storages:

* `<img-ui>`: corresponds to the Dropbox folder for static ui images
* `<img-ji>`: corresponds to uploaded images on our server.

Consult the elements for the full list of available properties.

There is one outlier - where we want to mock an `<img-ji>` without actually pointing to an uploaded image.
For this, still use `<img-ji>` (so that it's clear for app implementation), but set the `lib` attribute to `mock` and it will internally load the image from the Dropbox/ui/mock folder.

