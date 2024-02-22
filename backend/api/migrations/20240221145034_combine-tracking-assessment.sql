alter table jig_code drop column track_assessments;
alter table jig_code rename column display_score to scoring;
alter table jig_data drop column track_assessments;
alter table jig_data rename column display_score to scoring;
