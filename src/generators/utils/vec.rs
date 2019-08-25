
/**
 * A euclidean (geometric) vector of N dimensions.
 */
pub type Vector = Vec<f32>;

pub trait VectorMath {
    fn x(&mut self) -> &mut f32;
    fn y(&mut self) -> &mut f32;
    fn z(&mut self) -> &mut f32;
    fn magnitude(&self) -> f32;
    fn magnitude_squared(&self) -> f32;
    fn scale(&mut self, factor: f32);
    fn normalize(&mut self);
    fn normalized(&self) -> Self;
    fn angle_to(&self, other: &Vec<f32>) -> f32;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> f32;
}

impl VectorMath for Vector {

    fn x(&mut self) -> &mut f32 {
        &mut self[0]
    }

    fn y(&mut self) -> &mut f32 {
        &mut self[1]
    }
    
    fn z(&mut self) -> &mut f32 {
        &mut self[2]
    }

    fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    fn magnitude_squared(&self) -> f32 {
        let mut sum = 0.0;

        for i in 0..self.len() {
            sum += self[i] * self[i];
        }

        return sum;
    }

    fn scale(&mut self, factor: f32) {
        for i in 0..self.len() {
            self[i] *= factor;
        }
    }

    fn normalize(&mut self) {
        let mag = self.magnitude();
        for i in 0..self.len() {
            self[i] /= mag;
        }
    }

    fn normalized(&self) -> Self {
        let mut result = self.clone();
        result.normalize();
        return result;
    }

    fn angle_to(&self, other: &Self) -> f32 {
        self.mul(other) / (self.magnitude() * other.magnitude())
    }

    fn add(&self, other: &Self) -> Self {
        let mut res = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            res.push(self[i] + other[i]);
        }
        
        return res;
    }

    fn sub(&self, other: &Self) -> Self {
        let mut res = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            res.push(self[i] - other[i]);
        }
        
        return res;
    }

    // (dot-product)
    fn mul(&self, other: &Self) -> f32 {
        let mut dot = 0.0;

        for i in 0..self.len() {
            dot += self[i] * other[i];
        }

        return dot;
    }

}

#[test]
fn test_dot() {
    let vec1 = vec![1.0, 3.0, -5.0];
    let vec2 = vec![4.0, -2.0, -1.0];

    assert_eq!(vec1.mul(&vec2), 3.0);
}

/*
// tests
#[test]
fn test_ops() {
    let vec1 = Vector { x: 1.0, y: 3.0, z: -5.0 };
    let vec2 = Vector { x: 4.0, y: -2.0, z: -1.0 };

    assert_eq!(&vec1 + &vec2, Vector{ x: 5.0, y: 1.0, z: -6.0 });
    assert_eq!(&vec1 * &vec2, 3.0);
}*/