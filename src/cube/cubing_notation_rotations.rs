/// This file contains rotations in cubing notation. They are abstraction over CubeRotationEvent, and make it easy
/// to implement cubing algorithms.
use super::{
    rotation::{FaceRotation, Rotation},
    CubeRotationEvent,
};

/// TODO document directions fo reach variant
/// Cube rotations in cubing notation.
pub enum CubeRotation {
    X,
    X2,
    XPrime,
    Y,
    Y2,
    YPrime,
    Z,
    Z2,
    ZPrime,
}

/// Rotations for a 2x2 cube in cubing notation.
pub enum Rotation2x2 {
    /// Left. When looking at the front of the cube, the front row ends up at the bottom.
    L,
    /// Left twice.
    L2,
    /// Left counter clockwise. When looking at the front of the cube, the front row ends up at the top.
    LPrime,
    /// Right. When looking at the front of the cube, the front row ends up at the top.
    R,
    /// Right twice.
    R2,
    /// Right counter clockwise. When looking at the front of the cube, the front row ends up at the bottom.
    RPrime,
    U,
    U2,
    UPrime,
    D,
    D2,
    DPrime,
    F,
    F2,
    FPrime,
    B,
    B2,
    BPrime,
}

/// TODO add middle slice rotations
/// TODO add wide rotations
/// Rotations for a 3x3 cube in cubing notation.
pub enum Rotation3x3 {
    /// Left. When looking at the front of the cube, the front row ends up at the bottom.
    L,
    /// Left twice.
    L2,
    /// Left counter clockwise. When looking at the front of the cube, the front row ends up at the top.
    LPrime,
    /// Right. When looking at the front of the cube, the front row ends up at the top.
    R,
    /// Right twice.
    R2,
    /// Right counter clockwise. When looking at the front of the cube, the front row ends up at the bottom.
    RPrime,
    U,
    U2,
    UPrime,
    D,
    D2,
    DPrime,
    F,
    F2,
    FPrime,
    B,
    B2,
    BPrime,
}

impl Into<CubeRotationEvent> for &CubeRotation {
    fn into(self) -> CubeRotationEvent {
        match self {
            CubeRotation::X => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::X),
                negative_direction: true,
                twice: false,
            },
            CubeRotation::X2 => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::X),
                negative_direction: false,
                twice: true,
            },
            CubeRotation::XPrime => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::X),
                negative_direction: false,
                twice: false,
            },
            CubeRotation::Y => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::Y),
                negative_direction: false,
                twice: false,
            },
            CubeRotation::Y2 => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::Y),
                negative_direction: false,
                twice: true,
            },
            CubeRotation::YPrime => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::Y),
                negative_direction: true,
                twice: false,
            },
            CubeRotation::Z => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::Z),
                negative_direction: true,
                twice: false,
            },
            CubeRotation::Z2 => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::Z),
                negative_direction: false,
                twice: true,
            },
            CubeRotation::ZPrime => CubeRotationEvent {
                rotation: Rotation::Cube(super::rotation::CubeRotation::Z),
                negative_direction: false,
                twice: false,
            },
        }
    }
}

impl Into<CubeRotationEvent> for &Rotation2x2 {
    fn into(self) -> CubeRotationEvent {
        match self {
            Rotation2x2::L => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![-1])),
                negative_direction: false,
                twice: false,
            },
            Rotation2x2::L2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![-1])),
                negative_direction: false,
                twice: true,
            },
            Rotation2x2::LPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![-1])),
                negative_direction: true,
                twice: false,
            },
            Rotation2x2::R => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![1])),
                negative_direction: true,
                twice: false,
            },
            Rotation2x2::R2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![1])),
                negative_direction: false,
                twice: true,
            },
            Rotation2x2::RPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![1])),
                negative_direction: false,
                twice: false,
            },
            Rotation2x2::U => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![1])),
                negative_direction: true,
                twice: false,
            },
            Rotation2x2::U2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![1])),
                negative_direction: false,
                twice: true,
            },
            Rotation2x2::UPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![1])),
                negative_direction: false,
                twice: false,
            },
            Rotation2x2::D => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
                negative_direction: false,
                twice: false,
            },
            Rotation2x2::D2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
                negative_direction: false,
                twice: true,
            },
            Rotation2x2::DPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
                negative_direction: true,
                twice: false,
            },
            Rotation2x2::F => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![1])),
                negative_direction: true,
                twice: false,
            },
            Rotation2x2::F2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![1])),
                negative_direction: false,
                twice: true,
            },
            Rotation2x2::FPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![1])),
                negative_direction: false,
                twice: false,
            },
            Rotation2x2::B => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![-1])),
                negative_direction: false,
                twice: false,
            },
            Rotation2x2::B2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![-1])),
                negative_direction: false,
                twice: true,
            },
            Rotation2x2::BPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![-1])),
                negative_direction: true,
                twice: false,
            },
        }
    }
}

impl Into<CubeRotationEvent> for &Rotation3x3 {
    fn into(self) -> CubeRotationEvent {
        match self {
            Rotation3x3::L => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![-1])),
                negative_direction: false,
                twice: false,
            },
            Rotation3x3::L2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![-1])),
                negative_direction: false,
                twice: true,
            },
            Rotation3x3::LPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![-1])),
                negative_direction: true,
                twice: false,
            },
            Rotation3x3::R => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![1])),
                negative_direction: true,
                twice: false,
            },
            Rotation3x3::R2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![1])),
                negative_direction: false,
                twice: true,
            },
            Rotation3x3::RPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::X(vec![1])),
                negative_direction: false,
                twice: false,
            },
            Rotation3x3::U => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![1])),
                negative_direction: true,
                twice: false,
            },
            Rotation3x3::U2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![1])),
                negative_direction: false,
                twice: true,
            },
            Rotation3x3::UPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![1])),
                negative_direction: false,
                twice: false,
            },
            Rotation3x3::D => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
                negative_direction: false,
                twice: false,
            },
            Rotation3x3::D2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
                negative_direction: false,
                twice: true,
            },
            Rotation3x3::DPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
                negative_direction: true,
                twice: false,
            },
            Rotation3x3::F => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![1])),
                negative_direction: true,
                twice: false,
            },
            Rotation3x3::F2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![1])),
                negative_direction: false,
                twice: true,
            },
            Rotation3x3::FPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![1])),
                negative_direction: false,
                twice: false,
            },
            Rotation3x3::B => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![-1])),
                negative_direction: false,
                twice: false,
            },
            Rotation3x3::B2 => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![-1])),
                negative_direction: false,
                twice: true,
            },
            Rotation3x3::BPrime => CubeRotationEvent {
                rotation: Rotation::Face(FaceRotation::Z(vec![-1])),
                negative_direction: true,
                twice: false,
            },
        }
    }
}
