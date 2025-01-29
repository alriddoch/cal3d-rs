use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use super::bufreadersource::BufReaderSource;
use super::datasource::{DataSource, SourceError};
use super::skeleton::CalCoreSkeleton;

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
        }
    }
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
    skeleton: &mut CalCoreSkeleton,
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
    skeleton: &mut CalCoreSkeleton,
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
        return Err(LoaderError::FormatError(String::from(
            "Bonecount is not positive",
        )));
    }

    for bone_id in 0..bone_count {
        todo!();
    }

    todo!();
    // skeleton.calculateState();

    todo!();

    Ok(())
    // FIXME Populdate stuff
}
