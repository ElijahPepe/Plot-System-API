-- Add migration script here
ALTER TABLE `plotsystem_ftp_configurations` CHANGE `schematic_path` `schematics_path` varchar(255);