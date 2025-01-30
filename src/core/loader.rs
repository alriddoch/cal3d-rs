use std::fs;
use std::io::BufReader;
use std::ops::Mul;
use std::path::PathBuf;
use std::rc::Rc;

use crate::{CalQuaternion, CalVector};

use super::bone::{CalCoreBone, CalLightType};
use super::bufreadersource::BufReaderSource;
use super::datasource::{DataSource, SourceError};
use super::skeleton::CalCoreSkeleton;
use super::CoreError;

const SKELETON_FILE_MAGIC: &[u8; 4] = b"CSF\0";
const ANIMATION_FILE_MAGIC: &[u8; 4] = b"CAF\0";
const ANIMATEDMORPH_FILE_MAGIC: &[u8; 4] = b"CPF\0";
const MESH_FILE_MAGIC: &[u8; 4] = b"CMF\0";
const MATERIAL_FILE_MAGIC: &[u8; 4] = b"CRF\0";

const SKELETON_XMLFILE_MAGIC: &[u8; 4] = b"XSF\0";
const ANIMATION_XMLFILE_MAGIC: &[u8; 4] = b"XAF\0";
const MESH_XMLFILE_MAGIC: &[u8; 4] = b"XMF\0";
const MATERIAL_XMLFILE_MAGIC: &[u8; 4] = b"XRF\0";

const CAL3D_VERSION: i32 = 1301;
const LIBRARY_VERSION: i32 = CAL3D_VERSION;

// file versions
const CURRENT_FILE_VERSION: i32 = LIBRARY_VERSION;
const EARLIEST_COMPATIBLE_FILE_VERSION: i32 = 699;

const FIRST_FILE_VERSION_WITH_ANIMATION_COMPRESSION6: i32 = 1300;
const FIRST_FILE_VERSION_WITH_ANIMATION_COMPRESSION5: i32 = 1300;
const FIRST_FILE_VERSION_WITH_ANIMATION_COMPRESSION4: i32 = 1300;
const FIRST_FILE_VERSION_WITH_ANIMATION_COMPRESSION: i32 = 1300;
const FIRST_FILE_VERSION_WITH_VERTEX_COLORS: i32 = 91300; //removed from spec (one would require both mesh type vertex color and texture color
const FIRST_FILE_VERSION_WITH_VERTEX_SLAVES_ATTRIBUTES: i32 = 1400; //to implement (more generic than vertex color) all atribute that doesnt have semantic (not in[vertex,normal,tangent,physics) in cal3d but interpole linearly (color and others stuff)

const FIRST_FILE_VERSION_WITH_NODE_LIGHTS: i32 = 91300; //removed
const FIRST_FILE_VERSION_WITH_MATERIAL_TYPES: i32 = 1300;
const FIRST_FILE_VERSION_WITH_MORPH_TARGETS_IN_MORPH_FILES: i32 = 1300;
const FIRST_FILE_VERSION_WITH_RELATIVE_BONE_TRANSLATION: i32 = 1300;
const FIRST_FILE_VERSION_WITH_UPDATED_MORPHMIXER: i32 = 1301;

const LOADER_ROTATE_X_AXIS: i32 = 1;
const LOADER_INVERT_V_COORD: i32 = 2;
const LOADER_FLIP_WINDING: i32 = 4;

pub static loadingMode: i32 = 0;

pub enum LoaderError {
    IoError(std::io::Error),
    MagicError,
    VersionError,
    FormatError(String),
}

impl From<std::io::Error> for LoaderError {
    fn from(error: std::io::Error) -> Self {
        LoaderError::IoError(error)
    }
}

impl From<SourceError> for LoaderError {
    fn from(error: SourceError) -> Self {
        match error {
            SourceError::IoError(e) => LoaderError::IoError(e),
            SourceError::FormatError(e) => LoaderError::FormatError(e),
        }
    }
}

impl From<CoreError> for LoaderError {
    fn from(error: CoreError) -> Self {
        match error {
            CoreError::OtherError(e) => LoaderError::FormatError(e),
        }
    }
}

fn CalVectorFromDataSrc(dataSrc: &mut dyn DataSource) -> Result<CalVector<f32>, LoaderError> {
    let mut v = CalVector::<f32> {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    v.x = dataSrc.readFloat()?;
    v.y = dataSrc.readFloat()?;
    v.z = dataSrc.readFloat()?;

    Ok(v)
}

//261
/*****************************************************************************/
/** Loads a core skeleton instance.
 *
 * This function loads a core skeleton instance from a file.
 *
 * @param strFilename The file to load the core skeleton instance from.
 *
 * @return One of the following values:
 *         \li a pointer to the core skeleton
 *         \li \b 0 if an error happened
 *****************************************************************************/
pub fn loadCoreSkeleton(
    filename: &PathBuf,
    skeleton: &Rc<CalCoreSkeleton>,
) -> Result<(), LoaderError> {
    // FIXME check for XML file extension, and call XML lib.
    let mut buff_reader = BufReader::new(fs::File::open(filename)?);

    let mut source = BufReaderSource::new(buff_reader);

    return loadCoreSkeletonFromSource(&mut source, skeleton);
}

//953
/*****************************************************************************/
/** Loads a core skeleton instance.
 *
 * This function loads a core skeleton instance from a data source.
 *
 * @param dataSrc The data source to load the core skeleton instance from.
 *
 * @return One of the following values:
 *         \li a pointer to the core skeleton
 *         \li \b 0 if an error happened
 *****************************************************************************/
fn loadCoreSkeletonFromSource(
    dataSrc: &mut dyn DataSource,
    skeleton: &Rc<CalCoreSkeleton>,
) -> Result<(), LoaderError> {
    // FIXME Check stuff
    let mut magic: [u8; 4] = [0; 4];
    let magic_len = magic.len();
    dataSrc.readBytes(&mut magic, magic_len)?;
    if &magic != SKELETON_FILE_MAGIC.as_slice() {
        return Err(LoaderError::MagicError);
    }

    let version = dataSrc.readInteger()?;
    if version < EARLIEST_COMPATIBLE_FILE_VERSION || version > CURRENT_FILE_VERSION {
        return Err(LoaderError::VersionError);
    }

    let bone_count = dataSrc.readInteger()?;
    if bone_count <= 0 {
        return Err(LoaderError::FormatError(format!(
            "Bonecount {bone_count} is not positive",
        )));
    }

    for bone_id in 0..bone_count {
        let pCoreBone = loadCoreBones(dataSrc, version, skeleton.clone())?;

        skeleton.addCoreBone(pCoreBone.clone());

        // FIXME: This seems redundant, as it's called from within addCoreBone above.
        skeleton.mapCoreBoneName(bone_id, pCoreBone.getName())?;
    }

    todo!();
    // skeleton.calculateState();

    todo!();

    Ok(())
    // FIXME Populdate stuff
}

/*****************************************************************************/
/** Loads a core bone instance.
 *
 * This function loads a core bone instance from a data source.
 *
 * @param dataSrc The data source to load the core bone instance from.
 *
 * @return One of the following values:
 *         \li a pointer to the core bone
 *         \li \b 0 if an error happened
 *****************************************************************************/
fn loadCoreBones(
    dataSrc: &mut dyn DataSource,
    version: i32,
    skeleton: Rc<CalCoreSkeleton>,
) -> Result<Rc<CalCoreBone>, LoaderError> {
    let hasNodeLights = (version >= FIRST_FILE_VERSION_WITH_NODE_LIGHTS);

    //   if !dataSrc.ok()  {
    //     dataSrc.setError();
    //     return 0;
    //   }

    // read the name of the bone
    let strName = dataSrc.readString()?;

    // get the translation of the bone
    let tx = dataSrc.readFloat()?;
    let ty = dataSrc.readFloat()?;
    let tz = dataSrc.readFloat()?;

    // get the rotation of the bone
    let rx = dataSrc.readFloat()?;
    let ry = dataSrc.readFloat()?;
    let rz = dataSrc.readFloat()?;
    let rw = dataSrc.readFloat()?;

    // get the bone space translation of the bone
    let txBoneSpace = dataSrc.readFloat()?;
    let tyBoneSpace = dataSrc.readFloat()?;
    let tzBoneSpace = dataSrc.readFloat()?;

    // get the bone space rotation of the bone
    let rxBoneSpace = dataSrc.readFloat()?;
    let ryBoneSpace = dataSrc.readFloat()?;
    let rzBoneSpace = dataSrc.readFloat()?;
    let rwBoneSpace = dataSrc.readFloat()?;

    // get the parent bone id
    let parentId = dataSrc.readInteger()?;

    // get the lgith type and light color
    let lightType = CalLightType::LIGHT_TYPE_NONE;
    let lightColor: CalVector<f32>;
    if hasNodeLights {
        let lightType = dataSrc.readInteger()?;
        let lightColor = CalVectorFromDataSrc(dataSrc)?;
    }

    let mut rot = CalQuaternion::<f32>::new(rw, rx, ry, rz);
    let rotbs = CalQuaternion::<f32>::new(rwBoneSpace, rxBoneSpace, ryBoneSpace, rzBoneSpace);
    let mut trans = CalVector::new(tx, ty, tz);

    if (loadingMode & LOADER_ROTATE_X_AXIS) == LOADER_ROTATE_X_AXIS {
        if parentId == -1 {
            // only root bone necessary

            // Root bone must have quaternion rotated
            let x_axis_90 = CalQuaternion::new(0.7071067811, 0.7071067811, 0.0, 0.0);
            // rot *= x_axis_90;
            rot = rot.mul(x_axis_90);
            // Root bone must have translation rotated also
            // trans *= x_axis_90;
            trans = x_axis_90.mul(trans);
        }
    }

    // check if an error happened
    //   if !dataSrc.ok()  {
    //     dataSrc.setError();
    //     return 0;
    //   }

    // read the number of children
    let childCount = dataSrc.readInteger()?;
    if childCount < 0 {
        return Err(LoaderError::FormatError(format!(
            "Child count {childCount} is not positive",
        )));
    }

    let mut childs = Vec::<i32>::with_capacity(childCount as usize);

    // load all children ids
    for _ in 0..childCount {
        let childId = dataSrc.readInteger()?;
        if childId < 0 {
            return Err(LoaderError::FormatError(format!(
                "Child ID {childId} is not positive",
            )));
        }

        childs.push(childId);
    }

    // allocate a new core bone instance
    let pCoreBone = Rc::new(CalCoreBone::new(
        strName,
        // CalCoreSkeleton *m_pCoreSkeleton;
        skeleton,
        parentId,
        childs,
        trans,
        rot,
        // CalVector        m_translationAbsolute;
        // CalQuaternion    m_rotationAbsolute;
        CalVector::<f32>::new(txBoneSpace, tyBoneSpace, tzBoneSpace),
        rotbs,
        // name: strName, m_parentId, m_translation,
    ));

    Ok(pCoreBone)
}
