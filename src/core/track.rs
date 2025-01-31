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
    // std::vector<CalCoreKeyframe*> m_keyframes;
}

impl CalCoreTrack {
    pub fn new(
        m_coreBoneId: i32,
        m_translationRequired: bool,
        m_highRangeRequired: bool,
        m_translationIsDynamic: bool,
    ) -> Self {
        CalCoreTrack {
            m_coreBoneId,
            m_translationRequired,
            m_highRangeRequired,
            m_translationIsDynamic,
        }
    }
}
