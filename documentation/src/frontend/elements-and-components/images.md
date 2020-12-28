# Images

There is never a need to use `<img>` directly. We have precisely two storages:

* `<img-ui>`: corresponds to the Dropbox folder for static ui images
* `<img-ji>`: corresponds to uploaded images on our server.

Consult the elements for the full list of available properties.

### Components

Since these images are ubiquitous, it's recommended to import the component from `~/components/images` and use those instead of manually writing out the element everywhere. The components accept the same properties as the element (including the optional, built-in `slot`).

### Mocks

There is one outlier - where we want to mock an `<img-ji>` without actually pointing to an uploaded image.
For this, we still use the `<img-ji>` element (so that it's clear for app implementation), but set the `lib` attribute to `mock` and it will internally load the image from the Dropbox/ui/mock folder.

As mentioned above, we generally want to use the component instead, and that's via `MockJiImage` (also in `~/components/images`). This only requires specifying a `size` - and it will point to the appropriate `img.png` (it uses png instead of jpg, even if jpg would be appropriate in the real case, since it is just a mock and the intent is to cover _all_ cases, including where it needs transparency)

For example:

```
import {MockJiImage} from "~/components/images";

//In some component:
${MockJiImage({size: "full", slot: "image"})}
```