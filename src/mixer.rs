use crate::animation::CompositionFunction;
use crate::core::{CalCoreAnimation, CalCoreKeyframe, CalCoreModel};
use crate::{CalAnimation, CalAnimationAction, CalAnimationCycle};
use crate::{CalModel, CalSkeleton};
use crate::{CalQuaternion, CalVector};
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

pub trait CalMixerTrait {
    /*****************************************************************************/
    /**
    	* Is the object an instance of the default mixer (i.e. an instance of CalMixer) ?
    	*
    	* @return \li \b true if an instance of CalMixer
    	*         \li \b false if not an instance of CalMixer
    	*
    	*****************************************************************************/
    fn isDefaultMixer(&self) -> bool {
        false
    }

    /*****************************************************************************/
    /**
    	* Notifies the instance that updateAnimation was last called
    	* deltaTime seconds ago. The internal scheduler of the instance
    	* should terminate animations or update the timing information of
    	* active animations accordingly. It should not blend animations
    	* together or otherwise modify the CalModel associated to these
    	* animations.
    	*
    	* The CalModel::update method will call updateSkeleton immediately
    	* after updateAnimation if the instance was allocated by
    	* CalModel::create (in which case it is a CalMixer instance) or if
    	* the instance was set via CalModel::setAbstractMixer.
    	*
    	* @param deltaTime The elapsed time in seconds since the last call.
    	*
    	*****************************************************************************/
    fn updateAnimation(&mut self, deltaTime: f32);

    /*****************************************************************************/
    /**
    	* Updates the skeleton of the corresponding CalModel (as provided to
    	* the create method) to match the current animation state (as
    	* updated by the last call to updateAnimation).  The tracks of each
    	* active animation are blended to compute the position and
    	* orientation of each bone of the skeleton. The updateAnimation
    	* method should be called just before calling updateSkeleton to
    	* define the set of active animations.
    	*
    	* The CalModel::update method will call updateSkeleton immediately
    	* after updateAnimation if the instance was allocated by
    	* CalModel::create (in which case it is a CalMixer instance) or if
    	* the instance was set via CalModel::setAbstractMixer.
    	*
    	*****************************************************************************/
    fn updateSkeleton(&self, model: &CalModel);
}

pub enum CalAbstractMixer {
    None,
    CalMixer(CalMixer),
}

impl CalMixerTrait for CalAbstractMixer {
    fn isDefaultMixer(&self) -> bool {
        match self {
            CalAbstractMixer::CalMixer(mixer) => mixer.isDefaultMixer(),
            _ => false,
        }
    }

    fn updateAnimation(&mut self, deltaTime: f32) {
        match self {
            CalAbstractMixer::CalMixer(mixer) => mixer.updateAnimation(deltaTime),
            _ => {}
        }
    }
    fn updateSkeleton(&self, model: &CalModel) {
        match self {
            CalAbstractMixer::CalMixer(mixer) => mixer.updateSkeleton(model),
            _ => {}
        }
    }
}

const FlagPosRot: u32 = 1;
const FlagMeshScale: u32 = 2;

#[derive(Copy, Clone)]
struct BoneAdjustment {
    // What parts of the adjustment are to be applied?
    pub flags_: u32,
    // Relative to the parent frame of reference.
    pub localPos_: CalVector<f32>,
    pub localOri_: CalQuaternion<f32>,
    // Scales X, Y, and Z of mesh by these parameters.  The scale parameters are with
    // respect to the absolute coordinate space, e.g., Z is up in 3dMax, as opposed
    // to the local coordinate space of the bone.
    pub meshScaleAbsolute_: CalVector<f32>,
    // The adjustment is a highest priority "replace" animation for the bone.  Lower priority
    // animations for the bone, including other replace animations, will be attenuated by 1 - rampValue.
    pub rampValue_: f32,
}

impl Default for BoneAdjustment {
    fn default() -> Self {
        Self {
            flags_: 0,
            localPos_: CalVector::new(0.0, 0.0, 0.0),
            localOri_: CalQuaternion::new(0.0, 1.0, 0.0, 0.0),
            meshScaleAbsolute_: CalVector::new(0.0, 0.0, 0.0),
            rampValue_: 0.0,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct BoneAdjustmentAndBoneId {
    pub boneAdjustment_: BoneAdjustment,
    pub boneId_: usize,
}

const CalMixerBoneAdjustmentsMax: usize = 20;

pub struct CalMixer {
    m_numBoneAdjustments: usize,
    m_boneAdjustmentAndBoneIdArray: [BoneAdjustmentAndBoneId; CalMixerBoneAdjustmentsMax],
    // virtual void applyBoneAdjustments();
    m_pModel: Rc<RefCell<CalModel>>,
    m_vectorAnimation: Vec<CalAnimation>, /* std::vector<CalAnimation *> */
    m_listAnimationAction: Vec<Rc<RefCell<CalAnimationAction>>>,
    m_listAnimationCycle: Vec<Rc<RefCell<CalAnimationCycle>>>,
    m_animationTime: f32,
    m_animationDuration: f32,
    m_timeFactor: f32,
}

impl CalMixer {
    pub fn new(m_pModel: Rc<RefCell<CalModel>>) -> Self {
        let coreAnimationCount = m_pModel
            .borrow()
            .getCoreModel()
            .borrow()
            .getCoreAnimationCount();

        let mut vector_animation = Vec::with_capacity(coreAnimationCount);

        for i in 0..coreAnimationCount {
            vector_animation.insert(i, CalAnimation::None)
        }

        CalMixer {
            m_numBoneAdjustments: 0,
            m_boneAdjustmentAndBoneIdArray: [BoneAdjustmentAndBoneId::default();
                CalMixerBoneAdjustmentsMax],
            m_pModel,
            m_vectorAnimation: vector_animation,
            m_listAnimationAction: Vec::new(),
            m_listAnimationCycle: Vec::new(),
            m_animationTime: 0.0,
            m_animationDuration: 0.0,
            m_timeFactor: 0.0,
        }
    }

    // 599 cpp
    pub fn blendCycle(
        &mut self,
        core_model: &CalCoreModel,
        id: usize,
        weight: f32,
        delay: f32,
    ) -> bool {
        // get the animation for the given id, with range check
        let Some(pAnimation) = self.m_vectorAnimation.get(id) else {
            return false;
        };

        // create a new animation instance if it is not active yet
        match pAnimation {
            CalAnimation::None => {
                // take the fast way out if we are trying to clear an inactive animation
                if weight == 0.0 {
                    return true;
                }

                // These need to be borrowed for the lifetime of pCoreAnimation below

                // get the core animation
                let Some(pCoreAnimation) = core_model.getCoreAnimation(id) else {
                    return false;
                };

                // Ensure that the animation's first and last key frame match for proper
                // looping.
                addExtraKeyframeForLoopedAnim(&pCoreAnimation.borrow());

                // allocate a new animation cycle instance
                let pAnimationCycle =
                    Rc::new(RefCell::new(CalAnimationCycle::new(pCoreAnimation.clone())));

                // insert new animation into the tables
                self.m_vectorAnimation
                    .insert(id, CalAnimation::Cycle(pAnimationCycle.clone()));
                self.m_listAnimationCycle.push(pAnimationCycle.clone());

                // blend the animation
                pAnimationCycle.borrow_mut().blend(weight, delay)
            }
            CalAnimation::Cycle(pAnimationCycle) => {
                // blend the animation cycle
                pAnimationCycle.borrow_mut().blend(weight, delay);
                pAnimationCycle.borrow().checkCallbacks(0.0, &self.m_pModel);

                // clear the animation cycle from the active vector if the target weight is zero
                if weight == 0.0 {
                    self.m_vectorAnimation.insert(id, CalAnimation::None);
                }

                true
            }
            _ => false,
        }
    }

    // 679 cpp
    /*****************************************************************************/
    /** Fades an animation cycle out.
     *
     * This function fades an animation cycle out in a given amount of time.
     *
     * @param id The ID of the animation cycle that should be faded out.
     * @param delay The time in seconds until the the animation cycle is
     *              completely removed.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn clearCycle(&mut self, id: usize, delay: f32) -> bool {
        // get the animation for the given id

        let Some(pAnimation) = self.m_vectorAnimation.get(id) else {
            // CalError::setLastError(CalError::INVALID_HANDLE, __FILE__, __LINE__);
            return false;
        };

        match pAnimation {
            CalAnimation::None => true,
            CalAnimation::Cycle(cycle_ref) => {
                let mut pAnimationCycle = cycle_ref.borrow_mut();

                // set animation cycle to async state
                pAnimationCycle.setAsync(self.m_animationTime, self.m_animationDuration);

                // blend the animation cycle
                pAnimationCycle.blend(0.0, delay);
                pAnimationCycle.checkCallbacks(0.0, &self.m_pModel);

                drop(pAnimationCycle);

                // clear the animation cycle from the active vector
                self.m_vectorAnimation.insert(id, CalAnimation::None);

                true
            }
            _ => {
                // CalError::setLastError(CalError::INVALID_ANIMATION_TYPE, __FILE__, __LINE__);
                false
            }
        }
    }

    // 946 cpp
    fn applyBoneAdjustments(&self, skeleton: &CalSkeleton) {
        let vectorBone = skeleton.getVectorBone();
        for i in 0..self.m_numBoneAdjustments {
            let ba = &self.m_boneAdjustmentAndBoneIdArray[i];
            let bo_ref = &vectorBone[ba.boneId_];
            let mut bone = bo_ref.borrow_mut();

            if ba.boneAdjustment_.flags_ & FlagMeshScale == FlagMeshScale {
                bone.setMeshScaleAbsolute(&ba.boneAdjustment_.meshScaleAbsolute_);
            }
            let cbo = bone.getCoreBone();
            if ba.boneAdjustment_.flags_ & FlagPosRot == FlagPosRot {
                let adjustedLocalPos = *cbo.borrow().getTranslation();
                let adjustedLocalOri = ba.boneAdjustment_.localOri_;
                let scale = 1.0;
                let rampValue = ba.boneAdjustment_.rampValue_;
                let replace = true;
                let unrampedWeight = 1.0;
                bone.blendState(
                    unrampedWeight,
                    &adjustedLocalPos,
                    &adjustedLocalOri,
                    scale,
                    replace,
                    rampValue,
                    true,
                );
            }
        }
    }
}

fn addExtraKeyframeForLoopedAnim(pCoreAnimation: &CalCoreAnimation) {
    let core_animation_duration = pCoreAnimation.getDuration();
    let listCoreTrack = pCoreAnimation.getListCoreTrack();

    let Some(core_track) = listCoreTrack.first().map(|r| r.borrow()) else {
        return;
    };

    let Some(lastKeyframe_time) = core_track
        .getCoreKeyframe(core_track.getCoreKeyframeCount() - 1)
        .map(|frame| frame.getTime())
    else {
        return;
    };

    drop(core_track);

    if lastKeyframe_time < core_animation_duration {
        for coreTrack in listCoreTrack.iter() {
            let mut core_track_mut = coreTrack.borrow_mut();
            let Some(firstKeyframe) = core_track_mut.getCoreKeyframe(0) else {
                dbg!("Core track has no keyframes");
                continue;
            };
            let newKeyframe = CalCoreKeyframe::new(
                core_animation_duration,
                firstKeyframe.getTranslation().clone(),
                firstKeyframe.getRotation().clone(),
            );

            // newKeyframe.setTranslation(firstKeyframe.getTranslation());
            // newKeyframe.setRotation(firstKeyframe.getRotation());
            // newKeyframe.setTime(pCoreAnimation.getDuration());

            core_track_mut.addCoreKeyframe(newKeyframe);
        }
    }
}

impl CalMixerTrait for CalMixer {
    fn isDefaultMixer(&self) -> bool {
        true
    }

    // 846 cpp
    fn updateAnimation(&mut self, deltaTime: f32) {
        use crate::animation::State;

        // update the current animation time
        if self.m_animationDuration == 0.0 {
            self.m_animationTime = 0.0;
        } else {
            self.m_animationTime += deltaTime * self.m_timeFactor;
            if self.m_animationTime >= self.m_animationDuration || self.m_animationTime < 0.0 {
                self.m_animationTime = self.m_animationTime % self.m_animationDuration;
            }
            if self.m_animationTime < 0.0 {
                self.m_animationTime += self.m_animationDuration;
            }
        }

        // update all active animation actions of this model
        self.m_listAnimationAction.retain_mut(|action| {
            let mut animation_action = action.borrow_mut();
            if animation_action.update(deltaTime) {
                animation_action.checkCallbacks(animation_action.getTime(), &self.m_pModel);
                true
            } else {
                // animation action has ended, destroy and remove it from the animation list
                animation_action.completeCallbacks(&self.m_pModel);

                false
            }
        });

        // let iteratorAnimationAction = self.m_listAnimationAction.iter().peekable();

        // while let Some(action) = iteratorAnimationAction.peek() {
        //     let animation_action = action.borrow_mut();
        //     // update and check if animation action is still active
        //     if animation_action.update(deltaTime) {
        //         animation_action.checkCallbacks(animation_action.getTime(), self.m_pModel);
        //         iteratorAnimationAction.next();
        //     } else {
        //         // animation action has ended, destroy and remove it from the animation list
        //         animation_action.completeCallbacks(self.m_pModel);
        //         delete(*iteratorAnimationAction);
        //         iteratorAnimationAction = self.m_listAnimationAction.erase(iteratorAnimationAction);
        //     }
        // }

        // todo: update all active animation poses of this model

        // update the weight of all active animation cycles of this model
        let mut accumulatedWeight = 0.0;
        let mut accumulatedDuration = 0.0;

        self.m_listAnimationCycle.retain_mut(|cycle| {
            let mut animation_cycle = cycle.borrow_mut();
            // update and check if animation cycle is still active
            if animation_cycle.update(deltaTime) {
                // check if it is in sync. if yes, update accumulated weight and duration
                if matches!(animation_cycle.getState(), State::STATE_SYNC) {
                    accumulatedWeight += animation_cycle.getWeight();
                    accumulatedDuration += animation_cycle.getWeight()
                        * animation_cycle.getCoreAnimation().borrow().getDuration();
                }

                animation_cycle.checkCallbacks(self.m_animationTime, &self.m_pModel);
                true
            } else {
                // animation cycle has ended, destroy and remove it from the animation list
                animation_cycle.completeCallbacks(&self.m_pModel);
                false
            }
        });

        // adjust the global animation cycle duration
        if accumulatedWeight > 0.0 {
            self.m_animationDuration = accumulatedDuration / accumulatedWeight;
        } else {
            self.m_animationDuration = 0.0;
        }
    }

    // 1035 cpp
    // model is needed, although we have self.m_pModel, as it's already borrowed.
    fn updateSkeleton(&self, model: &CalModel) {
        // get the skeleton we need to update
        let pSkeleton = model.getSkeleton();

        // clear the skeleton state
        pSkeleton.borrow_mut().clearState();

        let mut skeleton = pSkeleton.borrow_mut();

        // get the bone vector of the skeleton
        let vectorBone = skeleton.getVectorBone();

        // For each bone, reset the transform-related variables to the core (bind pose) bone position and orientation.
        for bone in vectorBone.iter() {
            bone.borrow_mut().setCoreTransformStateVariables();
        }

        // The bone adjustments are "replace" so they have to go first, giving them
        // highest priority and full influence.  Subsequent animations affecting the same bones,
        // including subsequent replace animations, will have their incluence attenuated appropriately.
        // We need to pass in skeleton reference, as it's already borrowed here.
        self.applyBoneAdjustments(skeleton.deref());

        // loop through all animation actions
        for pAction in self.m_listAnimationAction.iter() {
            let action = pAction.borrow();
            if action.isOn() {
                // get the core animation instance
                let pCoreAnimation = action.getCoreAnimation();

                let core_animation = pCoreAnimation.borrow();
                // get the list of core tracks of above core animation
                let listCoreTrack = core_animation.getListCoreTrack();

                // loop through all core tracks of the core animation
                for pTrack in listCoreTrack.iter() {
                    // get the appropriate bone of the track
                    let pBone = &vectorBone[pTrack.borrow().getCoreBoneId()];

                    // get the current translation and rotation
                    // CalVector translation;
                    // CalQuaternion rotation;
                    let (translation, rotation) = pTrack.borrow().getState(action.getTime());

                    // Replace and CrossFade both blend with the replace function.
                    let compFunc = action.getCompositionFunction();
                    let replace =
                        !matches!(compFunc, CompositionFunction::CompositionFunctionAverage)
                            && !matches!(compFunc, CompositionFunction::CompositionFunctionNull);

                    let scale = action.getScale();

                    let track = pTrack.borrow();
                    let absoluteTrans = track.getTranslationRequired();
                    pBone.borrow_mut().blendState(
                        action.getWeight(),
                        &translation,
                        &rotation,
                        scale,
                        replace,
                        action.getRampValue(),
                        absoluteTrans,
                    );
                }
            }
        }

        // lock the skeleton state
        skeleton.lockState();

        // loop through all animation cycles
        for iteratorAnimationCycle in self.m_listAnimationCycle.iter() {
            let pAnimCycle = iteratorAnimationCycle.borrow();

            // get the core animation instance
            let pCoreAnimation = pAnimCycle.getCoreAnimation();

            // calculate adjusted time
            let animationTime;
            if matches!(pAnimCycle.getState(), crate::animation::State::STATE_SYNC) {
                if self.m_animationDuration == 0.0 {
                    animationTime = 0.0;
                } else {
                    animationTime = self.m_animationTime * pCoreAnimation.borrow().getDuration()
                        / self.m_animationDuration;
                }
            } else {
                animationTime = pAnimCycle.getTime();
            }

            let core_animation = pCoreAnimation.borrow();
            // get the list of core tracks of above core animation
            let listCoreTrack = core_animation.getListCoreTrack();

            // loop through all core tracks of the core animation
            for iteratorCoreTrack in listCoreTrack.iter() {
                let pTrack = iteratorCoreTrack.borrow();

                // get the appropriate bone of the track
                let pBone = &vectorBone[pTrack.getCoreBoneId()];

                // get the current translation and rotation
                // CalVector translation;
                // CalQuaternion rotation;
                let (translation, rotation) = pTrack.getState(animationTime);

                // blend the bone state with the new state
                let absoluteTrans = pTrack.getTranslationRequired();
                pBone.borrow_mut().blendState(
                    pAnimCycle.getWeight(),
                    &translation,
                    &rotation,
                    1.0,
                    false,
                    1.0,
                    absoluteTrans,
                );
            }
        }

        // lock the skeleton state
        skeleton.lockState();

        // let the skeleton calculate its final state
        skeleton.calculateState();
    }
}
