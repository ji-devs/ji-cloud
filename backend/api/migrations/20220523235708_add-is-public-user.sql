alter table user_profile
    add column  bio                      text                   default ''::text    not null,
    add column  location_public          bool                   default false      not null,
    add column  organization_public      bool                   default false      not null,
    add column  persona_public           bool                   default false      not null,
    add column  language_public          bool                   default false      not null;

