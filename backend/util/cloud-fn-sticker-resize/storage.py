import os
import tempfile
from typing import Optional

from google.cloud import storage, exceptions
from google.cloud.storage import Client, Bucket


def init_storage_client() -> Client:
    return storage.Client()


def media_key(library, id, file_kind) -> str:
    """
    `library`: {`"global"`, `"user"`}
    `id`: hyphenated UUID string
    `file_kind`: `{"original.png", "resized.png"}`
    """
    return f"media/{library}/{id}/{file_kind}"


def download_blob_to_temp_file(media_bucket: Bucket, file_key) -> Optional[str]:
    """
    Returns the filename where the image was downloaded to (if any).

    If a file name was returned, it MUST be freed using `os.remove(_)`.
    """
    _, temp_local_filename = tempfile.mkstemp()

    try:
        media_bucket.blob(file_key).download_to_filename(temp_local_filename)
        print(f"Image {file_key} was downloaded to {temp_local_filename}")
        return temp_local_filename

    except exceptions.NotFound:
        print(f"Image {file_key} was not found in media bucket!")
        os.remove(temp_local_filename)
        return None


def upload_file_to_blob(media_bucket: Bucket, bucket_file_key, local_filename):
    media_bucket.blob(bucket_file_key).upload_from_filename(filename=local_filename)
