-- Returns 1 row if api key exists and is related to plot
-- Returns 0 rows if api key does not exist or is not related to plot

SELECT * 
FROM plotsystem_api_keys
INNER JOIN plotsystem_buildteams 
ON plotsystem_api_keys.id = plotsystem_buildteams.api_key_id
INNER JOIN plotsystem_buildteam_has_countries
ON plotsystem_buildteams.id = plotsystem_buildteam_has_countries.buildteam_id
INNER JOIN plotsystem_countries
ON plotsystem_countries.id = plotsystem_buildteam_has_countries.country_id
INNER JOIN plotsystem_city_projects
ON plotsystem_countries.id = plotsystem_city_projects.country_id
INNER JOIN plotsystem_plots
ON plotsystem_city_projects.id = plotsystem_plots.city_project_id
WHERE plotsystem_api_keys.api_key = 'mzNvlteU4ak0QW56MX5ynZu8tUtWu1YL'
AND plotsystem_plots.id = 1;