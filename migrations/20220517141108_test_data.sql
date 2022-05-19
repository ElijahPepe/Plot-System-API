-- Add migration script here
INSERT INTO `plotsystem_ftp_configurations` (
    `id`,
    `schematic_path`,
    `address`,
    `port`,
    `isSFTP`,
    `username`,
    `password`
  )
VALUES (
    1,
    './schem',
    '0.0.0.1',
    22,
    0,
    'user1',
    'password'
  ),
  (
    2,
    './schematics',
    '127.0.0.1',
    21,
    1,
    'user2',
    '12345678'
  ),
  (
    3,
    './bte/plotsystem/plot_schematics',
    '192.45.42.2',
    23,
    0,
    'user3',
    'abcdefgh'
  );
INSERT INTO `plotsystem_servers` (`id`, `ftp_configuration_id`, `name`)
VALUES (1, 1, 'BTE-Alps-1'),
  (2, 2, 'America01'),
  (3, 2, 'America02');
INSERT INTO `plotsystem_countries` (
    `id`,
    `server_id`,
    `name`,
    `head_id`,
    `continent`
  )
VALUES (1, 1, 'Austria', '0123456789', 'Europe'),
  (2, 1, 'Switzerland', '123456789', 'Europe'),
  (3, 1, 'liechtenstain', '1112312131', 'Europe'),
  (4, 2, 'Mexico', '1234275681', 'North America'),
  (
    5,
    3,
    'California',
    '1233781923',
    'North America'
  );
INSERT INTO `plotsystem_api_keys` (`id`, `api_key`, `created_at`)
VALUES (
    1,
    'mzNvlteU4ak0QW56MX5ynZu8tUtWu1YL',
    '2020-05-17 14:11:08'
  ),
  (
    2,
    'LWZPajV2S3Cd4aCLpyz4Kl2or88sGCR4',
    '2022-06-12 15:01:51'
  );
INSERT INTO `plotsystem_buildteams` (`id`, `name`, `api_key_id`)
VALUES (1, 'BTE ALPS', 1),
  (2, 'BTE AMERICA', 2);
INSERT INTO `plotsystem_buildteam_has_countries` (`id`, `country_id`, `buildteam_id`)
VALUES (1, 1, 1),
  (2, 2, 1),
  (3, 3, 1),
  (4, 4, 2),
  (5, 5, 2);
INSERT INTO `plotsystem_city_projects` (
    `id`,
    `country_id`,
    `name`,
    `description`,
    `visible`
  )
VALUES (1, 1, 'Vienna', 'Vienna Cityproject', 1),
  (2, 2, 'Zürch', 'Zürch Cityproject', 1),
  (3, 3, 'Vaduz', 'Vaduz Cityproject', 1),
  (4, 4, 'IDK', 'sth in Mexico Cityproject', 0),
  (
    5,
    5,
    'San Francisco',
    'San Francisco Cityproject',
    1
  );
INSERT INTO `plotsystem_builders` (
    `uuid`,
    `name`,
    `score`,
    `completed_plots`,
    `first_slot_id`,
    `second_slot_id`,
    `third_slot_id`,
    `lang`
  )
VALUES (
    '000dc68c-4b9e-4e4f-bc18-99aaaa35ef29',
    'Virber',
    100,
    1,
    NULL,
    NULL,
    NULL,
    'DE'
  ),
  (
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    'JO_KIL',
    242,
    4,
    NULL,
    NULL,
    NULL,
    'EN'
  ),
  (
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    'waggiswagen',
    1,
    0,
    NULL,
    NULL,
    NULL,
    'FR'
  );
INSERT INTO `plotsystem_builder_is_reviewer` (`id`, `builder_uuid`, `buildteam_id`)
VALUES (1, '02539287-17bf-4c10-9086-76d4f8b9db29', 1),
  (2, '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4', 2);
INSERT INTO `plotsystem_reviews` (
    `id`,
    `reviewer_uuid`,
    `rating`,
    `feedback`,
    `review_date`,
    `sent`
  )
VALUES (
    1,
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    '5,5,5,5',
    'Krasser Build',
    '2020-05-17 14:11:08',
    1
  ),
  (
    2,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '1,1,1,1',
    'Finished like all my other builds',
    '2020-05-17 14:11:08',
    0
  ),
  (
    3,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '5,5,5,5',
    'Me when the building',
    '2020-05-17 14:11:08',
    1
  );
INSERT INTO `plotsystem_difficulties` (`id`, `name`, `multiplier`, `score_requirement`)
VALUES (1, 'Easy', 1, 0),
  (2, 'Normal', 2, 100),
  (3, 'Hard', 3, 200),
  (4, 'Extreme', 4, 300);
INSERT INTO `plotsystem_plots` (
    `id`,
    `city_project_id`,
    `difficulty_id`,
    `create_player`,
    `owner_uuid`,
    `review_id`,
    `member_uuids`,
    `status`,
    `mc_coordinates`,
    `score`,
    `last_activity`,
    `create_date`,
    `pasted`,
    `outline`
  )
VALUES (
    1,
    2,
    1,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    1,
    NULL,
    'finished',
    'idfk',
    255,
    '2022-05-18 09:52:38.000000',
    '2022-05-18 09:52:38.000000',
    1,
    'asfasfasfasfafasfasfasf'
  ),
  (
    2,
    1,
    3,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    1,
    NULL,
    'finished',
    'idfk',
    432,
    '2020-05-17 14:11:08',
    '2020-05-17 14:11:08',
    0,
    'jaksdhiopi a8u039r8p azshoij'
  ),
  (
    3,
    3,
    4,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    NULL,
    NULL,
    'finished',
    'idfk',
    59,
    '2020-05-17 14:11:08',
    '2020-05-17 14:11:08',
    0,
    'jaksdhiopi a8u039r8p azshoij'
  ),
  (
    4,
    2,
    2,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    NULL,
    NULL,
    'unfinished',
    'idfk',
    NULL,
    '2020-05-17 14:11:08',
    '2020-05-17 14:11:08',
    0,
    'jaksdhiopi a8u039r8p azshoij'
  ),
  (
    5,
    4,
    3,
    '014cc2e4-eac2-4bc9-9a79-9215a7abbdf4',
    '02539287-17bf-4c10-9086-76d4f8b9db29',
    NULL,
    NULL,
    'unreviewed',
    'idfk',
    NULL,
    '2020-05-17 14:11:08',
    '2020-05-17 14:11:08',
    0,
    'jaksdhiopi a8u039r8p azshoij'
  );