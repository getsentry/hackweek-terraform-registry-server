DROP TABLE IF EXISTS module;

CREATE TABLE module
(
    module_id serial primary key,    
    namespace varchar(64) not null,
    name varchar(64) not null,
    system varchar(64) not null,
    version varchar(64) not null,
    path varchar(4096) not null
);
