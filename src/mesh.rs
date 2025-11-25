use crate::CalSubmesh;
use crate::core::CalCoreMesh;
use crate::core::CalCoreModel;
use std::{cell::RefCell, rc::Rc};

pub struct CalMesh {
    m_pCoreMesh: Rc<RefCell<CalCoreMesh>>,
    m_vectorSubmesh: Vec<CalSubmesh>,
}

impl CalMesh {
    pub fn new(core_mesh: Rc<RefCell<CalCoreMesh>>) -> Self {
        CalMesh {
            m_pCoreMesh: core_mesh,
            m_vectorSubmesh: Vec::new(),
        }
    }

    pub fn getCoreMesh(&self) -> &Rc<RefCell<CalCoreMesh>> {
        &self.m_pCoreMesh
    }

    // 211
    /*****************************************************************************/
    /** Sets the material set.
     *
     * This function sets the material set of the mesh instance.
     *
     * @param setId The ID of the material set.
     *****************************************************************************/
    pub fn setMaterialSet(&mut self, setId: i32, core: &CalCoreModel) {
        // change material of every submesh

        for submesh in self.m_vectorSubmesh.iter_mut() {
            // get the core material thread id of the submesh

            let coreMaterialThreadId = submesh.getCoreSubmesh().getCoreMaterialThreadId();

            // get the core material id for the given set id in the material thread

            if let Some(coreMaterialId) = core.getCoreMaterialId(coreMaterialThreadId, setId) {
                // set the new core material id in the submesh
                submesh.setCoreMaterialId(coreMaterialId);
            };
        }
    }
}
