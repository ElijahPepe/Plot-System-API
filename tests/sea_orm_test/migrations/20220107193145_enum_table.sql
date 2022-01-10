-- Add migration script here

CREATE TABLE `mylimplies` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `status` enum('unclaimed','unfinished','unreviewed','completed') NOT NULL DEFAULT 'unclaimed',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;