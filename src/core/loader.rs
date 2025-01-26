use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use super::bufreadersource::BufReaderSource;
use super::datasource::DataSource;
use super::skeleton::CalCoreSkeleton;

pub enum LoaderError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for LoaderError {
    fn from(error: std::io::Error) -> Self {
        LoaderError::IoError(error)
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
pub fn loadCoreSkeleton(filename: &PathBuf) -> Result<CalCoreSkeleton, LoaderError> {
    // FIXME check for XML file extension, and call XML lib.
    let mut buff_reader = BufReader::new(fs::File::open(filename)?);

    let source = BufReaderSource::new(buff_reader);

    return loadCoreSkeletonFromSource(&source);
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

fn loadCoreSkeletonFromSource(dataSrc: &dyn DataSource) -> Result<CalCoreSkeleton, LoaderError> {
    // FIXME Check stuff
    let skeleton = CalCoreSkeleton {};
    Ok(skeleton)
    // FIXME Populdate stuff
}
