use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use super::bufreadersource::BufReaderSource;
use super::datasource::{DataSource, SourceError};
use super::skeleton::CalCoreSkeleton;

pub enum LoaderError {
    IoError(std::io::Error),
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
    // FIXME Check magic, then continue

    Ok(())
    // FIXME Populdate stuff
}
