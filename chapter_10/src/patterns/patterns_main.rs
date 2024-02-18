use super::super::enums::enums_with_data::RoughTime;

fn rough_time_to_english(rt: RoughTime) -> String{
    match rt {
        RoughTime::InThePast(units, count ) => 
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, 1) => 
        format!("a {} from now", units.singular()),
        RoughTime::InTheFuture(units, count) => 
            format!("{} {} from now", count, units.plural()),
    }
}