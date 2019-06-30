#[derive(Copy, Clone, Debug)]
pub struct Vector(pub i32, pub i32);

impl Vector {
    fn inside(self, rect: Rect) -> bool {
        let r = rect.normalized();
        self.0 >= rect.start.0
            && self.1 >= rect.start.1
            && self.0 <= rect.end.0
            && self.1 <= rect.end.1
    }
}

impl core::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl core::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl core::ops::Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}

impl core::ops::Div<i32> for Vector {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Vector(self.0 / rhs, self.1 / rhs)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub start: Vector,
    pub end: Vector,
}

impl Rect {
    pub fn new(start: Vector, end: Vector) -> Rect {
        Rect {
            start: start,
            end: end,
        }
    }

    pub fn new4(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        Rect {
            start: Vector(x1, y1),
            end: Vector(x2, y2),
        }
    }

    pub fn size(self) -> Vector {
        self.end - self.start
    }

    pub fn normalized(self) -> Self {
        let mut ret = self;
        if ret.start.0 > ret.end.0 {
            let end = ret.end.0;
            ret.end.0 = ret.start.0;
            ret.start.0 = end;
        }
        if ret.start.1 > ret.end.1 {
            let end = ret.end.1;
            ret.end.1 = ret.start.1;
            ret.start.1 = end;
        }
        ret
    }

    pub fn clip(self, bounds: Self) -> Self {
        let mut ret = self.normalized();
        let lim = bounds.normalized();
        if ret.start.0 < lim.start.0 {
            ret.start.0 = lim.start.0;
        }
        if ret.start.1 < lim.start.1 {
            ret.start.1 = lim.start.1;
        }
        if ret.end.0 < lim.end.0 {
            ret.end.0 = lim.end.0;
        }
        if ret.end.1 < lim.end.1 {
            ret.end.1 = lim.end.1;
        }
        ret
    }
}

impl core::ops::Add<Vector> for Rect {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}
