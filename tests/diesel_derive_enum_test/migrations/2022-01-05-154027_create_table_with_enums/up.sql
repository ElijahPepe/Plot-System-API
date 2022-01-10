-- Your SQL goes here
CREATE TABLE `table_with_enums` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `enum_field` enum('a','b','c') NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;