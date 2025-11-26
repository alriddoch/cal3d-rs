use std::rc::Rc;

use super::track::CalCoreTrack;

#[derive(Clone)]
pub struct CalCoreAnimation {
    // std::vector<CallbackRecord> m_listCallbacks;
    m_duration: f32,
    m_listCoreTrack: Vec<Rc<CalCoreTrack>>,
    // std::string m_name;
    // std::string m_filename;
}

impl CalCoreAnimation {
    pub fn new(m_duration: f32, m_listCoreTrack: Vec<Rc<CalCoreTrack>>) -> Self {
        CalCoreAnimation {
            m_duration,
            m_listCoreTrack,
        }
    }
}
