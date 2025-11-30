pub use crate::{CalQuaternion, CalVector};

pub trait Blend {
    type Item;

    fn blend(&mut self, d: f32, v: &Self::Item);
}

impl Blend for CalVector<f32> {
    type Item = CalVector<f32>;

    fn blend(&mut self, d: f32, v: &CalVector<f32>) {
        self.x += d * (v.x - self.x);
        self.y += d * (v.y - self.y);
        self.z += d * (v.z - self.z);
    }
}

impl Blend for CalQuaternion<f32> {
    type Item = CalQuaternion<f32>;

    fn blend(&mut self, mut d: f32, q: &CalQuaternion<f32>) {
        let mut norm = self.v.x * q.v.x + self.v.y * q.v.y + self.v.z * q.v.z + self.s * q.s;

        let mut bFlip = false;

        if norm < 0.0 {
            norm = -norm;
            bFlip = true;
        }

        let inv_d = if 1.0 - norm < 0.000001 {
            let inv_d = 1.0 - d;
            inv_d
        } else {
            let theta = f32::acos(norm);

            let s = 1.0 / f32::sin(theta);

            let inv_d = f32::sin((1.0 - d) * theta) * s;
            d = f32::sin(d * theta) * s;
            inv_d
        };

        if bFlip {
            d = -d;
        }

        self.v.x = inv_d * self.v.x + d * q.v.x;
        self.v.y = inv_d * self.v.y + d * q.v.y;
        self.v.z = inv_d * self.v.z + d * q.v.z;
        self.s = inv_d * self.s + d * q.s;
    }
}
