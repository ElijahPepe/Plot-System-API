table! {
    plotsystem_builders (uuid) {
        uuid -> Varchar,
        name -> Varchar,
        score -> Integer,
        completed_plots -> Integer,
        first_slot -> Nullable<Integer>,
        second_slot -> Nullable<Integer>,
        third_slot -> Nullable<Integer>,
    }
}

table! {
    plotsystem_city_projects (id) {
        id -> Integer,
        country_id -> Integer,
        name -> Varchar,
        description -> Varchar,
        visible -> Tinyint,
    }
}

table! {
    plotsystem_countries (id) {
        id -> Integer,
        server_id -> Integer,
        name -> Varchar,
        head_id -> Nullable<Varchar>,
    }
}

table! {
    plotsystem_difficulties (id) {
        id -> Integer,
        name -> Varchar,
        multiplier -> Double,
        score_requirment -> Integer,
    }
}

table! {
    plotsystem_ftp_configurations (id) {
        id -> Integer,
        schematic_path -> Nullable<Varchar>,
        address -> Varchar,
        port -> Integer,
        isSFTP -> Tinyint,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    plotsystem_plots (id) {
        id -> Integer,
        city_project_id -> Integer,
        difficulty_id -> Integer,
        review_id -> Nullable<Integer>,
        owner_uuid -> Nullable<Varchar>,
        member_uuids -> Nullable<Varchar>,
        status -> Enum,
        mc_coordinates -> Varchar,
        score -> Nullable<Integer>,
        last_activity -> Nullable<Datetime>,
        create_date -> Datetime,
        create_player -> Varchar,
        pasted -> Tinyint,
    }
}

table! {
    plotsystem_reviews (id) {
        id -> Integer,
        reviewer_uuid -> Varchar,
        rating -> Varchar,
        feedback -> Varchar,
        review_date -> Datetime,
        sent -> Tinyint,
    }
}

table! {
    plotsystem_servers (id) {
        id -> Integer,
        ftp_configuration_id -> Nullable<Integer>,
        name -> Varchar,
    }
}

joinable!(plotsystem_city_projects -> plotsystem_countries (country_id));
joinable!(plotsystem_countries -> plotsystem_servers (server_id));
joinable!(plotsystem_plots -> plotsystem_builders (owner_uuid));
joinable!(plotsystem_plots -> plotsystem_city_projects (city_project_id));
joinable!(plotsystem_plots -> plotsystem_difficulties (difficulty_id));
joinable!(plotsystem_plots -> plotsystem_reviews (review_id));
joinable!(plotsystem_reviews -> plotsystem_builders (reviewer_uuid));
joinable!(plotsystem_servers -> plotsystem_ftp_configurations (ftp_configuration_id));

allow_tables_to_appear_in_same_query!(
    plotsystem_builders,
    plotsystem_city_projects,
    plotsystem_countries,
    plotsystem_difficulties,
    plotsystem_ftp_configurations,
    plotsystem_plots,
    plotsystem_reviews,
    plotsystem_servers,
);
