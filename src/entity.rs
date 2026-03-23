use once_cell::sync::Lazy;

use crate::{Float, UVec2, Vec3};

pub const EARTH: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: 399,
        name: "EARTH".into(),
        frame: "IAU_EARTH".into(),
        label: "".into(),
    },
    radii: Vec3::new(6378136.6, 6378136.6, 6356751.0),
    orbit_period: 365.25 * 86400.0,
    spin_period: 0.0,
});

pub const MOON: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: 301,
        name: "MOON".to_string(),
        frame: "IAU_MOON".to_string(),
        label: "".to_string(),
    },
    radii: Vec3::new(1738.1, 1738.1, 1736.0) * 1e3,
    orbit_period: 29.5 * 86400.0,
    spin_period: 29.5 * 86400.0,
});

pub const MARS: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: 499,
        name: "MARS".to_string(),
        frame: "IAU_MARS".to_string(),
        label: "".to_string(),
    },
    radii: Vec3::new(3396.19, 3396.19, 3376.2) * 1e3,
    orbit_period: 687.0 * 86400.0,
    spin_period: 0.0,
});

pub const PHOBOS: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: 401,
        name: "PHOBOS".to_string(),
        frame: "IAU_PHOBOS".to_string(),
        label: "".to_string(),
    },
    radii: Vec3::new(13.0, 11.4, 9.1) * 1e3,
    orbit_period: 7.0 * 3600.0 + 39.0 * 60.0,
    spin_period: 7.0 * 3600.0 + 39.0 * 60.0,
});

pub const DEIMOS: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: 402,
        name: "DEIMOS".to_string(),
        frame: "IAU_DEIMOS".to_string(),
        label: "".to_string(),
    },
    radii: Vec3::new(7.8, 6.0, 5.1) * 1e3,
    orbit_period: 30.312 * 3600.0,
    spin_period: 30.312 * 3600.0,
});

pub const DIDYMOS: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: -658030,
        name: "DIDYMOS".to_string(),
        frame: "DIDYMOS_FIXED".to_string(),
        label: "".to_string(),
    },
    radii: Vec3::new(409.5, 400.5, 302.5),
    orbit_period: 700.0 * 86400.0,
    spin_period: 2.26 * 3600.0,
});

pub const DIMORPHOS: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: -658031,
        name: "DIMORPHOS".to_string(),
        frame: "DIMORPHOS_FIXED".to_string(),
        label: "post_impact".to_string(),
    },
    radii: Vec3::new(88.5, 84.0, 57.0),
    orbit_period: 11.3676 * 3600.0,
    spin_period: 11.3676 * 3600.0,
});

pub const DIMORPHOS_PRE: Lazy<Body> = Lazy::new(|| Body {
    entity: Entity {
        id: -658031,
        name: "DIMORPHOS".to_string(),
        frame: "DIMORPHOS_FIXED".to_string(),
        label: "pre_impact".to_string(),
    },
    radii: Vec3::new(88.5, 84.0, 57.0),
    orbit_period: 11.921473 * 3600.0,
    spin_period: 11.921473 * 3600.0,
});

pub const TIRI: Lazy<Camera> = Lazy::new(|| Camera {
    entity: Entity {
        id: -91200,
        name: "HERA_TIRI".to_string(),
        frame: "HERA_TIRI".to_string(),
        label: "".to_string(),
    },
    px: UVec2::new(1024, 768),
    fovy: 10.0,
    filters: vec![
        "CLOSE".to_string(),
        "Filter a (7.8um)".to_string(),
        "Filter b (8.6um)".to_string(),
        "Filter c (9.6um)".to_string(),
        "Filter d (10.6um)".to_string(),
        "Filter e (11.6um)".to_string(),
        "Filter f (13.0um)".to_string(),
        "Filter g (wide)".to_string(),
    ],
});

pub const AFC: Lazy<Camera> = Lazy::new(|| Camera {
    entity: Entity {
        id: -91110,
        name: "HERA_AFC-1".to_string(),
        frame: "HERA_AFC-1".to_string(),
        label: "".to_string(),
    },
    px: UVec2::new(1024, 1024),
    fovy: 5.47,
    filters: vec![],
});

pub const HERA: Lazy<Spacecraft> = Lazy::new(|| Spacecraft {
    entity: Entity {
        id: -91000,
        name: "HERA_SPACECRAFT".to_string(),
        frame: "HERA_SPACECRAFT".to_string(),
        label: "".to_string(),
    },
    id_cameras: vec!["HERA_TIRI".into(), "HERA_AFC-1".into()],
});

pub const HALCA: Lazy<Spacecraft> = Lazy::new(|| Spacecraft {
    entity: Entity {
        id: -58,
        name: "HALCA".to_string(),
        frame: "".to_string(),
        label: "PHOBOS2".to_string(),
    },
    id_cameras: vec![],
});

pub const MEX: Lazy<Spacecraft> = Lazy::new(|| Spacecraft {
    entity: Entity {
        id: 0,
        name: "MEX_SPACECRAFT".to_string(),
        frame: "MEX_SPACECRAFT".to_string(),
        label: "".to_string(),
    },
    id_cameras: vec![],
});

pub const TGO: Lazy<Spacecraft> = Lazy::new(|| Spacecraft {
    entity: Entity {
        id: 0,
        name: "TGO_SPACECRAFT".to_string(),
        frame: "TGO_SPACECRAFT".to_string(),
        label: "".to_string(),
    },
    id_cameras: vec![],
});

#[derive(Debug, Clone)]
pub enum EntityKind {
    Body(Body),
    Camera(Camera),
    Spacecraft(Spacecraft),
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: isize,
    pub name: String,
    pub frame: String,
    pub label: String,
}

impl Entity {
    pub const fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            frame: String::new(),
            label: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct Body {
    pub entity: Entity,
    pub radii: Vec3,
    pub orbit_period: Float,
    pub spin_period: Float,
}

impl Body {
    pub const fn new() -> Self {
        Self {
            entity: Entity::new(),
            radii: Vec3::ZERO,
            orbit_period: 0.0,
            spin_period: 0.0,
        }
    }

    pub fn radius(&self) -> Float {
        self.radii.element_sum() / 3.0
    }

    pub fn diameter(&self) -> Float {
        self.radius() * 2.0
    }

    pub fn flattening_radius(&self) -> Float {
        crate::math::flattening_radius(&self.radii)
    }
}

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Body(id={}, name={}, frame={}, label={}, radii={}, orbit_period={}, spin_period={})",
            self.entity.id,
            self.entity.name,
            self.entity.frame,
            self.entity.label,
            self.radii,
            self.orbit_period,
            self.spin_period
        )
    }
}

#[derive(Clone)]
pub struct Camera {
    pub entity: Entity,
    pub px: UVec2,
    pub fovy: Float, // in degree
    pub filters: Vec<String>,
}

impl Camera {
    pub const fn new() -> Self {
        Self {
            entity: Entity::new(),
            px: UVec2::ZERO,
            fovy: 0.0,
            filters: vec![],
        }
    }

    pub fn npx(&self) -> usize {
        self.px.element_product()
    }
}

impl std::fmt::Debug for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Camera(id={}, name={}, frame={}, label={}, px={}, fovy={}, filters={:?})",
            self.entity.id,
            self.entity.name,
            self.entity.frame,
            self.entity.label,
            self.px,
            self.fovy,
            self.filters,
        )
    }
}

#[derive(Clone)]
pub struct Spacecraft {
    pub entity: Entity,
    pub id_cameras: Vec<String>,
}

impl Spacecraft {
    pub const fn new() -> Self {
        Self {
            entity: Entity::new(),
            id_cameras: vec![],
        }
    }
}

impl std::fmt::Debug for Spacecraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        for (ii, id) in self.id_cameras.iter().enumerate() {
            if ii > 0 {
                s += ", ";
            }
            s += &format!("{}", id);
        }
        write!(
            f,
            "Spacecraft(id={}, name={}, frame={}, label={}, id_cameras=[{}])",
            self.entity.id, self.entity.name, self.entity.frame, self.entity.label, s
        )
    }
}
