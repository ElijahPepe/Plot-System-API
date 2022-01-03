pub struct FtpConfiguration {
    id: i64,
    schematic_path: String,
    address: String,
    port: i64,
    username: String,
    password: String,
}

pub struct Country {
    id: i64,
    server_id: i64,
    name: String,
    head_id: String,
}

pub struct Server {
    id: i64,
    ftp_configuration_id: i64,
    name: String,
}

pub struct CityProject {
    id: i64,
    country_id: i64,
    name: String,
    description: String,
    visible: i8,
}

pub struct Builder {
    uuid: String,
    name: String,
    score: i64,
    completed_plots: i64,
    first_slot: i64,
    second_slot: i64,
    third_slot: i64,
}

pub struct Reviews {
    id: i64,
    reviewer_uuid: String,
    rating: String,
    feedback: String,
    review_date: dateTimeWhaah,
    sent: i8,
}
