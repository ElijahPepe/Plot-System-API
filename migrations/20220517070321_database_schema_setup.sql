-- Add migration script here
CREATE TABLE `plotsystem_ftp_configurations` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `schematic_path` VARCHAR(255),
  `address` VARCHAR(255) NOT NULL,
  `port` INT NOT NULL,
  `isSFTP` TINYINT(1) NOT NULL,
  `username` VARCHAR(255) NOT NULL,
  `password` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id`)
);
CREATE TABLE `plotsystem_servers` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `ftp_configuration_id` INT,
  `name` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`ftp_configuration_id`) REFERENCES `plotsystem_ftp_configurations`(`id`)
);
CREATE TABLE `plotsystem_countries` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `server_id` INT NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `head_id` VARCHAR(10),
  `continent` ENUM(
    'Europe',
    'Asia',
    'Africa',
    'Oceania',
    'South America',
    'North America'
  ) NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`server_id`) REFERENCES `plotsystem_servers`(`id`)
);
CREATE TABLE `plotsystem_api_keys` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `api_key` VARCHAR(32) NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
);
CREATE TABLE `plotsystem_buildteams` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NOT NULL,
  `api_key_id` INT NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`api_key_id`) REFERENCES `plotsystem_api_keys`(`id`)
);
CREATE TABLE `plotsystem_buildteam_has_countries` (
  `id` INT NOT NULL,
  `country_id` INT NOT NULL,
  `buildteam_id` INT NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`country_id`) REFERENCES `plotsystem_countries`(`id`),
  FOREIGN KEY (`buildteam_id`) REFERENCES `plotsystem_buildteams`(`id`)
);
CREATE TABLE `plotsystem_city_projects` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `country_id` INT NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `description` VARCHAR(255) NOT NULL,
  `visible` TINYINT(1) NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`country_id`) REFERENCES `plotsystem_countries`(`id`)
);
CREATE TABLE `plotsystem_builders` (
  `uuid` VARCHAR(36) NOT NULL,
  `name` VARCHAR(16) NOT NULL,
  `score` INT NOT NULL,
  `completed_plots` INT NOT NULL,
  `first_slot_id` INT,
  `second_slot_id` INT,
  `third_slot_id` INT,
  `lang` VARCHAR(5),
  PRIMARY KEY (`uuid`)
);
CREATE TABLE `plotsystem_builder_is_reviewer` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `builder_uuid` VARCHAR(36) NOT NULL,
  `buildteam_id` INT NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`builder_uuid`) REFERENCES `plotsystem_builders`(`uuid`),
  FOREIGN KEY (`buildteam_id`) REFERENCES `plotsystem_buildteams`(`id`)
);
CREATE TABLE `plotsystem_reviews` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `reviewer_uuid` VARCHAR(36) NOT NULL,
  `rating` VARCHAR(45) NOT NULL,
  `feedback` VARCHAR(420) NOT NULL,
  `review_date` DATETIME NOT NULL,
  `sent` TINYINT(1) NOT NULL,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`reviewer_uuid`) REFERENCES `plotsystem_builders`(`uuid`)
);
CREATE TABLE `plotsystem_difficulties` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NOT NULL,
  `multiplier` DOUBLE NOT NULL,
  `score_requirement` INT NOT NULL,
  PRIMARY KEY (`id`)
);
CREATE TABLE `plotsystem_plots` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `city_project_id` INT NOT NULL,
  `difficulty_id` INT NOT NULL,
  `create_player` VARCHAR(36) NOT NULL,
  `owner_uuid` VARCHAR(36),
  `review_id` INT,
  `member_uuids` VARCHAR(110),
  `status` ENUM(
    'unclaimed',
    'unfinished',
    'unreviewed',
    'finished',
    'completed'
  ) NOT NULL,
  `mc_coordinates` VARCHAR(255) NOT NULL,
  `score` INT,
  `last_activity` DATETIME,
  `create_date` DATETIME NOT NULL,
  `pasted` TINYINT(1) NOT NULL,
  `outline` LONGTEXT,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`city_project_id`) REFERENCES `plotsystem_city_projects`(`id`),
  FOREIGN KEY (`difficulty_id`) REFERENCES `plotsystem_difficulties`(`id`),
  FOREIGN KEY (`create_player`) REFERENCES `plotsystem_builders`(`uuid`),
  FOREIGN KEY (`owner_uuid`) REFERENCES `plotsystem_builders`(`uuid`),
  FOREIGN KEY (`review_id`) REFERENCES `plotsystem_reviews`(`id`)
);
ALTER TABLE `plotsystem_builders`
ADD FOREIGN KEY (`first_slot_id`) REFERENCES `plotsystem_plots`(`id`),
  ADD FOREIGN KEY (`second_slot_id`) REFERENCES `plotsystem_plots`(`id`),
  ADD FOREIGN KEY (`third_slot_id`) REFERENCES `plotsystem_plots`(`id`);