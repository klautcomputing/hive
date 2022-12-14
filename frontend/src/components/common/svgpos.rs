#[derive(PartialEq, Clone, Debug)]
pub struct SvgPos {
    pub pos: (f32, f32),
    pub size: f32,
}

impl SvgPos {
    pub fn new(x: i8, y: i8) -> Self {
        Self {
            pos: (x as f32, y as f32),
            size: 31.5,
        }
    }

    pub fn center_offset(i: usize) -> (f32, f32) {
        (-3.0 * i as f32, -5.0 * i as f32)
    }

    pub fn center(&self) -> (f32, f32) {
        let p = self.pos;
        let h = 2.0 * self.size;
        let w = (3.0 as f32).sqrt() * self.size;
        return if (p.1 as i32).rem_euclid(2) == 0 {
            // even
            (p.0 * w, p.1 * 0.75 * h)
        } else {
            (0.5 * w + p.0 * w, p.1 * 0.75 * h)
            // odd
        };
    }

    pub fn center_with_offset(&self, center_offset: (f32, f32)) -> (f32, f32) {
        let center = self.center();
        (center.0 + center_offset.0, center.1 + center_offset.1)
    }
}

