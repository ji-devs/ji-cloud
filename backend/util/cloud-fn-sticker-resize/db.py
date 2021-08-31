import os

import sqlalchemy
from sqlalchemy import engine

from typing import List


def init_db_pool() -> engine.Engine:
    db_config = {
        "pool_size": 2,
        "max_overflow": 0,
        "pool_timeout": 30,  # seconds
        "pool_recycle": 1800,  # shouldn't take longer than this?
    }

    db_user = os.environ["DB_USER"]
    db_pass = os.environ["DB_PASS"]
    db_name = os.environ["DB_NAME"]
    db_socket_dir = "/cloudsql"
    cloud_sql_connection_name = os.environ[
        "CLOUD_SQL_CONNECTION_NAME"
    ]  # i.e "<PROJECT-NAME>:<INSTANCE-REGION>:<INSTANCE-NAME>"

    # pool = sqlalchemy.create_engine(
    #     engine.url.URL.create(
    #         drivername="postgresql+pg8000",
    #         username=db_user,
    #         password=db_pass,
    #         database=db_name,
    #         query={
    #             "unix_sock": f"{db_socket_dir}/{cloud_sql_connection_name}/.s.PGSQL.5432"
    #         },
    #     ),
    #     **db_config,
    # )
    
    pool = sqlalchemy.create_engine(f"postgresql+pg8000://{db_user}:{db_pass}@/{db_name}?unix_sock={db_socket_dir}/{cloud_sql_connection_name}/.s.PGSQL.5432")

    return pool


global_image_query = sqlalchemy.text(
    "select id"
    "from image_upload"
    "where (processing_result is not distinct from true) and (kind = 1) and processed_at < '2021-08-22T12:36:50.371709Z'" 
)

user_image_query = sqlalchemy.text(
    "select id"
    "from user_image_upload"
    "where (processing_result is not distinct from true) and (kind = 1) and processed_at < '2021-08-22T12:36:50.371709Z'" 
)


def fetch_global_stickers(db_pool: engine.Engine) -> List[str]:

    with db_pool.begin() as conn:
        old_image_ids = conn.execute(global_image_query).fetchall()

    return [row for row in old_image_ids]


def fetch_user_stickers(db_pool: engine.Engine) -> List[str]:

    with db_pool.begin() as conn:
        old_image_ids = conn.execute(user_image_query).fetchall()

    return [row for row in old_image_ids]
