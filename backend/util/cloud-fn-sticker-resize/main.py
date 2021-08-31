# https://cloud.google.com/functions/docs/tutorials/imagemagick#functions_imagemagick_setup-python
import os
import time

import image_ops, db, storage

MEDIA_BUCKET = os.environ["MEDIA_BUCKET"]

storage_client = storage.init_storage_client()
db_pool = db.init_db_pool()

n_global_resized = [0, 0]
n_user_resized = [0, 0]

def process_images(dummy_request):
    """Downloads and processes all images uploaded before resize logic fix deployment"""
    global n_global_resized

    media_bucket = storage_client.bucket(MEDIA_BUCKET)

    process_global_images(db_pool, media_bucket)

    process_user_images(db_pool, media_bucket)
    
    return f"Done! \n\n     resized, replaced: \nGlobal: {n_global_resized}\n  User: {n_user_resized}"



def process_global_images(db_pool: db.engine.Engine, media_bucket: storage.Bucket):
    global n_global_resized

    stickers = db.fetch_global_stickers(db_pool)

    print(
        f"A total of {len(stickers)} found that need to potentially resized in the global library. First: {stickers[0]}"
    )

    start = time.time()
    for image_id in stickers:
        image_ops.resize_one(media_bucket, "global", image_id, n_global_resized)

    print(f"Processing completed in: {time.time() - start}")
    print(
        f"Number of global images resized down to the required size: {n_global_resized[0]}"
    )
    print(
        f"Number of global images replaced with original that were mistakenly sized up: {n_global_resized[1]}"
    )


def process_user_images(db_pool: db.engine.Engine, media_bucket: storage.Bucket):
    global n_replaced_original, n_sized_down

    stickers = db.fetch_user_stickers(db_pool)

    print(
        f"A total of {len(stickers)} found that need to potentially resized in the user library. First: {stickers[0]}"
    )

    start = time.time()
    for image_id in stickers:
        image_ops.resize_one(media_bucket, "user", image_id, n_user_resized)

    print(f"Processing completed in: {time.time() - start}")
    print(f"Number of user images resized down to the required size: {n_user_resized[0]}")
    print(
        f"Number of user images replaced with original that were mistakenly sized up: {n_user_resized[1]}"
    )


if __name__ == "__main__":
    process_images(0)
