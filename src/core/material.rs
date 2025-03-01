#[derive(Debug)]
pub(super) struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub(super) fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[derive(Debug)]
pub(super) struct Map {
    strFilename: String,
    mapType: String,
    userData: i32,
}

impl Map {
    pub(super) fn new(strFilename: String, mapType: String, userData: i32) -> Self {
        Map {
            strFilename,
            mapType,
            userData,
        }
    }
}

#[derive(Debug)]
pub struct CalCoreMaterial {
    m_ambientColor: Color,
    m_diffuseColor: Color,
    m_specularColor: Color,
    m_shininess: f32,
    m_vectorMap: Vec<Map>,
    // Cal::UserData    m_userData;
    // std::string      m_name;
    // std::string      m_filename;
}

impl CalCoreMaterial {
    pub fn new(
        m_ambientColor: Color,
        m_diffuseColor: Color,
        m_specularColor: Color,
        m_shininess: f32,
        m_vectorMap: Vec<Map>,
    ) -> Self {
        CalCoreMaterial {
            m_ambientColor,
            m_diffuseColor,
            m_specularColor,
            m_shininess,
            m_vectorMap,
        }
    }
}
