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

    pub fn blend(&mut self, weight: f32, delay: f32) -> bool {
        self.m_targetWeight = weight;
        self.m_targetDelay = delay;

        return true;
    }

    // FIXME TODO This is a common method to cycle and action. How do we have common methods, and common fields?
    pub fn checkCallbacks(&self, animationTime: f32, model: &CalModel) {
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
}
