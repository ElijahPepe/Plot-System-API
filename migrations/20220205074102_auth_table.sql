-- Add migration script here
CREATE TABLE `api_keys` (
  `api_key` varchar(32) NOT NULL PRIMARY KEY,
  `name` varchar(45) NOT NULL UNIQUE,
  `country_id` int(11) NOT NULL UNIQUE,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (`country_id`) REFERENCES plotsystem_countries(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;