CREATE DATABASE IF NOT EXISTS fm_staging;
USE fm_staging;

DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
	id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	email varchar(50) NOT NULL UNIQUE,
	password varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
	id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	user_id BIGINT UNSIGNED NOT NULL,
	name varchar(200) NOT NULL,
	location varchar(200) NOT NULL,
	f_type varchar(200) NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO users (id, email, password) VALUES
		(1, 'abc@gmail.com', "0f4dd6c67bc8c827a2b181bc763f9ab96166d8f50840fe1ae0bbc0e77464da2c"),
		(2, 'john@gmail.com', "d9ede7c2ece01b059ec3af2f8b47836a6feda626cf0a76e49c6c9be2b151456d");

INSERT INTO files (id, name, location, f_type, user_id) VALUES
		(1, 'file.txt', '/data/fm_files/file.txt', 'txt', 1),
		(2, 'users.json', '/data/fm_files/users.json', 'json', 2);

SELECT * FROM files;
