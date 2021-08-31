# https://cloud.google.com/functions/docs/tutorials/imagemagick#functions_imagemagick_setup-python
import os
import time

import image_ops, db, storage

MEDIA_BUCKET = os.environ["MEDIA_BUCKET"]

storage_client = storage.init_storage_client()
db_pool = db.init_db_pool()

n_resized = [0, 0]

def process_images(dummy_request):
    """Downloads and processes all images uploaded before resize logic fix deployment
    """
    global n_resized
    
    media_bucket = storage_client.bucket(MEDIA_BUCKET)
    
    stickers = db.fetch_global_stickers(db_pool)
    
    for s in stickers:
        print(s)
        
    stickers = db.fetch_user_stickers(db_pool)
    
    for s in stickers:
        print(s)
    
    # process_global_images(db_pool, media_bucket)
    
    n_resized = [0, 0]
    
    # process_user_iamges(db_pool, media_bucket)
    
    return


def process_global_images(db_pool: db.engine.Engine, media_bucket: storage.Bucket):
    global n_resized
    
    stickers = db.fetch_global_stickers(db_pool)
    
    print(f"A total of {len(stickers)} found that need to potentially resized in the global library. First: {stickers[0]}")
        
    for img in stickers:
        start = time.time()
        image_ops.resize_one(media_bucket, "global", n_resized)

    print(f"Number of global images resized that were originally missed: {n_resized[0]}")
    print(f"Number of global images replaced with original that were mistakenly sized up: {n_resized[1]}")



def process_user_iamges(db_pool: db.engine.Engine, media_bucket: storage.Bucket):
    global n_replaced_original, n_sized_down
    
    stickers = db.fetch_user_stickers(db_pool)
    
    print(f"A total of {len(stickers)} found that need to potentially resized in the global library. First: {stickers[0]}")
        
    for img in stickers:
        start = time.time()
        image_ops.resize_one(media_bucket, "user", n_resized)

    print(f"Number of user images resized that were originally missed: {n_resized[0]}")
    print(f"Number of user images replaced with original that were mistakenly sized up: {n_resized[1]}")
    
    