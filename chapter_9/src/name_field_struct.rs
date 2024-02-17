struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap {pixels, size}
}

// public struct and it's fields
pub struct PubAllGrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}

// public struct with private fields
pub struct PubGrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

// In this game, brooms are monsters. You'll see
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

/// Two possible alternatives for what a `Broom` could be working on
#[derive(Copy, Clone)]
enum BroomIntent {FetchWater, DumpWater}

// Receive the input Broom by value, taking ownership.
fn chop(b:Broom) -> (Broom, Broom) {
    // Initialize `brrom1` mostly from `b`, chqnging only `height`. Since
    // `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
    let mut broom1 = Broom {height: b.height / 2, .. b};

    // Initialize `broom2` mostly from `broom1`. Since `String` is not
    // `Copy`, we must clone `name` explicitly
    let mut broom2 = Broom {name: broom1.name.clone(), ..broom1};

    // Give each fragment a distinct name.
    broom1.name.push_str(" |");
    broom2.name.push_str(" ||");
    return (broom1, broom2);
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_creating_struct () {
        let width = 1024;
        let height = 576;
        let image = GrayscaleMap{
            pixels: vec![0; width * height],
            size: (width, height)
        };
        assert_eq!(image.size, (1024, 576));
        assert_eq!(image.pixels.len(), 1024 * 576);
    }

    fn test_chop_broom() {
        let hockey = Broom {
            name: "Hockey".to_string(),
            height: 60,
            health: 100,
            position: (100.0, 200.0, 0.0),
            intent: BroomIntent::FetchWater
        };
        
        let (hockey1, hockey2) = chop(hockey);
        assert_eq!(hockey1.name, "Hockey |");
        assert_eq!(hockey1.height, 30);
        assert_eq!(hockey1.health, 100);
        
        assert_eq!(hockey2.name, "Hockey ||");
        assert_eq!(hockey2.height, 30);
        assert_eq!(hockey2.health, 100);
    }
}