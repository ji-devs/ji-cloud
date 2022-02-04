-- The previous migration included only JIG modules which had the theme set under
-- `content->base->theme`, however some module types have the theme directly under `content`. This
-- migration migrates the themes for those remaining modules.

-- Jigzi Theme ids
create table theme (
    display_name         text           not null,
    index                smallint       unique not null check (index >= 0),
    created_at           timestamptz not null    default now(),
    updated_at           timestamptz
);

--temporary insert to match theme display names with jig data theme
insert into theme(display_name, index)
values('Blank', 0),
    ('Jigzi', 1),
    ('Chalkboard', 2),
    ('MyNotebook', 3),
    ('BackToSchool', 4),
    ('MyWorkspace', 5),
    ('Comix', 6),
    ('Surrea', 7),
    ('Abstract', 8),
    ('Denim', 9),
    ('HappyBrush', 10),
    ('Graffiti', 11),
    ('JewishText', 12),
    ('ShabbatShalom', 13),
    ('RoshHashana', 14),
    ('AppleWithHoney', 15),
    ('Pomegranate', 16),
    ('YomKippur', 17),
    ('HappySukkot', 18),
    ('Sukkot', 19),
    ('IlluminatingHanukkah', 20),
    ('Chanukah', 21),
    ('ChanukahLights', 22),
    ('Purim', 23),
    ('PurimFeast', 24),
    ('PurimSweets', 25),
    ('HappyPassover', 26),
    ('PassoveMatza', 27),
    ('PassoverSeder', 28),
    ('HappyShavuot', 29),
    ('ShavuotDishes', 30),
    ('ShavuotFields', 31),
    ('OurIsrael', 32),
    ('Israel', 33),
    ('JerusalemCity', 34),
    ('JerusalemWall', 35),
    ('LovelySpring', 36),
    ('Spring', 37),
    ('WatermelonSummer', 38),
    ('SummerPool', 39),
    ('ExcitingFall', 40),
    ('Autumn', 41),
    ('WinterSnow', 42),
    ('IceAge', 43),
    ('LostInSpace', 44),
    ('Space', 45),
    ('Camping', 46),
    ('HappyBirthday', 47),
    ('Jungle', 48),
    ('OurPlanet', 49),
    ('Theater', 50),
    ('Travel', 51);

-- Remove "Override" in theme
select * from jig_data_module for update;

drop trigger set_updated_at on jig_data_module;

-- replace all "{"theme": "Jig"}" with "{"theme": <value>}"
-- json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"foo"}}','f4', 'f6') (result: foo)
with curr_theme (id, theme, updated_at) as (
    select jig_data_module.id               as id,
        jsonb_extract_path_text(contents, 'content', 'theme', 'Override') as theme,
        jig_data_module.updated_at  as updated_at
    from jig_data_module
    inner join jig_data on jig_data.id = jig_data_module.jig_data_id
    where contents -> 'content' -> 'theme' ? 'Override' -- exists override
)
update jig_data_module
set contents = jsonb_set(contents, '{content, theme}', to_jsonb(theme), false),
    updated_at = curr_theme.updated_at
from curr_theme
where curr_theme.id = jig_data_module.id;

-- replace all "{"theme": "Jig"}" with "{"theme": <value>}"
-- '{"a":1,"b":2}'::json->>'b'
with curr (module_id, theme_name, jig_theme, updated_at) as (
    select jig_data_module.id           as module_id,
        theme.display_name as theme_name,
        jig_data.theme as jig_theme,
        jig_data_module.updated_at      as updated_at
    from jig_data_module
    inner join jig_data on jig_data.id = jig_data_module.jig_data_id
    inner join theme on theme.index = jig_data.theme
    where contents -> 'content' ? 'theme' and contents -> 'content' ->> 'theme' = 'Jig'
)
update jig_data_module
set contents = jsonb_set(contents, '{content, theme}', to_jsonb(theme_name), false),
    updated_at = curr.updated_at
from
    curr
where curr.module_id = jig_data_module.id;

select trigger_updated_at('jig_data_module');

drop table if exists theme;
