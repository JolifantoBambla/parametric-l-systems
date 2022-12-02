use std::ops::{Add, AddAssign};
use glam::{Vec2, Vec3};

pub trait Bounds {
    type VecN: Add + AddAssign<Self::VecN> + PartialEq;
    fn min(&self) -> Self::VecN;
    fn max(&self) -> Self::VecN;
    fn contains(&self, point: Self::VecN) -> bool;
    fn grow(&mut self, point: Self::VecN);
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds2 {
    min: Vec2,
    max: Vec2,
}

impl Bounds2 {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min: min.min(max), max: max.max(min) }
    }
}

impl Bounds for Bounds2 {
    type VecN = Vec2;

    fn min(&self) -> Self::VecN {
        self.min
    }
    fn max(&self) -> Self::VecN {
        self.max
    }
    fn contains(&self, point: Self::VecN) -> bool {
        self.min.cmpge(point).all() && self.max.cmple(point).all()
    }
    fn grow(&mut self, point: Self::VecN) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds3 {
    min: Vec3,
    max: Vec3,
}

impl Bounds3 {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min: min.min(max), max: max.max(min) }
    }

    pub fn adjust_xy(&mut self, other: Bounds2) {
        self.min = other.min.extend(self.min.z);
        self.max = other.max.extend(self.max.z);
    }
}

impl Bounds for Bounds3 {
    type VecN = Vec3;
    fn min(&self) -> Vec3 {
        self.min
    }
    fn max(&self) -> Vec3 {
        self.max
    }
    fn contains(&self, point: Self::VecN) -> bool {
        self.min.cmpge(point).all() && self.max.cmple(point).all()
    }
    fn grow(&mut self, point: Self::VecN) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }
}