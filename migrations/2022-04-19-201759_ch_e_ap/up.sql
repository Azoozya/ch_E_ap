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