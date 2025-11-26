use std::{cell::RefCell, rc::Rc};

use super::track::CalCoreTrack;

#[derive(Clone)]
pub struct CalCoreAnimation {
    // std::vector<CallbackRecord> m_listCallbacks;
    m_duration: f32,
    m_listCoreTrack: Vec<Rc<RefCell<CalCoreTrack>>>,
    // std::string m_name;
    // std::string m_filename;
}

impl CalCoreAnimation {
    pub fn new(m_duration: f32, m_listCoreTrack: Vec<Rc<RefCell<CalCoreTrack>>>) -> Self {
        CalCoreAnimation {
            m_duration,
            m_listCoreTrack,
        }
    }

    pub fn getDuration(&self) -> f32 {
        self.m_duration
    }

    pub fn getListCoreTrack(&self) -> &Vec<Rc<RefCell<CalCoreTrack>>> {
        &self.m_listCoreTrack
    }

    pub fn getListCoreTrackMut(&mut self) -> &mut Vec<Rc<RefCell<CalCoreTrack>>> {
        &mut self.m_listCoreTrack
    }
}
