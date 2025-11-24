pub trait CalAbstractMixer {
    /*****************************************************************************/
    /**
    	* Is the object an instance of the default mixer (i.e. an instance of CalMixer) ?
    	*
    	* @return \li \b true if an instance of CalMixer
    	*         \li \b false if not an instance of CalMixer
    	*
    	*****************************************************************************/
    fn isDefaultMixer() -> bool {
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
    fn updateAnimation(deltaTime: f32);

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
    fn updateSkeleton();
}

pub struct CalMixer {
    // 	unsigned int m_numBoneAdjustments;
    // BoneAdjustmentAndBoneId m_boneAdjustmentAndBoneIdArray[CalMixerBoneAdjustmentsMax];
    // virtual void applyBoneAdjustments();
    // CalModel *m_pModel;
    // std::vector<CalAnimation *> m_vectorAnimation;
    // std::list<CalAnimationAction *> m_listAnimationAction;
    // std::list<CalAnimationCycle *> m_listAnimationCycle;
    // float m_animationTime;
    // float m_animationDuration;
    // float m_timeFactor;
}

impl CalMixer {
    pub fn blendCycle(&self, id: i32, weight: f32, delay: f32) -> bool {
        todo!();
    }
}

impl CalAbstractMixer for CalMixer {
    fn isDefaultMixer() -> bool {
        true
    }
    fn updateAnimation(deltaTime: f32) {
        todo!();
    }
    fn updateSkeleton() {
        todo!();
    }
}
