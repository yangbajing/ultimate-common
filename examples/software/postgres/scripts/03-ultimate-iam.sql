set timezone to 'Asia/Chongqing';
\c ultimate;
\c - ultimate;
create schema if not exists iam;
--
-- User
create table if not exists iam.user
(
    id     bigserial   not null,
    email  varchar
        constraint user_uk_email unique,
    phone  varchar
        constraint user_uk_phone unique,
    name   varchar,
    status int         not null,
    cid    bigint      not null,
    ctime  timestamptz not null,
    mid    bigint,
    mtime  timestamptz,
    constraint user_pk primary key (id)
);
--
-- UserCredential
create table if not exists iam.user_credential
(
    id            bigint       not null
        constraint user_credential_fk_user references iam.user (id),
    encrypted_pwd varchar(255) not null,
    cid           bigint       not null,
    ctime         timestamptz  not null,
    mid           bigint,
    mtime         timestamptz,
    constraint user_credential_pk primary key (id)
);

-- initial data
------------------
insert into iam."user" (id, email, phone, name, status, cid, ctime)
values (1, 'admin@ultimate.com', null, '超管', 100, 1, current_timestamp),
       (10000, 'user@ultimate.com', '13912345678', '普通用户', 100, 1, current_timestamp);
insert into iam.user_credential (id, encrypted_pwd, cid, ctime)
values (1,
        '#1#$argon2id$v=19$m=19456,t=2,p=1$hAPRw63nW4mdwOd0l0WnmA$wN1i4uYbL+h/FjsaMVae6n93A3LikkqJ4IwiAqr78x0', -- 密码为：2024.Ultimate
        1, current_timestamp);
-- 重置 user_id_seq，使新用户注册从ID为 10001 开始
alter sequence iam.user_id_seq restart 10001;
