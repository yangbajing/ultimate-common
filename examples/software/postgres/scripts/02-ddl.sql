set timezone to 'Asia/Chongqing';
--
create user ultimate with nosuperuser replication encrypted password '2024.Ultimate';
create database ultimate owner = ultimate template = template1;
