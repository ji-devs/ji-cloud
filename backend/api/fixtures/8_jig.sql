insert into module (id, created_at, kind)
values
 ('0cbfdd82-7c83-11eb-9f77-d7d86264c3bc', '2021-03-04 00:46:26.134651+00',0), 
 ('0cc03a02-7c83-11eb-9f77-f77f9ad65e9a', '2021-03-04 00:46:26.134651+00', null); 

insert into jig (id, creator_id, author_id, created_at)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00');


insert into jig_module (jig_id, module_id, index)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cbfdd82-7c83-11eb-9f77-d7d86264c3bc',  0),
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cc03a02-7c83-11eb-9f77-f77f9ad65e9a',  1);
