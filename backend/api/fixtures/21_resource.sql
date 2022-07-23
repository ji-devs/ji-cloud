insert into resource_data (id, display_name, created_at, updated_at, language,
                    last_synced_at, description, privacy_level, draft_or_live, translated_description)
values ('d806771a-1518-11ed-87fa-4785946384aa', 'resource1', '2021-03-04 00:46:26.134651+00', -- live1
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'test description', 0, 1, '{}'::jsonb),
       ('2f8d92d4-1519-11ed-87fa-63728884fdd0', 'resource2', '2021-03-04 00:46:26.134651+00', -- live2
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'test description', 0, 1, '{}'::jsonb),
       ('af827edc-1519-11ed-87fa-a7fcfabe535d', 'resource3', '2021-03-04 00:46:26.134651+00', -- live3
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'test description', 0, 1, '{}'::jsonb),
       ('d8067634-1518-11ed-87fa-cb90cc4fa07d', 'resource1', '2021-03-04 00:46:26.134651+00', -- draft1
        '2021-03-04 00:46:26.134651+00', 'en', null , 'test description', 1, 0, '{}'::jsonb),
       ('2f8d937e-1519-11ed-87fa-033ee1e7150d', 'resource2', '2021-03-06 00:46:26.134651+00', -- draft2
        '2021-03-06 00:46:26.134651+00', 'he', null, 'draft test description', 1, 0, '{}'::jsonb),
       ('af827f86-1519-11ed-87fa-0f83dd32ae96', 'resource3', '2021-03-04 00:46:26.134651+00', -- draft3
        '2021-03-04 00:46:26.134651+00', 'en', null, 'test description', 1, 0, '{}'::jsonb);


insert into resource (id, creator_id, author_id, live_id, draft_id, published_at)
values ('d8067526-1518-11ed-87fa-ebaf880b6d9c', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', 'd806771a-1518-11ed-87fa-4785946384aa',
        'd8067634-1518-11ed-87fa-cb90cc4fa07d', '2022-06-24 17:57:33.149359+00'),
       ('2f8d91d0-1519-11ed-87fa-eb1826fcf343', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', '2f8d92d4-1519-11ed-87fa-63728884fdd0',
        '2f8d937e-1519-11ed-87fa-033ee1e7150d', '2022-06-23 17:57:33.149359+00'),
       ('af827e00-1519-11ed-87fa-7b1aa26c85a8', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', 'af827edc-1519-11ed-87fa-a7fcfabe535d',
        'af827f86-1519-11ed-87fa-0f83dd32ae96', '2022-06-22 17:57:33.149359+00');

insert into resource_data_module (id, resource_data_id, index, kind, is_complete, contents, created_at)
values ('a6b248f8-1dd7-11ec-8426-975953035335', 
        'd806771a-1518-11ed-87fa-4785946384aa', 0, 0, true, '{}', '2021-03-04 00:46:26.134651+00'),    -- live1
       ('cfe8fd74-151c-11ed-b60c-6ba2ae5b9011', 
        'd8067634-1518-11ed-87fa-cb90cc4fa07d', 0, 0, false, '{}', '2021-03-04 00:46:26.134651+00'),    -- draft1
       ('e3378be8-151c-11ed-b60c-971f972f35d8', 
        '2f8d92d4-1519-11ed-87fa-63728884fdd0', 0, 0, true, '{}', '2021-03-04 00:46:26.134651+00'),    -- live2
       ('8fd39a64-151c-11ed-b60c-cf8efe275b73', 
        '2f8d937e-1519-11ed-87fa-033ee1e7150d', 0, 0, false, '{}', '2021-03-04 00:46:26.134651+00'),    -- draft2
       ('a6b24a88-1dd7-11ec-8426-57525d09b22c', 
        'af827f86-1519-11ed-87fa-0f83dd32ae96', 0, 0, false, '{}', '2021-03-04 00:46:26.134651+00');    -- draft3 


insert into resource_data_resource (resource_data_id, id, display_name, resource_type_id, resource_content)
values ('d806771a-1518-11ed-87fa-4785946384aa', '286b828c-1dd9-11ec-8426-571b03b2d3df', 'link test', 'a91aca34-519e-11ec-ab46-175eaaf1ff23',
        '{ "link": "url://url.url.url/urlese" }'),
       ('d806771a-1518-11ed-87fa-4785946384aa', '286b82fa-1dd9-11ec-8426-2b6953011bae', 'link test', 'a91aca34-519e-11ec-ab46-175eaaf1ff23', '{ "link": "url://url.url.url/url" }'),
       ('d8067634-1518-11ed-87fa-cb90cc4fa07d', '286b834a-1dd9-11ec-8426-6f641a50e23f',
        'link test', 'a91aca34-519e-11ec-ab46-175eaaf1ff23', '{ "link": "url://url.url.url/urls/s" }'),
       ('d8067634-1518-11ed-87fa-cb90cc4fa07d', '286b8390-1dd9-11ec-8426-fbeb80c504d9', 'link test', 'a91aca34-519e-11ec-ab46-175eaaf1ff23', '{ "link": "url://url.url.url/url/s" }');
