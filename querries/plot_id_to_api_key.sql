-- Returns 1 row if api key exists and is related to plot
-- Returns 0 rows if api key does not exist or is not related to plot

SELECT * 
FROM plotsystem_api_keys
INNER JOIN plotsystem_buildteams 
ON plotsystem_api_keys.api_key = plotsystem_buildteams.api_key
INNER JOIN plotsystem_buildteam_has_countries
ON plotsystem_buildteams.id = plotsystem_buildteam_has_countries.buildteam_id
INNER JOIN plotsystem_countries
ON plotsystem_buildteam_has_countries.country_id = plotsystem_countries.id
INNER JOIN plotsystem_city_projects
ON plotsystem_city_projects.country_id = plotsystem_countries.id
INNER JOIN plotsystem_plots
ON plotsystem_plots.city_project_id = plotsystem_city_projects.id
WHERE plotsystem_api_keys.api_key = 'hokahsdföachöslasdfhlschlösdaflh'
AND plotsystem_plots.id = 1