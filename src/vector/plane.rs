use crate::CalVector;

pub struct Plane {
    pub A: f32,
    pub B: f32,
    pub C: f32,
    pub D: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Plane {
            A: 0.0,
            B: 0.0,
            C: 0.0,
            D: 0.0,
        }
    }
}

impl Plane {
    pub fn Eval(&mut self, p: &CalVector<f32>) -> f32 {
        return p.x * self.A + p.y * self.B + p.z * self.C + self.D;
    }

    pub fn SetPosition(&mut self, p: &CalVector<f32>) {
        self.D = -p.x * self.A - p.y * self.B - p.z * self.C
    }

    pub fn setNormal(&mut self, p: &CalVector<f32>) {
        self.A = p.x;
        self.B = p.y;
        self.C = p.z;
        self.D = -1e32
    }

    pub fn Dist(&self, p: &CalVector<f32>) -> f32 {
        f32::abs(
            (p.x * self.A + p.y * self.B + p.z * self.C + self.D)
                / f32::sqrt(self.A * self.A + self.B * self.B + self.C * self.C),
        )
    }
}
