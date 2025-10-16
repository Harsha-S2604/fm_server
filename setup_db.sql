CREATE DATABASE IF NOT EXISTS fm_staging;
USE fm_staging;

DROP TABLE IF EXISTS files;
CREATE TABLE IF NOT EXISTS files (
	id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name varchar(200) NOT NULL,
	location varchar(200) NOT NULL,
	f_type varchar(200) NOT NULL
);

TRUNCATE TABLE files;

DELIMITER $$
DROP PROCEDURE IF EXISTS insert_if_empty;
CREATE PROCEDURE insert_if_empty()
BEGIN
	DECLARE row_count INT;
	SELECT COUNT(*) INTO row_count FROM files;

	IF row_count = 0 THEN	
		INSERT INTO files (id, name, location, f_type) VALUES
		(1, 'file.txt', '/data/fm_files/file.txt', 'txt'),
		(2, 'users.json', '/data/fm_files/users.json', 'json');
	END IF;
END $$

DELIMITER ;

CALL insert_if_empty();

SELECT * FROM files;
