/*
Partner billing helpers:
- Derive (principal_id -> leases) from realtor write_action audit logs + indexed hub leases.

Notes:
- This intentionally stays in schema `realtor` (not exposed via PostgREST's `api` schema).
- Time filtering should use `requested_at` (audit write_action.created_at), which reflects when
  the realtor service accepted the request, not onchain inclusion time.
*/

-- =============================================================================
-- INDEXES (support principal/timeframe + join to hub leases)
-- =============================================================================

create index if not exists write_action_create_lease_success_by_principal_created_at
on realtor.write_action (principal_id, created_at desc)
where action = 'create_lease' and status_code = 200 and principal_id is not null;

create index if not exists write_action_create_lease_success_receiver_salt_nukeable_after
on realtor.write_action ((response_body->>'receiver_salt'), (response_body->>'nukeable_after'))
where action = 'create_lease'
  and status_code = 200
  and response_body ? 'receiver_salt'
  and response_body ? 'nukeable_after';

create index if not exists hub_lease_current_by_receiver_salt_nukeable_after
on hub.lease_versions (receiver_salt, nukeable_after)
where valid_to_seq is null;

-- =============================================================================
-- VIEW: principal_id -> lease rows
-- =============================================================================

create or replace view realtor.principal_leases as
select
  wa.created_at as requested_at,
  wa.request_id,
  wa.principal_id,
  lv.lease_id,
  lv.receiver_salt,
  lv.lease_number,
  lv.start_time,
  lv.nukeable_after
from realtor.write_action wa
join hub.lease_versions lv
  on lv.valid_to_seq is null
 and lv.receiver_salt = (wa.response_body->>'receiver_salt')
 and lv.nukeable_after::text = (wa.response_body->>'nukeable_after')
where wa.action = 'create_lease'
  and wa.status_code = 200
  and wa.principal_id is not null
  and wa.response_body ? 'receiver_salt'
  and wa.response_body ? 'nukeable_after';

comment on view realtor.principal_leases is
$$Successful lease creations attributed to an upstream principal_id (typically an API key id), derived from realtor.write_action and hub.lease_versions.$$;

-- Optional: allow the stack's read-only DB browser role to view the derived billing view.
do $$
begin
  if exists (select 1 from pg_roles where rolname = 'db_readonly') then
    grant select on realtor.principal_leases to db_readonly;
  end if;
end $$;

