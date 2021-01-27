# Directory Structure

There are currently 2 top level directories:

  - **core**: Used everywhere. Examples are "buttons" and "images"
  - **entry**: Corresponds to the actual app entry point, follows the division of Rust code and backend routes.

Within `entry`, there are some optional patterns which may appear at any level:

  - **_common**: used in multiple places from this directory and deeper (but not parent)
  - **pages**: full pages.
  - **buttons**, **sections**, **widgets**, etc.: exactly as they sound, used in this directory and deeper (but not parent). The names here can also be specific for a unique one-off component/element.