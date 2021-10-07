create table jig_play_count
(
    jig_id     uuid primary key references jig (id) on delete cascade,
    play_count bigint not null default 0
);

insert into jig_play_count (jig_id)
select id
from jig;
