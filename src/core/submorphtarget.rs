use std::cell::RefCell;
use std::rc::Rc;

use crate::CalVector;

use super::submesh::{CalCoreSubmesh, CalMorphTargetType, TextureCoordinate};

struct BlendVertex {
    position: CalVector<f32>,
    normal: CalVector<f32>,
    textureCoords: Vec<TextureCoordinate>,
}

pub struct CalCoreSubMorphTarget {
    _name: String,

    m_vectorBlendVertex: Vec<BlendVertex>,
    m_coreSubmesh: Rc<RefCell<CalCoreSubmesh>>,
    m_morphTargetID: u32,
    m_morphTargetType: CalMorphTargetType,
}
