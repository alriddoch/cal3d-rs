use std::cell::RefCell;
use std::rc::Rc;
use crate::{CalQuaternion, CalVector};
use super::keyframe::CalCoreKeyframe;
use super::skeleton::CalCoreSkeleton;

pub struct CalCoreTrack {
    // /// The index of the associated CoreBone in the CoreSkeleton.
    m_coreBoneId: usize,

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
        m_coreBoneId: usize,
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

    pub fn getCoreBoneId(&self) -> usize {
        self.m_coreBoneId
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

    // 485 cpp
     /*****************************************************************************/
/** Returns a specified state.
  *
  * This function returns the state (translation and rotation of the core bone)
  * for the specified time and duration.
  *
  * @param time The time in seconds at which the state should be returned.
  * @param translation A reference to the translation reference that will be
  *                    filled with the specified state.
  * @param rotation A reference to the rotation reference that will be filled
  *                 with the specified state.
  *
  * @return One of the following values:
  *         \li \b true if successful
  *         \li \b false if an error happened
  *****************************************************************************/

pub fn getState(&self, time: f32 /* , CalVector& translation, CalQuaternion& rotation */) -> (CalVector<f32>, CalQuaternion<f32>) {

  // get the keyframe after the requested time
  let iteratorCoreKeyframeAfter = getUpperBound(time);

  // check if the time is after the last keyframe
  if(iteratorCoreKeyframeAfter == self.m_keyframes.end()) {
    // return the last keyframe state
    --iteratorCoreKeyframeAfter;
    rotation = (*iteratorCoreKeyframeAfter)->getRotation();
    translation = (*iteratorCoreKeyframeAfter)->getTranslation();

    return true;
  }

  // check if the time is before the first keyframe
  if(iteratorCoreKeyframeAfter == self.m_keyframes.begin())  {
    // return the first keyframe state
    rotation = (*iteratorCoreKeyframeAfter)->getRotation();
    translation = (*iteratorCoreKeyframeAfter)->getTranslation();

    return true;
  }

  // get the keyframe before the requested one
let  iteratorCoreKeyframeBefore = iteratorCoreKeyframeAfter;
  --iteratorCoreKeyframeBefore;

  // get the two keyframe pointers
  CalCoreKeyframe *pCoreKeyframeBefore;
  pCoreKeyframeBefore = *iteratorCoreKeyframeBefore;
  CalCoreKeyframe *pCoreKeyframeAfter;
  pCoreKeyframeAfter = *iteratorCoreKeyframeAfter;

  // calculate the blending factor between the two keyframe states
  float blendFactor;
  blendFactor = (time - pCoreKeyframeBefore->getTime()) / (pCoreKeyframeAfter->getTime() - pCoreKeyframeBefore->getTime());

  // blend between the two keyframes
  translation = pCoreKeyframeBefore->getTranslation();
  translation.blend(blendFactor, pCoreKeyframeAfter->getTranslation());

  rotation = pCoreKeyframeBefore->getRotation();
  rotation.blend(blendFactor, pCoreKeyframeAfter->getRotation());

  return true;
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
