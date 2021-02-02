select id as "id!",
    case kind
        -- PngCanvasImage
        when 0 then 3
        -- PngStickerImage
        when 1 then 0
    end "kind!: MediaKind", 
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    0 as "library!: MediaLibrary" -- global
from image_metadata
union all
select id as "id!",
    case variant
        -- GifAnimation
        when 0 then 1
        -- SpritesheetAnimation
        when 1 then 2
    end "kind!: MediaKind", 
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    0 as "library!: MediaLibrary" -- global
from animation
union all
select id as "id!",
    -- PngStickerImage
    0 as "kind!: MediaKind",
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    1 as "library!: MediaLibrary" -- user
from user_image_library
union all
select id as "id!",
    -- Mp3Audio
    4 as "kind!: MediaKind",
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    1 as "library!: MediaLibrary" -- user
from user_audio_library
union all
select id as "id!",
    kind as "kind!: MediaKind",
    created_at as "created_at!",
    updated_at,
    uploaded_at,
    2 as "library!: MediaLibrary" -- web
from web_media_library
