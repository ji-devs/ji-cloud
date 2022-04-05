create function update_learning_path_like() returns trigger
    language plpgsql
as
$$
begin
    update learning_path
    set liked_count = liked_count + 1
    where id = NEW.learning_path_id;
    return NEW;
end;
$$;

create trigger add_learning_path_like
    after insert
    on learning_path_like
    for each row
execute procedure update_learning_path_like();

create function update_learning_path_unlike() returns trigger
    language plpgsql
as
$$
begin
    update learning_path
    set liked_count = liked_count - 1
    where id = OLD.learning_path_id;
    return NULL;
end;
$$;

create trigger learning_path_unlike
    after delete
    on learning_path_like
    for each row
execute procedure update_learning_path_unlike();

create function bump_learning_path_data_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update learning_path_data set updated_at = now() where new.learning_path_data_id = learning_path_data.id or old.learning_path_data_id = learning_path_data.id;
    return null;
end;
$$;

create trigger bump_learning_path_data_updated_at after insert or delete on learning_path_data_age_range for each row execute procedure bump_learning_path_data_updated_at();
create trigger bump_learning_path_data_updated_at after insert or delete on learning_path_data_affiliation for each row execute procedure bump_learning_path_data_updated_at();
create trigger bump_learning_path_data_updated_at after insert or delete on learning_path_data_category for each row execute procedure bump_learning_path_data_updated_at();
create trigger bump_learning_path_data_updated_at after insert or delete on learning_path_data_jig for each row execute procedure bump_learning_path_data_updated_at();








