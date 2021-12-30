create table user_pdf_library
(
    -- ID of PDF
    id         uuid     primary key                  default uuid_generate_v1mc() not null, 
    -- Uploader of PDF
    user_id    uuid     references "user"(id)        on delete cascade,
    
    created_at timestamptz              default now() not null,
    updated_at timestamptz
);

create table user_pdf_upload
(
    pdf_id   uuid        primary key                  references user_pdf_library(id) on delete restrict not null,
    uploaded_at          timestamptz,
    processed_at         timestamptz,
    processing_result    boolean
);
