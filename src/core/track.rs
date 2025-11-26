use std::cell::RefCell;
use std::rc::Rc;

use super::keyframe::CalCoreKeyframe;
use super::skeleton::CalCoreSkeleton;

pub struct CalCoreTrack {
    // /// The index of the associated CoreBone in the CoreSkeleton.
    m_coreBoneId: i32,

    // // If translationRequired is false, then the translations are the same as the
    // // skeleton's translations.
    m_translationRequired: bool,
    m_highRangeRequired: bool,
    m_translationIsDynamic: bool,
    // static int m_translationRequiredCount;
    // static int m_translationNotRequiredCount;

    // /// List of keyframes, always sorted by time.
    m_keyframes: Vec<Rc<CalCoreKeyframe>>,
}

impl CalCoreTrack {
    pub fn new(
        m_coreBoneId: i32,
        m_translationRequired: bool,
        m_highRangeRequired: bool,
        m_translationIsDynamic: bool,
        m_keyframes: Vec<Rc<CalCoreKeyframe>>,
    ) -> Self {
        CalCoreTrack {
            m_coreBoneId,
            m_translationRequired,
            m_highRangeRequired,
            m_translationIsDynamic,
            m_keyframes,
        }
    }

    // 73
    pub fn addCoreKeyframe(&mut self, pCoreKeyframe: CalCoreKeyframe) -> bool {
        self.m_keyframes.push(Rc::new(pCoreKeyframe));
        let mut idx = self.m_keyframes.len() - 1;
        while idx > 0 && self.m_keyframes[idx].getTime() < self.m_keyframes[idx - 1].getTime() {
            self.m_keyframes.swap(idx, idx - 1);
            idx -= 1;
        }
        return true;
    }

    //226
    pub fn compress(
        &self,
        translationTolerance: f64,
        rotationToleranceDegrees: f64,
        skel: &Rc<RefCell<CalCoreSkeleton>>,
    ) {
        todo!();
    }

    //344
    pub fn collapseSequences(&self, translationTolerance: f64, rotationToleranceDegrees: f64) {
        todo!();
    }

    // 615 cpp
    pub fn getCoreKeyframeCount(&self) -> usize {
        self.m_keyframes.len()
    }

    // 620 cpp
    pub fn getCoreKeyframe(&self, idx: usize) -> Option<&Rc<CalCoreKeyframe>> {
        self.m_keyframes.get(idx)
    }
}
