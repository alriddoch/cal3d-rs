use crate::CalSkeleton;
use crate::core::CalCoreBone;
use crate::vector::blend::Blend;
use crate::vector::bounding::BoundingBox;
use crate::{CalQuaternion, CalVector};
use cgmath::{Matrix3, SquareMatrix};
use std::{cell::RefCell, rc::Rc};

pub struct CalBone {
    m_pCoreBone: Rc<RefCell<CalCoreBone>>,
    m_pSkeleton: Option<Rc<RefCell<CalSkeleton>>>,
    m_accumulatedWeight: f32,
    m_accumulatedWeightAbsolute: f32,
    m_accumulatedReplacementAttenuation: f32,
    m_firstBlendScale: f32,
    m_meshScaleAbsolute: CalVector<f32>, // w.r.t. absolute coord system in 3dsMax (Z up), not local coord of bone.
    m_translation: CalVector<f32>,
    m_rotation: CalQuaternion<f32>,
    m_translationAbsolute: CalVector<f32>,
    m_rotationAbsolute: CalQuaternion<f32>,
    m_translationBoneSpace: CalVector<f32>,
    m_rotationBoneSpace: CalQuaternion<f32>,
    m_transformMatrix: Matrix3<f32>,
    m_boundingBox: BoundingBox,
}

impl CalBone {
    pub fn new(core_bone: &Rc<RefCell<CalCoreBone>>) -> Self {
        Self {
            m_pCoreBone: core_bone.clone(),
            m_pSkeleton: None,
            m_accumulatedWeight: 0.0,
            m_accumulatedWeightAbsolute: 0.0,
            m_accumulatedReplacementAttenuation: 0.0,
            m_firstBlendScale: 0.0,
            m_meshScaleAbsolute: CalVector::<f32>::new(0.0, 0.0, 0.0), // w.r.t. absolute coord system in 3dsMax (Z up), not local coord of bone.
            m_translation: CalVector::<f32>::new(0.0, 0.0, 0.0),
            m_rotation: CalQuaternion::new(0.0, 1.0, 0.0, 0.0),
            m_translationAbsolute: CalVector::<f32>::new(0.0, 0.0, 0.0),
            m_rotationAbsolute: CalQuaternion::new(0.0, 1.0, 0.0, 0.0),
            m_translationBoneSpace: CalVector::<f32>::new(0.0, 0.0, 0.0),
            m_rotationBoneSpace: CalQuaternion::new(0.0, 1.0, 0.0, 0.0),
            m_transformMatrix: Matrix3::<f32>::identity(),
            m_boundingBox: BoundingBox::default(),
        }
    }

    pub fn getCoreBone(&self) -> &Rc<RefCell<CalCoreBone>> {
        &self.m_pCoreBone
    }

    pub fn setSkeleton(&mut self, skeleton: &Rc<RefCell<CalSkeleton>>) {
        self.m_pSkeleton = Some(skeleton.clone());
    }

    pub fn setMeshScaleAbsolute(&mut self, sv: &CalVector<f32>) {
        self.m_meshScaleAbsolute = *sv;
    }

    // 35 cpp
    /*****************************************************************************/
    /** Interpolates the current state to another state.
     *
     * This function interpolates the current state (relative translation and
     * rotation) of the bone instance to another state of a given weight.
     *
     * @param unrampedWeight The blending weight, not incorporating ramp value
     * @param translation The relative translation to be interpolated to.
     * @param rotation The relative rotation to be interpolated to.
     * @param scale Optional scale from 0-1 applies to transformation directly without affecting weights.
     * @param replace If true, subsequent animations will have their weight attenuated by 1 - rampValue.
     * @param rampValue Amount to attenuate weight when ramping in/out the animation.
     * @param absoluteTranslation If true, use the translation as absolute, otherwise add it to the current bone translation as relative.
     *****************************************************************************/

    pub fn blendState(
        &mut self,
        unrampedWeight: f32,
        translation: &CalVector<f32>,
        rotation: &CalQuaternion<f32>,
        mut scale: f32,
        replace: bool,
        rampValue: f32,
        absoluteTranslation: bool,
    ) {
        // Attenuate the weight by the accumulated replacement attenuation.  Each applied
        // "replacement" animation attenuates the weights of the subsequent animations by
        // the inverse of its rampValue, so that when a replacement animation ramps up to
        // full, all lesser priority animations automatically ramp down to zero.
        let rampedWeight = unrampedWeight * rampValue;
        let attenuatedWeight = rampedWeight * self.m_accumulatedReplacementAttenuation;

        // It appears that quaternion::blend() only works with blend factors of 0-1, so
        // I'll clamp the scale to that range.
        if scale < 0.0 {
            scale = 0.0;
        }
        if scale > 1.0 {
            scale = 1.0;
        }

        // Now apply weighted, scaled transformation.  For weights, Cal starts with the
        // first and then blends the later ones in proportion to their weights.  Though this
        // would seem to depend on the order, you can reason by induction that it does not.
        // Each application of an animation gives it the correct proportion to the others in
        // aggregate and leaves in tact the proportions among the others.
        if self.m_accumulatedWeightAbsolute == 0.0 {
            // It is the first state, so we can just copy it into the bone state.  The first animation
            // must be applied with scale = 1.0 since it is the initial pose rather than something
            // to be blended onto a pose.  If we scale the first state, the skeleton will look like
            // a crumpled spider.
            self.m_accumulatedWeightAbsolute = attenuatedWeight;
            self.m_translationAbsolute = if absoluteTranslation {
                *translation
            } else {
                self.m_translation + translation
            };
            self.m_rotationAbsolute = *rotation;

            // I would like to scale this blend, but I cannot since it is the initial pose.  Thus I
            // will store away this scale and compensate appropriately on the second blend.  See below.
            // After applying blend2, the blend1 = 1 - blend2.  If I would like to scale blend1 to 30%
            // of its original scale, for example, then I would like,
            //
            //      ( 1 - blend2' ) = 0.3 * ( 1 - blend2 )
            // so,
            //      blend2' = 1 - 0.3 * ( 1 - blend2 )
            //
            // or similarly for any value of m_firstBlendScale instead of 30%.
            self.m_firstBlendScale = scale;
        } else {
            // Consider an example with two animations, one or both of them "replace" animations.
            // Wave is a "replace" animation, played on top of Walk.  Wave is applied first since it is a
            // "replace" animation and Walk is not.  Imagine Wave is ramping in, currently at 80%.  Wave sets
            // the initial pose 100% and then Walk is applied over that pose with a blend factor of 0.2.  The result
            // is that Wave is 80% and Walk is 20%, which is what you'd expect for replace semantics.
            //
            // Animation    RampedWeight  AttenuatedWeight    InAccumWeightAbs  OutAccAttenuation   Factor
            // Wave         0.8           0.8                 0.0               0.2 (replace)       n/a (100%)
            // Walk         1.0           0.2                 0.8               0.2 (not replace)   0.2/(0.8+0.2) = 0.2
            //
            // Consider the same example with two animations, but neither of them "replace" animations.
            // Assume Wave is applied first.  Imagine Wave is ramping in, currently at 80%.  Wave sets
            // the initial pose 100% and then Walk is applied over that pose with a blend factor of 0.55.  The result
            // is that Wave is 45% and Walk is 55%, which is about what you'd expect for non-replace semantics.
            //
            // Animation    RampedWeight  AttenuatedWeight    InAccumWeightAbs  OutAccAttenuation   Factor
            // Wave         0.8           0.8                 0.0               1.0 (not replace)   n/a (100%)
            // Walk         1.0           1.0                 0.8               1.0 (not replace)   1.0/(0.8+1.0) = 0.55
            //
            // Consider the same example again but reverse the order of Wave and Walk, so Walk is applied first.
            // As before, imagine Wave is ramping in, currently at 80%.  Walk sets the initial pose 100%
            // and then Wave is applied over that pose with a blend factor of 0.44.  The result
            // is that Wave is 44% and Walk is 56%, which is also about what you'd expect for non-replace semantics.
            //
            // Animation    RampedWeight  AttenuatedWeight    InAccumWeightAbs  OutAccAttenuation   Factor
            // Walk         1.0           1.0                 0.0               1.0 (not replace)   n/a (100%)
            // Wave         0.8           0.8                 1.0               1.0 (not replace)   0.8/(0.8+1.0) = 0.44
            //
            // Now consider an example in which Point and Wave are both applied over Walk, with Point applied
            // first at highest priority.  Assume that Point is ramped at 90% and Wave is ramped at 80%.  Both
            // Point and Wave are "replace" animations.  Walk is not.  The result is Walk is 2%, Wave is about 8%,
            // and Point is about 90%, which seems like a reasonable result.
            //
            // Animation    RampedWeight  AttenuatedWeight    InAccumWeightAbs  OutAccAttenuation   Factor
            // Point        0.9           0.9                 0                 0.1 (replace)       n/a (100%)
            // Wave         0.8           0.08                0.9               0.02 (replace)      0.08/(0.9+0.08) = 0.082
            // Walk         1.0           0.02                0.98              0.02 (not replace)  0.02/(0.98+0.02) = 0.02
            //
            // Finally, consider an example in which Point and Wave are both applied over Walk, but in which
            // none of the animations is a "replace" animation.  For this example, assume that Point, Wave,
            // and Walk all are fully ramped in at 100%.  The result is Walk is 33%, Wave is about 33%,
            // and Point is about 33%, which seems like the right result.
            //
            // Animation    RampedWeight  AttenuatedWeight    InAccumWeightAbs  OutAccAttenuation   Factor
            // Point        1.0           1.0                 0.0               1.0 (not replace)   n/a (100%)
            // Wave         1.0           1.0                 1.0               1.0 (not replace)   1.0/(1.0+1.0) = 0.5
            // Walk         1.0           1.0                 2.0               1.0 (not replace)   1.0/(1.0+2.0) = 0.33
            let mut factor =
                scale * attenuatedWeight / (self.m_accumulatedWeightAbsolute + attenuatedWeight);

            // If the scale of the first blend was not 1.0, then I will adjust the factor of the second blend
            // to compensate,
            //
            //      factor' = 1 - m_firstBlendScale * ( 1 - factor )
            //
            assert!(factor <= 1.0);
            factor = 1.0 - self.m_firstBlendScale * (1.0 - factor);
            let newTrans = if absoluteTranslation {
                *translation
            } else {
                self.m_translation + translation
            };
            self.m_translationAbsolute.blend(factor, &newTrans);
            self.m_rotationAbsolute.blend(factor, rotation);
            self.m_accumulatedWeightAbsolute += attenuatedWeight;
            self.m_firstBlendScale = 1.0;
        }
        if replace {
            self.m_accumulatedReplacementAttenuation *= 1.0 - rampValue;
        }
    }

    // 328
    /*****************************************************************************/
    /** Clears the current state.
     *
     * This function clears the current state (absolute translation and rotation)
     * of the bone instance and all its children.
     *****************************************************************************/
    pub fn clearState(&mut self) {
        self.m_accumulatedWeight = 0.0;
        self.m_accumulatedWeightAbsolute = 0.0;
        self.m_accumulatedReplacementAttenuation = 1.0;
        self.m_firstBlendScale = 1.0;
        self.m_meshScaleAbsolute = CalVector::<f32>::new(1.0, 1.0, 1.0);
    }

    // 343 cpp
    /*****************************************************************************/
    /** Resets the bone transform state variables for rotation and translation.
     *
     * This function changes the state of the bone to its default non-animated
     * position and orientation. Child bones are unaffected and may be animated
     * independently.
     *****************************************************************************/
    pub fn setCoreTransformStateVariables(&mut self) {
        self.m_translation = *self.m_pCoreBone.borrow().getTranslation();
        self.m_rotation = *self.m_pCoreBone.borrow().getRotation();
    }

    // 422 cpp
    /*****************************************************************************/
    /** Locks the current state.
     *
     * This function locks the current state (absolute translation and rotation)
     * of the bone instance and all its children.
     *****************************************************************************/

    pub fn lockState(&mut self) {
        // clamp accumulated weight
        if self.m_accumulatedWeightAbsolute > 1.0 - self.m_accumulatedWeight {
            self.m_accumulatedWeightAbsolute = 1.0 - self.m_accumulatedWeight;
        }

        if self.m_accumulatedWeightAbsolute > 0.0 {
            if self.m_accumulatedWeight == 0.0 {
                // it is the first state, so we can just copy it into the bone state
                self.m_translation = self.m_translationAbsolute;
                self.m_rotation = self.m_rotationAbsolute;

                self.m_accumulatedWeight = self.m_accumulatedWeightAbsolute;
            } else {
                // it is not the first state, so blend all attributes
                let factor = self.m_accumulatedWeightAbsolute
                    / (self.m_accumulatedWeight + self.m_accumulatedWeightAbsolute);

                self.m_translation
                    .blend(factor, &self.m_translationAbsolute);
                self.m_rotation.blend(factor, &self.m_rotationAbsolute);

                self.m_accumulatedWeight += self.m_accumulatedWeightAbsolute;
            }

            self.m_accumulatedWeightAbsolute = 0.0;
        }
    }
}
