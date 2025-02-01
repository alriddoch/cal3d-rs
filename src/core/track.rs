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

    //226
    pub fn compress(&self, translationTolerance: f64, rotationToleranceDegrees: f64, skel: &Rc<CalCoreSkeleton>) {
        todo!();
    }

    //344
    pub fn collapseSequences(&self, translationTolerance: f64, rotationToleranceDegrees: f64) {
        todo!();
    }
}
