CREATE DATABASE IF NOT EXISTS fm_staging;
USE fm_staging;

DROP TABLE IF EXISTS files;
CREATE TABLE IF NOT EXISTS files (
	id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	file_name varchar(200) NOT NULL,
	file_location varchar(200) NOT NULL
);

TRUNCATE TABLE files;

DELIMITER $$
DROP PROCEDURE IF EXISTS insert_if_empty;
CREATE PROCEDURE insert_if_empty()
BEGIN
	DECLARE row_count INT;
	SELECT COUNT(*) INTO row_count FROM files;

	IF row_count = 0 THEN	
		INSERT INTO files (id, file_name, file_location) VALUES
		(1, 'file.txt', '/data/fm_files/file.txt'),
		(2, 'users.json', '/data/fm_files/users.json');
	END IF;
END $$

DELIMITER ;

CALL insert_if_empty();

SELECT * FROM files;
