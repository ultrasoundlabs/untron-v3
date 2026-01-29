/*
Realtor write audit log.

This is intentionally not exposed via PostgREST (which is configured to only expose schema `api`).
*/

create schema if not exists realtor;

create table if not exists realtor.write_action (
    id bigserial primary key,
    created_at timestamptz not null default now(),

    request_id uuid,
    principal_id text,
    remote_ip text,
    user_agent text,

    action text not null,
    method text not null,
    path text not null,
    status_code int not null,
    duration_ms bigint not null,

    error_kind text,
    error_message text,

    request_body jsonb,
    response_body jsonb
);

create index if not exists write_action_created_at_idx
    on realtor.write_action (created_at desc);

create index if not exists write_action_action_idx
    on realtor.write_action (action, created_at desc);

create index if not exists write_action_principal_id_idx
    on realtor.write_action (principal_id, created_at desc);

-- Optional: allow the stack's read-only DB browser role to view audit logs.
do $$
begin
    if exists (select 1 from pg_roles where rolname = 'db_readonly') then
        grant usage on schema realtor to db_readonly;
        grant select on all tables in schema realtor to db_readonly;
        alter default privileges in schema realtor grant select on tables to db_readonly;
    end if;
end $$;
