-- Add migration script here
CREATE TABLE `tokens` (
  `id` int(11) NOT NULL,
  `token_owner` int(11) NOT NULL,
  `token` varchar(255) NOT NULL,
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;