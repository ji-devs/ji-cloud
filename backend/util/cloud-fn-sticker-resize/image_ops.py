import os

from google.cloud.storage import Bucket
from wand.image import Image

from storage import media_key, download_blob_to_temp_file, upload_file_to_blob

STICKER_WIDTH = 1440
STICKER_HEIGHT = 810

# FIXME batch this? slow vs. memory intensive
def resize_one(media_bucket: Bucket, library, n_resized) -> None:
    """
    Resizes the previously incorrectly sized Sticker images:
    * If the image is larger than required, then it needs to be sized down.
    * Otherwise the image was sized up when it should not have been. In this case we replace `resized.png` with the `original.png` in the same resource directory.
    """
    global n_sized_down, n_replaced_original

    resized_image_key = media_key(library, id, "resized.png")

    temp_local_filename = download_blob_to_temp_file(media_bucket, resized_image_key)

    if temp_local_filename is None:
        return

    with Image(filename=temp_local_filename) as image:
        width, height = image.width(), image.height()

        if width > STICKER_WIDTH or height > STICKER_HEIGHT:
            image.resize(STICKER_WIDTH, STICKER_HEIGHT, "sincfast")
            upload_file_to_blob(media_bucket, resized_image_key, temp_local_filename)
            print(f"Resized one image to proper size: {resized_image_key}")
            n_resized[0] += 1

        elif width <= STICKER_WIDTH or height <= STICKER_HEIGHT:
            os.remove(temp_local_filename)

            original_image_key = media_key(library, id, "original.png")
            temp_local_filename = download_blob_to_temp_file(
                media_bucket, original_image_key
            )
            if temp_local_filename is None:
                print(
                    f"Error occured while downloading the {original_image_key} for replacement!"
                )

            upload_file_to_blob(media_bucket, resized_image_key, temp_local_filename)
            print(f"Replaced {resized_image_key} with {original_image_key}")

            n_resized[1] += 1

        elif width < STICKER_WIDTH and height < STICKER_HEIGHT:
            print("Small image was sized correctly? ({width}, {height})?")
        else:
            print("Case not handled? ({width}, {height})")

    os.remove(temp_local_filename)
