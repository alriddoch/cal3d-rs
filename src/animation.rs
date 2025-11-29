use crate::CalModel;
use crate::core::CalCoreAnimation;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub enum CalAnimation {
    None,
    Cycle(Rc<RefCell<CalAnimationCycle>>),
    Action(Rc<RefCell<CalAnimationAction>>),
}

#[derive(Clone)]
pub enum Type {
    TYPE_NONE = 0,
    TYPE_CYCLE,
    TYPE_POSE,
    TYPE_ACTION,
}

#[derive(Clone)]
pub enum State {
    STATE_NONE = 0,
    STATE_SYNC,
    STATE_ASYNC,
    STATE_IN,
    STATE_STEADY,
    STATE_OUT,
    STATE_STOPPED,
}

#[derive(Clone)]
pub enum CompositionFunction {
    CompositionFunctionNull = 0,
    CompositionFunctionReplace,
    CompositionFunctionAverage,
    CompositionFunctionCrossFade,
}

#[derive(Clone)]
pub enum SequencingMode {
    SequencingModeNull = 0,
    SequencingModeAutomatic,
    SequencingModeManual,
}

#[derive(Clone)]
pub struct CalAnimationAction {
    m_pCoreAnimation: Rc<RefCell<CalCoreAnimation>>,
    m_lastCallbackTimes: Vec<f32>,
    m_type: Type,
    m_state: State,
    m_time: f32,
    m_timeFactor: f32,
    m_weight: f32,
    m_delayIn: f32,
    m_delayOut: f32,
    m_delayTarget: f32,
    m_weightTarget: f32,
    m_autoLock: bool,
    m_scale: f32,
    m_rampValue: f32,
    m_compositionFunction: CompositionFunction,
    m_sequencingMode: SequencingMode,
    m_manualOn: bool,
}

impl CalAnimationAction {
    pub fn new(core_animation: Rc<RefCell<CalCoreAnimation>>) -> Self {
        Self {
            m_pCoreAnimation: core_animation,
            m_lastCallbackTimes: Vec::new(),
            m_type: Type::TYPE_ACTION,
            m_state: State::STATE_NONE,
            m_time: 0.0,
            m_timeFactor: 1.0,
            m_weight: 0.0,
            m_delayIn: 0.0,
            m_delayOut: 0.0,
            m_delayTarget: 0.0,
            m_weightTarget: 0.0,
            m_autoLock: false,
            m_scale: 1.0,
            m_rampValue: 0.0,
            m_compositionFunction: CompositionFunction::CompositionFunctionNull,
            m_sequencingMode: SequencingMode::SequencingModeNull,
            m_manualOn: false,
        }
    }

    pub fn getState(&self) -> &State {
        &self.m_state
    }

    pub fn getTime(&self) -> f32 {
        self.m_time
    }

    pub fn getWeight(&self) -> f32 {
        self.m_weight
    }

    // 290 cpp
    /*****************************************************************************/
    /** Updates the animation action instance.
     *
     * This function updates the animation action instance for a given amount of
     * time.  It has no effect on manual actions.
     *
     * @param deltaTime The elapsed time in seconds since the last update.
     *
     * @return One of the following values:
     *         \li \b true if the animation action instance is still active or is manual
     *         \li \b false if the execution of the animation action instance has
     *             ended
     *****************************************************************************/
    pub fn update(&mut self, deltaTime: f32) -> bool {
        // Mixer should not call update on manual actions.
        // Return true, not false, if manual, because we ignore manual, and our
        // return parameter indicates whether the action has ended.  A manual action
        // doesn't end.
        if !matches!(
            self.m_sequencingMode,
            SequencingMode::SequencingModeAutomatic
        ) {
            return true;
        }

        // update animation action time

        if !matches!(self.m_state, State::STATE_STOPPED) {
            self.m_time = self.m_time + deltaTime * self.m_timeFactor;
        }

        // handle IN phase
        if matches!(self.m_state, State::STATE_IN) {
            // check if we are still in the IN phase
            if self.m_time < self.m_delayIn {
                self.m_weight = self.m_time / self.m_delayIn * self.m_weightTarget;
                //self.m_weight = self.m_time / self.m_delayIn;
            } else {
                self.m_state = State::STATE_STEADY;
                self.m_weight = self.m_weightTarget;
            }
        }

        // handle STEADY
        if matches!(self.m_state, State::STATE_STEADY) {
            // check if we reached OUT phase
            if !self.m_autoLock
                && self.m_time >= self.m_pCoreAnimation.borrow().getDuration() - self.m_delayOut
            {
                self.m_state = State::STATE_OUT;
            } else if self.m_autoLock && self.m_time > self.m_pCoreAnimation.borrow().getDuration()
            {
                // if the anim is supposed to stay locked on last keyframe, reset the time here.
                self.m_state = State::STATE_STOPPED;
                self.m_time = self.m_pCoreAnimation.borrow().getDuration();
            }
        }

        // handle OUT phase
        if matches!(self.m_state, State::STATE_OUT) {
            // check if we are still in the OUT phase
            if self.m_time < self.m_pCoreAnimation.borrow().getDuration() {
                self.m_weight = (self.m_pCoreAnimation.borrow().getDuration() - self.m_time)
                    / self.m_delayOut
                    * self.m_weightTarget;
            } else {
                // we reached the end of the action animation
                self.m_weight = 0.0;
                return false;
            }
        }

        return true;
    }

    pub fn checkCallbacks(&self, animationTime: f32, model: &Rc<RefCell<CalModel>>) {
        // TODO Write a callback system, when it's clear what it's for.
    }

    pub fn completeCallbacks(&self, model: &Rc<RefCell<CalModel>>) {
        // TODO Write a callback system, when it's clear what it's for.
    }
}

#[derive(Clone)]
pub struct CalAnimationCycle {
    m_pCoreAnimation: Rc<RefCell<CalCoreAnimation>>,
    m_lastCallbackTimes: Vec<f32>,
    m_type: Type,
    m_state: State,
    m_time: f32,
    m_timeFactor: f32,
    m_weight: f32,
    m_targetDelay: f32,
    m_targetWeight: f32,
}

impl CalAnimationCycle {
    pub fn new(core_animation: Rc<RefCell<CalCoreAnimation>>) -> Self {
        Self {
            m_pCoreAnimation: core_animation,
            m_lastCallbackTimes: Vec::new(),
            m_type: Type::TYPE_CYCLE,
            m_state: State::STATE_SYNC,
            m_time: 0.0,
            m_timeFactor: 1.0,
            m_weight: 0.0,
            m_targetDelay: 0.0,
            m_targetWeight: 0.0,
        }
    }

    pub fn getCoreAnimation(&self) -> &Rc<RefCell<CalCoreAnimation>> {
        &self.m_pCoreAnimation
    }

    pub fn getState(&self) -> &State {
        &self.m_state
    }

    pub fn getTime(&self) -> f32 {
        self.m_time
    }

    pub fn getTimeFactor(&self) -> f32 {
        self.m_timeFactor
    }

    pub fn getWeight(&self) -> f32 {
        self.m_weight
    }

    pub fn setState(&mut self, state: &State) {
        self.m_state = state.clone();
    }

    pub fn setTime(&mut self, time: f32) {
        self.m_time = time;
    }

    pub fn setWeight(&mut self, weight: f32) {
        self.m_weight = weight;
    }

    pub fn blend(&mut self, weight: f32, delay: f32) -> bool {
        self.m_targetWeight = weight;
        self.m_targetDelay = delay;

        return true;
    }

    // 95 cpp
    /*****************************************************************************/
    /** Updates the animation cycle instance.
     *
     * This function updates the animation cycle instance for a given amount of
     * time.
     *
     * @param deltaTime The elapsed time in seconds since the last update.
     *
     * @return One of the following values:
     *         \li \b true if the animation cycle instance is still active
     *         \li \b false if the execution of the animation cycle instance has
     *             ended
     *****************************************************************************/
    pub fn update(&mut self, deltaTime: f32) -> bool {
        if self.m_targetDelay <= f32::abs(deltaTime) {
            // we reached target delay, set to full weight
            self.setWeight(self.m_targetWeight);
            self.m_targetDelay = 0.0;

            // check if we reached the cycles end
            if self.getWeight() == 0.0 {
                return false;
            }
        } else {
            // not reached target delay yet, interpolate between current and target weight

            let factor = deltaTime / self.m_targetDelay;
            self.setWeight((1.0 - factor) * self.getWeight() + factor * self.m_targetWeight);
            self.m_targetDelay -= deltaTime;
        }

        // update animation cycle time if it is in async state
        if matches!(self.getState(), State::STATE_ASYNC) {
            let animation_duration = self.getCoreAnimation().borrow().getDuration();
            self.setTime(self.getTime() + deltaTime * self.getTimeFactor());
            if self.getTime() >= animation_duration {
                self.setTime(self.getTime() % animation_duration);
            }
            if self.getTime() < 0.0 {
                self.setTime(self.getTime() + animation_duration);
            }
        }

        return true;
    }

    // FIXME TODO This is a common method to cycle and action. How do we have common methods, and common fields?
    pub fn checkCallbacks(&self, animationTime: f32, model: &Rc<RefCell<CalModel>>) {
        // TODO Write a callback system, when it's clear what it's for.

        // let list = self.m_pCoreAnimation.getCallbackList();

        // for i in 0..list.len() {
        //     if self.m_lastCallbackTimes.len() <= i {
        //         // need these two lines to allow dynamic adding of callbacks.
        //         self.m_lastCallbackTimes.push(animationTime);
        //     }

        //     list[i]
        //         .callback
        //         .AnimationUpdate(animationTime, model, model.getUserData());
        //     if (animationTime > 0.0 && animationTime < self.m_lastCallbackTimes[i]) {
        //         // looped
        //         self.m_lastCallbackTimes[i] -= self.m_pCoreAnimation.getDuration();
        //     } else if (animationTime < 0.0 && animationTime > self.m_lastCallbackTimes[i]) {
        //         // reverse-looped
        //         self.m_lastCallbackTimes[i] += self.m_pCoreAnimation.getDuration();
        //     }

        //     if ((animationTime >= 0.0
        //         && animationTime >= self.m_lastCallbackTimes[i] + list[i].min_interval)
        //         || (animationTime < 0.0
        //             && animationTime <= self.m_lastCallbackTimes[i] - list[i].min_interval))
        //     {
        //         list[i]
        //             .callback
        //             .AnimationUpdate(animationTime, model, model.getUserData());
        //         m_lastCallbackTimes[i] = animationTime;
        //     }
        // }
    }

    pub fn completeCallbacks(&self, model: &Rc<RefCell<CalModel>>) {
        // TODO Write a callback system, when it's clear what it's for.
    }
}
