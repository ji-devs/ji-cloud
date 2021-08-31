import os
from ctypes import c_void_p, c_size_t

from google.cloud.storage import Bucket
from wand.api import library as wand_lib
from wand.image import Image

from storage import media_key, download_blob_to_temp_file, upload_file_to_blob

wand_lib.MagickSetCompressionQuality.argtypes = [c_void_p, c_size_t]

STICKER_WIDTH = 1440
STICKER_HEIGHT = 810

# FIXME batch this? slow vs. memory intensive
def resize_one(media_bucket: Bucket, library, image_id, n_resized) -> None:
    """
    Resizes the previously incorrectly sized Sticker images:
    * If the image is larger than required, then it needs to be sized down.
    * Otherwise the image was sized up when it should not have been. In this case we replace `resized.png` with the `original.png` in the same resource directory.
    """
    global n_sized_down, n_replaced_original

    original_image_key = media_key(library, image_id, "original.png")
    resized_image_key = media_key(library, image_id, "resized.png")

    temp_local_filename = download_blob_to_temp_file(media_bucket, original_image_key)

    if temp_local_filename is None:
        return

    with Image(filename=temp_local_filename) as image:
        width, height = image.width, image.height

        if width > STICKER_WIDTH or height > STICKER_HEIGHT:
            image.transform(resize=f"{STICKER_WIDTH}x{STICKER_HEIGHT}")
            
            wand_lib.MagickSetCompressionQuality(image.wand, 75)
            image.save(filename=temp_local_filename)

            upload_file_to_blob(media_bucket, resized_image_key, temp_local_filename)
            
            n_resized[0] += 1
            print(f"{n_resized[0]}: Resized one image to proper size: {resized_image_key}")


        elif width <= STICKER_WIDTH or height <= STICKER_HEIGHT:
            
            wand_lib.MagickSetCompressionQuality(image.wand, 75)
            image.save(filename=temp_local_filename)

            upload_file_to_blob(media_bucket, resized_image_key, temp_local_filename)
            
            n_resized[1] += 1
            print(f"{n_resized[1]}: Replaced {resized_image_key} with {original_image_key}")

            

        else:
            print("Case not handled? ({width}, {height})")

    os.remove(temp_local_filename)
