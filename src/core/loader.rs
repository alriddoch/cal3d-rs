use std::cell::RefCell;
use std::fs;
use std::io::BufReader;
use std::ops::Mul;
use std::path::PathBuf;
use std::rc::Rc;

use crate::{CalQuaternion, CalVector};

use super::animation::CalCoreAnimation;
use super::bone::{CalCoreBone, CalLightType};
use super::bufreadersource::BufReaderSource;
use super::datasource::{DataSource, SourceError};
use super::skeleton::CalCoreSkeleton;
use super::track::CalCoreTrack;
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

pub fn versionHasCompressionFlag( version: i32) -> bool {
    return version >= 1300;
  }

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

//115
/*****************************************************************************/
/** Loads a core animation instance.
 *
 * This function loads a core animation instance from a file.
 *
 * @param strFilename The file to load the core animation instance from.
 *
 * @return One of the following values:
 *         \li a pointer to the core animation
 *         \li \b 0 if an error happened
 *****************************************************************************/

  pub fn loadCoreAnimation(filename: &PathBuf, skel: &Rc<CalCoreSkeleton>) -> Result<Rc<RefCell<CalCoreAnimation>>, LoaderError> {
    let magic: String = String::from_utf8_lossy(ANIMATION_XMLFILE_MAGIC)
    .trim_matches(char::from(0))
    .to_owned();
if filename.to_str().unwrap().ends_with(magic.as_str()) {
    todo!();
}

let mut buff_reader = BufReader::new(fs::File::open(filename)?);

let mut source = BufReaderSource::new(buff_reader);

    let coreanim = loadCoreAnimationFromSource( &mut source, skel )?;

    Ok(coreanim)
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
    let magic: String = String::from_utf8_lossy(SKELETON_XMLFILE_MAGIC)
        .trim_matches(char::from(0))
        .to_owned();
    if filename.to_str().unwrap().ends_with(magic.as_str()) {
        todo!();
    }

    let mut buff_reader = BufReader::new(fs::File::open(filename)?);

    let mut source = BufReaderSource::new(buff_reader);

    return loadCoreSkeletonFromSource(&mut source, skeleton);
}

//552
 /*****************************************************************************/
/** Loads a core animation instance.
  *
  * This function loads a core animation instance from a data source.
  *
  * @param dataSrc The data source to load the core animation instance from.
  *
  * @return One of the following values:
  *         \li a pointer to the core animation
  *         \li \b 0 if an error happened
  *****************************************************************************/

  pub fn loadCoreAnimationFromSource(dataSrc: &mut dyn DataSource, skel: &Rc<CalCoreSkeleton>) -> Result<Rc<RefCell<CalCoreAnimation>>, LoaderError> {
  
    let mut magic: [u8; 4] = [0; 4];
    let magic_len = magic.len();
    dataSrc.readBytes(&mut magic, magic_len)?;
    if &magic != ANIMATION_FILE_MAGIC.as_slice() {
        return Err(LoaderError::MagicError);
    }

    let version = dataSrc.readInteger()?;
    if version < EARLIEST_COMPATIBLE_FILE_VERSION || version > CURRENT_FILE_VERSION {
        return Err(LoaderError::VersionError);
    }
    
    let useAnimationCompression = usesAnimationCompression(version);
    if versionHasCompressionFlag(version) {
       let compressionFlag = dataSrc.readInteger()?;

       // Only really need the first bit.
       useAnimationCompression = compressionFlag != 0;
    }
  
  
    // allocate a new core animation instance
    // FIXME Maybe move this down?
    let pCoreAnimation =  Rc::new(RefCell::new(CalCoreAnimation::new()));
  
    // get the duration of the core animation
    let duration = dataSrc.readFloat()?;
  
    // check for a valid duration
    if duration <= 0.0    {
        return Err(LoaderError::FormatError(format!("Animation duration {duration} is negative")));
    }
  
    // set the duration in the core animation instance
    pCoreAnimation.setDuration(duration);
  
    // read the number of tracks
    let trackCount = dataSrc.readInteger()?;
    if trackCount <= 0    {
        return Err(LoaderError::FormatError(format!("Animation track count {trackCount} is negative")));
    }
  
      // read flags
      let flags = 0;
      if version >= LIBRARY_VERSION {
        flags = dataSrc.readInteger()?;
      }
  
    // load all core bones
    
    for trackId in 0..trackCount    {
      // load the core track
      let pCoreTrack= loadCoreTrack(dataSrc,skel, version, useAnimationCompression)?;
  
      // add the core track to the core animation instance
      pCoreAnimation.addCoreTrack(pCoreTrack);
    }
  
    return Ok(pCoreAnimation);
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
        let bone = loadCoreBones(dataSrc, version, skeleton.clone())?;

        skeleton.addCoreBone(bone.clone());

        // FIXME: This seems redundant, as it's called from within addCoreBone above.
        skeleton.mapCoreBoneName(bone_id, bone.borrow().getName())?;
    }

    skeleton.calculateState();
    Ok(())
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
) -> Result<Rc<RefCell<CalCoreBone>>, LoaderError> {
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
    let pCoreBone = Rc::new(RefCell::new(CalCoreBone::new(
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
    )));

    Ok(pCoreBone)
}

//1158
fn usesAnimationCompression(version: i32 ) -> bool {
   (version >= FIRST_FILE_VERSION_WITH_ANIMATION_COMPRESSION)
}

//2051
/*****************************************************************************/
/** Loads a core track instance.
*
* This function loads a core track instance from a data source.
*
* @param dataSrc The data source to load the core track instance from.
*
* @return One of the following values:
*         \li a pointer to the core track
*         \li \b 0 if an error happened
*****************************************************************************/
pub fn loadCoreTrack(
     dataSrc: &mut dyn DataSource,  skel: &Rc<CalCoreSkeleton>,
    version: i32,  useAnimationCompression: bool) -> Result<Rc<CalCoreTrack>, LoaderError> {
// if(!dataSrc.ok())
// {
// dataSrc.setError();
// return 0;
// }

// Read the bone id.
let coreBoneId: i32;
let translationRequired = true;
let highRangeRequired = true;
let translationIsDynamic = true;
let keyframeCount: i32;
let buf: [u8; 4 ];

// If this file version supports animation compression, then I store the boneId in 15 bits,
// and use the 16th bit to record if translation is required.
if useAnimationCompression  {
    dataSrc.readBytes( buf, 4 )?;


// Stored low byte first.  Top 3 bits of coreBoneId are compression flags.
coreBoneId = buf[ 0 ] as i32 +  ( buf[ 1 ] as i32 & 0x1f ) * 256;
translationRequired = ( buf[ 1 ] & 0x80 ) == 0x80;
highRangeRequired = ( buf[ 1 ] & 0x40 ) == 0x40;
translationIsDynamic = ( buf[ 1 ] & 0x20) == 0x20;
keyframeCount = buf[ 2 ] as i32 +  buf[ 3 ] as i32 * 256;
//if( keyframeCount > keyframeTimeMax ) {
//  CalError::setLastError(CalError::INVALID_FILE_FORMAT, __FILE__, __LINE__);
//  return NULL;
//}
} else {
coreBoneId = dataSrc.readInteger()?;

// Read the number of keyframes.
keyframeCount = dataSrc.readInteger()?;
if keyframeCount <= 0 {
    return Err(LoaderError::FormatError(format!("Keyframe count {keyframeCount} is negative")));
}
}

if coreBoneId < 0 {
    return Err(LoaderError::FormatError(format!("Core bone ID {coreBoneId} is negative")));
}

// allocate a new core track instance
let pCoreTrack = Rc::new(CalCoreTrack::new(coreBoneId, translationRequired, highRangeRequired, translationIsDynamic));

let cb = skel.getCoreBone( coreBoneId );


// load all core keyframes

 let lastCoreKeyframe: Option<Rc<CalCoreKeyframe>> = None;
for keyframeId in 0..keyframeCount {
// load the core keyframe
todo!();
// let pCoreKeyframe = loadCoreKeyframe(
// dataSrc, cb, version, lastCoreKeyframe, translationRequired, highRangeRequired, translationIsDynamic,
// useAnimationCompression);
// lastCoreKeyframe = pCoreKeyframe;
// if(pCoreKeyframe == 0)
// {
// delete pCoreTrack;
// return 0;
// }
// if (loadingMode & LOADER_ROTATE_X_AXIS)
// {
// // Check for anim rotation
// if (skel && skel.getCoreBone(coreBoneId).getParentId() == -1)  // root bone
// {
// // rotate root bone quaternion
// CalQuaternion rot = pCoreKeyframe.getRotation();
// CalQuaternion x_axis_90(0.7071067811f,0.0f,0.0f,0.7071067811f);
// rot *= x_axis_90;
// pCoreKeyframe.setRotation(rot);
// // rotate root bone displacement
// CalVector vec = pCoreKeyframe.getTranslation();
// vec *= x_axis_90;
// pCoreKeyframe.setTranslation(vec);
// }
// }

// // add the core keyframe to the core track instance
// pCoreTrack.addCoreKeyframe(pCoreKeyframe);
}

// // Whenever I load the track, I update its translationRequired status.  The status can
// // go from required to not required, but not the other way around.
// pCoreTrack.setTranslationRequired( translationRequired );
// pCoreTrack.setHighRangeRequired( highRangeRequired );
// pCoreTrack.setTranslationIsDynamic( translationIsDynamic );
// if( collapseSequencesOn ) {
// pCoreTrack.collapseSequences( translationTolerance, rotationToleranceDegrees );
// }
// if( loadingCompressionOn ) {

// // This function MIGHT call setTranslationRequired() on the track.
// // Alas, you may be passing me NULL for skel, in which case compress() won't update the
// // translationRequired flag; instead it will leave it, as above.
// pCoreTrack.compress( translationTolerance, rotationToleranceDegrees, skel );
// }

Ok(pCoreTrack)
}
