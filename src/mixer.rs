use crate::CalModel;
use crate::core::{CalCoreAnimation, CalCoreKeyframe};
use crate::{CalAnimation, CalAnimationAction, CalAnimationCycle};
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
    fn updateSkeleton(&self);
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
    fn updateSkeleton(&self) {
        match self {
            CalAbstractMixer::CalMixer(mixer) => mixer.updateSkeleton(),
            _ => {}
        }
    }
}

pub struct CalMixer {
    m_numBoneAdjustments: u32,
    // BoneAdjustmentAndBoneId m_boneAdjustmentAndBoneIdArray[CalMixerBoneAdjustmentsMax];
    // virtual void applyBoneAdjustments();
    m_pModel: Rc<RefCell<CalModel>>,
    /* std::vector<CalAnimation *> */ m_vectorAnimation: Vec<CalAnimation>,
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
            m_pModel,
            m_vectorAnimation: vector_animation,
            m_listAnimationAction: Vec::new(),
            m_listAnimationCycle: Vec::new(),
            m_animationTime: 0.0,
            m_animationDuration: 0.0,
            m_timeFactor: 0.0,
        }
    }

    pub fn blendCycle(&mut self, id: usize, weight: f32, delay: f32) -> bool {
        // get the animation for the given id, with range check
        let Some(pAnimation) = self.m_vectorAnimation.get(id) else {
            return false;
        };

        // create a new animation instance if it is not active yet
        if matches!(pAnimation, CalAnimation::None) {
            // take the fast way out if we are trying to clear an inactive animation
            if weight == 0.0 {
                return true;
            }

            // These need to be borrowed for the lifetime of pCoreAnimation below
            let model_scope = self.m_pModel.borrow();
            let core_model_scope = model_scope.getCoreModel().borrow();

            // get the core animation
            let Some(pCoreAnimation) = core_model_scope.getCoreAnimation(id) else {
                return false;
            };

            // Ensure that the animation's first and last key frame match for proper
            // looping.
            addExtraKeyframeForLoopedAnim(&pCoreAnimation.borrow());

            // allocate a new animation cycle instance
            let pAnimationCycle =
                Rc::new(RefCell::new(CalAnimationCycle::new(pCoreAnimation.clone())));

            drop(core_model_scope);
            drop(model_scope);

            // insert new animation into the tables
            self.m_vectorAnimation
                .insert(id, CalAnimation::Cycle(pAnimationCycle.clone()));
            self.m_listAnimationCycle.push(pAnimationCycle.clone());

            // blend the animation
            return pAnimationCycle.borrow_mut().blend(weight, delay);
        }

        // check if this is really a animation cycle instance
        let CalAnimation::Cycle(pAnimationCycle) = pAnimation else {
            return false;
        };

        // blend the animation cycle
        pAnimationCycle.borrow_mut().blend(weight, delay);
        pAnimationCycle.borrow().checkCallbacks(0.0, &self.m_pModel);

        // clear the animation cycle from the active vector if the target weight is zero
        if weight == 0.0 {
            self.m_vectorAnimation.insert(id, CalAnimation::None);
        }

        return true;
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

    fn updateSkeleton(&self) {
        todo!();
    }
}
