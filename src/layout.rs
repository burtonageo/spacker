#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Size2d {
    width: u32,
    height: u32
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position2d {
    x: i32,
    y: i32
}

#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct Rect {
    position: Position2d,
    size: Size2d
}

#[derive(Debug, Hash)]
pub struct Canvas {
    dimensions: (u32, u32),
    regions: Vec<Region>
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            dimensions: (width, height),
            regions: Vec::new()
        }
    }

    pub fn add_image(&mut self, region: ImageDimensions) {
        self.regions.push(region)
    }

    pub fn layout(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}

enum Error {
    NoRoom
}

#[derive(Clone, Debug, Default, Hash)]
pub struct ImageDimensions {
    size: Size2d
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        ImageDimensions {
            size: Size2d {
                width: width,
                height: height
            }
        }
    }
}

#[derive(Clone, Debug, Default, Hash)]
struct Region {
    dimensions: Rect
}

impl From<ImageDimensions> for Region {
    fn from(sprite: ImageDimensions) -> Self {
        let dimensions = Rect {
            position: Default::default(),
            size: sprite.size
        };

        Region {
            dimensions: dimensions
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout() {
        let canvas = Canvas::new(100, 100);
        let regions = []
    }
}
