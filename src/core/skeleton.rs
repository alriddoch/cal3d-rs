use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::CalCoreModel;
use super::bone::CalCoreBone;

#[derive(Default)]
pub struct CalCoreSkeleton {
    m_vectorCoreBone: Vec<Rc<RefCell<CalCoreBone>>>,
    m_mapCoreBoneNames: BTreeMap<String, i32>,
    m_vectorRootCoreBoneId: Vec<i32>,
}

impl CalCoreSkeleton {
    pub fn getCoreBone(&self, coreBoneId: usize) -> Option<Rc<RefCell<CalCoreBone>>> {
        self.m_vectorCoreBone.get(coreBoneId).map(|b| b.clone())
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
    pub fn addCoreBone(&mut self, bone: Rc<RefCell<CalCoreBone>>) -> i32 {
        let boneId = self.m_vectorCoreBone.len() as i32;

        // if necessary, add the core bone to the root bone list
        if bone.borrow().getParentId() == -1 {
            self.m_vectorRootCoreBoneId.push(boneId);
        }

        // add a reference from the bone's name to its id
        self.mapCoreBoneName(boneId, bone.borrow().getName());

        // Delayed, as the bone is moved
        self.m_vectorCoreBone.push(bone);

        return boneId;
    }

    /*****************************************************************************/
    /** Calculates the current state.
     *
     * This function calculates the current state of the core skeleton instance by
     * calculating all the core bone states.
     *****************************************************************************/
    pub fn calculateState(&mut self) {
        // calculate all bone states of the skeleton

        for iteratorRootCoreBoneId in self.m_vectorRootCoreBoneId.iter() {
            self.m_vectorCoreBone[*iteratorRootCoreBoneId as usize]
                .borrow_mut()
                .calculateState();
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
    pub fn mapCoreBoneName(
        &mut self,
        coreBoneId: i32,
        strName: &str,
    ) -> Result<(), super::CoreError> {
        //Make sure the ID given is a valid corebone ID number
        if (coreBoneId < 0) || (coreBoneId >= self.m_vectorCoreBone.len() as i32) {
            return Err(super::CoreError::OtherError(format!(
                "Bone id {coreBoneId} outside range 0..{}",
                self.m_vectorCoreBone.len()
            )));
        }

        //Add the mapping or overwrite an existing mapping
        self.m_mapCoreBoneNames
            .insert(strName.to_string(), coreBoneId);

        Ok(())
    }

    // 254 cpp
    /*****************************************************************************/
    /** Calculates bounding boxes.
     *
     * This function Calculates the bounding box of every bone in the core Skeleton.
     *
     * @param pCoreModel The coreModel (needed for vertices data).
     *****************************************************************************/

    pub fn calculateBoundingBoxes(&self, pCoreModel: &Rc<RefCell<crate::core::CalCoreModel>>) {
        // First, find out whether all the bounding boxes have already been precomputed.
        // If so, we can bail out early.
        let mut alreadyComputed = true;
        for bone in self.m_vectorCoreBone.iter() {
            if !bone.borrow().isBoundingBoxPrecomputed() {
                alreadyComputed = false;
                break;
            }
        }
        if alreadyComputed {
            return;
        }

        // Initialize all bounding boxes empty.
        for bone in self.m_vectorCoreBone.iter() {
            bone.borrow_mut().initBoundingBox();
        }

        // Loop over all vertices updating bounding boxes.
        for pCoreMesh in pCoreModel.borrow().getCoreMeshes() {
            for pCoreSubmesh in pCoreMesh.borrow().getCoreSubmeshes() {
                let submesh = pCoreSubmesh.borrow();
                if submesh.getSpringCount() == 0 {
                    let vectorVertex = submesh.getVectorVertex();

                    for vertexId in vectorVertex.iter() {
                        let vectorInfluence = &vertexId.vectorInfluence;
                        for influenceId in vectorInfluence.iter() {
                            if influenceId.weight > 0.5 {
                                let boneId = influenceId.boneId as usize;

                                self.m_vectorCoreBone[boneId]
                                    .borrow_mut()
                                    .updateBoundingBox(&vertexId.position);

                                break; // there can be at most one bone with majority influence
                            }
                        }
                    }
                }
            }
        }

        // Mark bounding boxes as computed.
        for bone in self.m_vectorCoreBone.iter() {
            bone.borrow_mut().setBoundingBoxPrecomputed(true);
        }
    }
}
