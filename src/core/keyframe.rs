use crate::{CalQuaternion, CalVector};

pub struct CalCoreKeyframe {
    m_time: f32,
     m_translation: CalVector<f32>,
     m_rotation: CalQuaternion<f32>,
}

impl CalCoreKeyframe {
    pub fn getTranslation(&self) -> &CalVector<f32> {
        &self.m_translation
    }

    pub fn setTranslation(&mut self, val: &CalVector<f32>) {
        self.m_translation = *val;
    }

    pub fn getRotation(&self) -> &CalQuaternion<f32> {
        &self.m_rotation
    }

    pub fn setRotation(&mut self, val: &CalQuaternion<f32>) {
        self.m_rotation = *val;
    }
}
