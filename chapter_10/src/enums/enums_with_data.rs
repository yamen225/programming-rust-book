use super::enums_main::TimeUnit;

/// A timestamp that has been deliberatly rounded off, so our program
/// says "6 months ago" instead of "February 9, 2016, at 9:49 AM"
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

struct Point3D {
    x: u32,
    y: u32,
    z: u32
}

enum Shape {
    Sphere {center: Point3D, radius:f32},
    Cuboid {corner1: Point3D, corner2: Point3D}
}

struct DifferentialEquation{
    a7a: String
}

struct EarlyModernistPoem{
    hey_jude: String
}

enum RelationShipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_enums_with_data() {
        let four_score_and_seven_years_ago = 
            RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);

        let three_hours_from_now_on = 
            RoughTime::InTheFuture(TimeUnit::Hours, 3);

    }
}