#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Animation {
    steps: Vec<AnimationSteps>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct AnimationSteps {
    collisions: Vec<CollisionBox>,
    frames: Vec<AnimationFrame>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum CollisionType {
    Normal,
    Attack,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct CollisionBox {
    collision_type: CollisionType,
    coordinates: [i16; 4],
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct AnimationFrame {
    pub group: u16,
    pub image: u16,
    pub offset: (i16, i16),
    pub ticks: Option<u16>,
    pub flip: (bool, bool),
}

impl Animation {
    pub fn new(steps: Vec<AnimationSteps>) -> Animation {
        Animation {
            steps,
        }
    }
    pub fn steps(&self) -> &[AnimationSteps] {
        &self.steps[..]
    }
}

impl AnimationSteps {
    pub fn new(collisions: Vec<CollisionBox>, frames: Vec<AnimationFrame>) -> AnimationSteps {
        AnimationSteps {
            collisions,
            frames,
        }
    }
    #[allow(dead_code)]
    pub fn collisions(&self) -> &[CollisionBox] {
        &self.collisions[..]
    }
    pub fn frames(&self) -> &[AnimationFrame] {
        &self.frames[..]
    }
}

impl CollisionBox {
    pub fn new(collision_type: CollisionType, coordinates: [i16; 4]) -> CollisionBox {
        CollisionBox {
            collision_type,
            coordinates,
        }
    }
}
