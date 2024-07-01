create table subscriptions (
  id blob primary key, -- UUID
  email text not null unique,
  name text not null,
  subscribed_at text not null
) strict;
