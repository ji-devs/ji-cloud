insert into locale_bundle (id, created_at, display_name)
values
    ('7c11d04e-7c67-11eb-a0d7-9b5fb8f29de9','2021-03-03 21:29:06.969944+00', 'main page'),
    ('8046692c-7c67-11eb-a0d7-b7924e4289df','2021-03-03 21:29:14.050269+00', 'locale'),
    ('8359a48a-7c67-11eb-a0d7-0fd74777a62c','2021-03-03 21:29:19.209697+00', 'login'),
    ('85a46ffe-7c67-11eb-a0d7-277d94fe130c','2021-03-03 21:29:23.055108+00', 'test 4');



insert into locale_item_kind(id, created_at, display_name)
values
    ('1623eab4-7c68-11eb-a0d7-5f27277abc1b', '2021-03-03 21:33:25.482088+00', 'kind 1'),
    ('17114886-7c68-11eb-a0d7-57627e53e068', '2021-03-03 21:33:27.038138+00', 'kind 2'),
    ('182c77f4-7c68-11eb-a0d7-cbab2eafe435', '2021-03-03 21:33:28.893911+00', 'kind 3'),
    ('18f46e4e-7c68-11eb-a0d7-6351ca8502e6', '2021-03-03 21:33:30.204483+00', 'kind 4'),
    ('1a749ee2-7c68-11eb-a0d7-f3acfa7b44a1', '2021-03-03 21:33:32.722233+00', 'kind 5'),
    ('1b4097e0-7c68-11eb-a0d7-836dd16fe69a', '2021-03-03 21:33:34.058978+00', 'kind 6');


-- unlike with everything else, the sequential ids mean that we can't just insert with the ids, since that doesn't increment the sequence.
-- luckily, ids are deterministic.
insert into locale_entry (bundle_id, section, item_kind_id, english, status, in_app, in_element, in_mock, created_at)
values
    ('7c11d04e-7c67-11eb-a0d7-9b5fb8f29de9', 'A', '1623eab4-7c68-11eb-a0d7-5f27277abc1b', 'Entry #1', 0, true, true, false, '2021-03-03 21:41:17.146015+00'),
    ('8359a48a-7c67-11eb-a0d7-0fd74777a62c', 'A', '1623eab4-7c68-11eb-a0d7-5f27277abc1b', 'Entry #2', 0, true, true, false, '2021-03-03 21:41:33.251747+00'),
    ('7c11d04e-7c67-11eb-a0d7-9b5fb8f29de9', 'A', '1623eab4-7c68-11eb-a0d7-5f27277abc1b', 'Entry #3', 0, true, true, false, '2021-03-03 21:41:36.928568+00'),
    ('7c11d04e-7c67-11eb-a0d7-9b5fb8f29de9', 'B', '1623eab4-7c68-11eb-a0d7-5f27277abc1b', 'Entry #B-1', 0, true, true, false, '2021-03-03 21:41:48.042122+00'),
    ('7c11d04e-7c67-11eb-a0d7-9b5fb8f29de9', 'C', '1623eab4-7c68-11eb-a0d7-5f27277abc1b', 'Entry #B-1', 0, true, true, false, '2021-03-03 21:41:53.888321+00');


