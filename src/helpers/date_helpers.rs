use chrono::Datelike;


pub fn date_builder() -> String {
    let utc = chrono::Utc::now();
    let year = utc.year();
    let month =  if utc.month() < 10 {
        format!("0{}", utc.month())
    } else {
        utc.month().to_string()
    };
    let day = if utc.day() < 10 {
        format!("0{}", utc.day())
    } else {
        utc.day().to_string()
    };
    format!("{}-{}-{}", year, month, day )
}