use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::bone::CalCoreBone;

#[derive(Default)]
pub struct CalCoreSkeleton {
    m_vectorCoreBone: RefCell<Vec<Rc<RefCell<CalCoreBone>>>>,
    m_mapCoreBoneNames: RefCell<BTreeMap<String, i32>>,
    m_vectorRootCoreBoneId: RefCell<Vec<i32>>,
}

impl CalCoreSkeleton {
    pub fn getCoreBone(&self, coreBoneId: i32) -> Option<Rc<RefCell<CalCoreBone>>> {
        self.m_vectorCoreBone.borrow().get(coreBoneId as usize).map(|b| {b.clone()})
    }

    //43
    /*****************************************************************************/
    /** Adds a core bone.
     *
     * This function adds a core bone to the core skeleton instance.
     *
     * @param pCoreBone A pointer to the core bone that should be added.
     *
     * @return One of the following values:
     *         \li the assigned bone \b ID of the added core bone
     *         \li \b -1 if an error happened
     *****************************************************************************/
    pub fn addCoreBone(&self, bone: Rc<RefCell<CalCoreBone>>) -> i32 {
        let boneId = self.m_vectorCoreBone.borrow().len() as i32;

        // if necessary, add the core bone to the root bone list
        if bone.borrow().getParentId() == -1 {
            self.m_vectorRootCoreBoneId.borrow_mut().push(boneId);
        }

        // add a reference from the bone's name to its id
        self.mapCoreBoneName(boneId, bone.borrow().getName());

        // Delayed, as the bone is moved
        self.m_vectorCoreBone.borrow_mut().push(bone);

        return boneId;
    }

    /*****************************************************************************/
    /** Calculates the current state.
     *
     * This function calculates the current state of the core skeleton instance by
     * calculating all the core bone states.
     *****************************************************************************/
    pub fn calculateState(&self) {
        // calculate all bone states of the skeleton

        for iteratorRootCoreBoneId in self.m_vectorRootCoreBoneId.borrow().iter() {
            self.m_vectorCoreBone.borrow()[*iteratorRootCoreBoneId as usize].borrow_mut().calculateState();
        }
    }

    //224
    /*****************************************************************************/
    /** Maps the name of a bone to a specific bone id
     *
     * This function returns true or false depending on whether the mapping
     * was successful or not. Note that it is possible to overwrite and existing
     * mapping and no error will be given.
     *
     * @param coreBoneId The id of the core bone to be associated with the name.
     * @param strName The name of the core bone that will be associated with the id.
     *
     * @return One of the following values:
     *         \li true if the mapping was successful
     *         \li false if an invalid ID was given
     *****************************************************************************/
    pub fn mapCoreBoneName(&self, coreBoneId: i32, strName: &str) -> Result<(), super::CoreError> {
        //Make sure the ID given is a valid corebone ID number
        if (coreBoneId < 0) || (coreBoneId >= self.m_vectorCoreBone.borrow().len() as i32) {
            return Err(super::CoreError::OtherError(format!(
                "Bone id {coreBoneId} outside range 0..{}",
                self.m_vectorCoreBone.borrow().len()
            )));
        }

        //Add the mapping or overwrite an existing mapping
        self.m_mapCoreBoneNames
            .borrow_mut()
            .insert(strName.to_string(), coreBoneId);

        Ok(())
    }
}
