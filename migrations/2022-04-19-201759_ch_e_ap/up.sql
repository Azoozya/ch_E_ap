-- Your SQL goes here
-- Update this section when updating functions prepare_up_table in ~/src/sql/mysql/request
CREATE TABLE `Users` (
  `id` int unsigned NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` varchar(20) COLLATE 'ascii_bin' NOT NULL,
  `pubkey` varchar(64) COLLATE 'ascii_bin' NOT NULL
);

CREATE TABLE `Challenges` (
  `user_id` int unsigned NOT NULL,
  `nonce` int unsigned NULL,
  `expire` bigint unsigned NOT NULL,
  FOREIGN KEY (`user_id`) REFERENCES `Users` (`id`) ON DELETE CASCADE
);

CREATE TABLE `Cookies` (
  `user_id` int unsigned NOT NULL,
  `expire` bigint unsigned NOT NULL,
  FOREIGN KEY (`user_id`) REFERENCES `Users` (`id`) ON DELETE CASCADE
);

CREATE USER 'Register'@'localhost' IDENTIFIED BY 'password';
GRANT SELECT,INSERT ON `cheap` . `Users` TO 'Auth'@'localhost';
GRANT INSERT ON `cheap` . `Challenges` TO 'Auth'@'localhost';
GRANT INSERT ON `cheap` . `Cookies` TO 'Auth'@'localhost';
FLUSH PRIVILEGES;

CREATE USER 'Auth'@'localhost' IDENTIFIED BY 'password';
GRANT SELECT ON `cheap` . `Users` TO 'Auth'@'localhost';
GRANT SELECT,UPDATE ON `cheap` . `Challenges` TO 'Auth'@'localhost';
GRANT UPDATE ON `cheap` . `Cookies` TO 'Auth'@'localhost';
FLUSH PRIVILEGES;

CREATE USER 'MiddleW'@'localhost' IDENTIFIED BY 'password';
--GRANT ON `cheap` . `Users` TO 'Auth'@'localhost';
--GRANT ON `cheap` . `Challenges` TO 'Auth'@'localhost';
GRANT SELECT ON `cheap` . `Cookies` TO 'Auth'@'localhost';
FLUSH PRIVILEGES;

CREATE USER 'Idle'@'localhost' IDENTIFIED BY 'password';
GRANT DELETE ON `cheap` . `Users` TO 'Auth'@'localhost';
--GRANT ON `cheap` . `Challenges` TO 'Auth'@'localhost';
GRANT SELECT ON `cheap` . `Cookies` TO 'Auth'@'localhost';
FLUSH PRIVILEGES;

-- Columns privileges:
--Select	
--Insert
--Update
--References
--Alter routine
--Execute

--GRANT ALL PRIVILEGES ON cheap . * TO 'new_user'@'localhost';
--REVOKE ALL PRIVILEGES ON cheap . * FROM 'new_user'@'localhost';
--FLUSH PRIVILEGES;
--DROP USER new_user@‘localhost’;

DELIMITER //

DROP PROCEDURE IF EXISTS retrieve;
CREATE PROCEDURE retrieve(arg VARCHAR(20))
BEGIN
   IF (SELECT COUNT(*) FROM `Users` WHERE `name`=arg) > 0
   THEN
      SELECT `id` FROM `Users` WHERE `name`=arg;
   ELSE
      SELECT COUNT(`id`) FROM `Users` WHERE `name`=arg;
   END IF;
END 
//
DELIMITER ;