create schema if not exists api;

do $$
begin
  if not exists (select 1 from pg_roles where rolname = 'authenticator') then
    create role authenticator login password 'authenticator_pw';
  end if;

  if not exists (select 1 from pg_roles where rolname = 'web_anon') then
    create role web_anon nologin;
  end if;
end $$ ;

grant web_anon to authenticator ;

grant usage on schema api to web_anon ;

-- make future api views/tables readable without extra GRANTs
alter default privileges in schema api grant select on tables to web_anon ;
