use super::plane::Plane;
use crate::CalVector;
use cgmath::{Matrix3, SquareMatrix};

pub struct BoundingBox {
    pub plane: [Plane; 6],
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            plane: [
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
            ],
        }
    }
}

impl BoundingBox {
    /*****************************************************************************/
    /** Computes points of a bounding box.
     *
     * This function computes the 8 points of a bounding box.
     *
     * @param p A pointer to CalVector[8], the 8 points of the bounding box
     *****************************************************************************/
    pub fn ComputePoints(&mut self, p: &mut [CalVector<f32>]) {
        // todo!();
        let mut idx = 0;

        for i in 0..2 {
            for j in 2..4 {
                for k in 4..6 {
                    let x: f32;
                    let y: f32;
                    let z: f32;

                    let m = Matrix3::<f32>::new(
                        self.plane[i].A,
                        self.plane[i].B,
                        self.plane[i].C,
                        self.plane[j].A,
                        self.plane[j].B,
                        self.plane[j].C,
                        self.plane[k].A,
                        self.plane[k].B,
                        self.plane[k].C,
                    );

                    let det = m.determinant();

                    if det != 0.0 {
                        let m = Matrix3::<f32>::new(
                            -self.plane[i].D,
                            self.plane[i].B,
                            self.plane[i].C,
                            -self.plane[j].D,
                            self.plane[j].B,
                            self.plane[j].C,
                            -self.plane[k].D,
                            self.plane[k].B,
                            self.plane[k].C,
                        );

                        let x = m.determinant() / det;

                        let m = Matrix3::<f32>::new(
                            self.plane[i].A,
                            -self.plane[i].D,
                            self.plane[i].C,
                            self.plane[j].A,
                            -self.plane[j].D,
                            self.plane[j].C,
                            self.plane[k].A,
                            -self.plane[k].D,
                            self.plane[k].C,
                        );

                        let y = m.determinant() / det;

                        let m = Matrix3::<f32>::new(
                            self.plane[i].A,
                            self.plane[i].B,
                            -self.plane[i].D,
                            self.plane[j].A,
                            self.plane[j].B,
                            -self.plane[j].D,
                            self.plane[k].A,
                            self.plane[k].B,
                            -self.plane[k].D,
                        );

                        let z = m.determinant() / det;

                        p[idx].x = x;
                        p[idx].y = y;
                        p[idx].z = z;
                    } else {
                        p[idx].x = 0.0;
                        p[idx].y = 0.0;
                        p[idx].z = 0.0;
                    }

                    idx += 1;
                    // p = p[1:];
                }
            }
        }
    }
}
