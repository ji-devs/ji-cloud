select id as "id!",
    case kind
        -- PngCanvasImage
        when 0 then 3
        -- PngStickerImage
        when 1 then 0
    end::int2 "kind!: MediaKind", 
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    0::int2 as "library!: MediaLibrary" -- global
from image_metadata
left join image_upload on image_id = id
union all
select id as "id!",
    case kind
        -- GifAnimation
        when 0 then 1
        -- SpritesheetAnimation
        when 1 then 2
    end::int2 "kind!: MediaKind", 
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    0::int2 as "library!: MediaLibrary" -- global
from animation_metadata
left join global_animation_upload on animation_id = id
union all
select id as "id!",
    -- PngStickerImage
    0::int2 as "kind!: MediaKind",
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    1::int2 as "library!: MediaLibrary" -- user
from user_image_library
left join user_image_upload on image_id = id
union all
select id as "id!",
    -- Mp3Audio
    4::int2 as "kind!: MediaKind",
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    1::int2 as "library!: MediaLibrary" -- user
from user_audio_library
union all
select id as "id!",
    kind as "kind!: MediaKind",
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    2::int2 as "library!: MediaLibrary" -- web
from web_media_library
