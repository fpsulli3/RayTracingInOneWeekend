use std::ops;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn dot(&self, other: &Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y*other.z - self.z*other.y,
            y: self.x*other.z - self.z*other.x,
            z: self.x*other.y - self.y*other.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        return self.dot(self);
    }

    pub fn length(&self) -> f32 {
        return self.dot(self).sqrt();
    }

    pub fn normalized(&self) -> Self {
        return self / self.length();
    }

    pub fn normalize(&mut self)  {
        let l = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}
impl ops::Div<f32> for &mut Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        return &self.origin + t*&self.direction;
    }
}
