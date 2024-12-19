/// This file contains rotations in cubing notation. They are abstractions over CubeRotationEvent, and make it easy
/// to implement cubing algorithms.
use super::{rotation::Rotation, CubeRotationEvent};

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
                rotation: Rotation::cube_x(),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotation::X2 => CubeRotationEvent {
                rotation: Rotation::cube_x(),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            CubeRotation::XPrime => CubeRotationEvent {
                rotation: Rotation::cube_x(),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            CubeRotation::Y => CubeRotationEvent {
                rotation: Rotation::cube_y(),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            CubeRotation::Y2 => CubeRotationEvent {
                rotation: Rotation::cube_y(),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            CubeRotation::YPrime => CubeRotationEvent {
                rotation: Rotation::cube_y(),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotation::Z => CubeRotationEvent {
                rotation: Rotation::cube_z(),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotation::Z2 => CubeRotationEvent {
                rotation: Rotation::cube_z(),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            CubeRotation::ZPrime => CubeRotationEvent {
                rotation: Rotation::cube_z(),
                negative_direction: false,
                twice: false,
                animation: None,
            },
        }
    }
}

impl Into<CubeRotationEvent> for &Rotation2x2 {
    fn into(self) -> CubeRotationEvent {
        match self {
            Rotation2x2::L => CubeRotationEvent {
                rotation: Rotation::face_x(-1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation2x2::L2 => CubeRotationEvent {
                rotation: Rotation::face_x(-1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation2x2::LPrime => CubeRotationEvent {
                rotation: Rotation::face_x(-1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation2x2::R => CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation2x2::R2 => CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation2x2::RPrime => CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation2x2::U => CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation2x2::U2 => CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation2x2::UPrime => CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation2x2::D => CubeRotationEvent {
                rotation: Rotation::face_y(-1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation2x2::D2 => CubeRotationEvent {
                rotation: Rotation::face_y(-1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation2x2::DPrime => CubeRotationEvent {
                rotation: Rotation::face_y(-1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation2x2::F => CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation2x2::F2 => CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation2x2::FPrime => CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation2x2::B => CubeRotationEvent {
                rotation: Rotation::face_z(-1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation2x2::B2 => CubeRotationEvent {
                rotation: Rotation::face_z(-1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation2x2::BPrime => CubeRotationEvent {
                rotation: Rotation::face_z(-1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
        }
    }
}

impl Into<CubeRotationEvent> for &Rotation3x3 {
    fn into(self) -> CubeRotationEvent {
        match self {
            Rotation3x3::L => CubeRotationEvent {
                rotation: Rotation::face_x(-1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation3x3::L2 => CubeRotationEvent {
                rotation: Rotation::face_x(-1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation3x3::LPrime => CubeRotationEvent {
                rotation: Rotation::face_x(-1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation3x3::R => CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation3x3::R2 => CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation3x3::RPrime => CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation3x3::U => CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation3x3::U2 => CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation3x3::UPrime => CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation3x3::D => CubeRotationEvent {
                rotation: Rotation::face_y(-1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation3x3::D2 => CubeRotationEvent {
                rotation: Rotation::face_y(-1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation3x3::DPrime => CubeRotationEvent {
                rotation: Rotation::face_y(-1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation3x3::F => CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            Rotation3x3::F2 => CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation3x3::FPrime => CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation3x3::B => CubeRotationEvent {
                rotation: Rotation::face_z(-1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            Rotation3x3::B2 => CubeRotationEvent {
                rotation: Rotation::face_z(-1),
                negative_direction: false,
                twice: true,
                animation: None,
            },
            Rotation3x3::BPrime => CubeRotationEvent {
                rotation: Rotation::face_z(-1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
        }
    }
}
