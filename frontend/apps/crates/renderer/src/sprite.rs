/// A sprite is an entity in the scenegraph with the following components:
/// * Primitive - Geometry::Quad(UnitQuad) 
/// * Material
///     Alpha
///         * vertex shader - UnitQuad
///         * fragment shader - DiffuseTexture 

use crate::prelude::*;
use wasm_bindgen::prelude::*;
use awsm_web::webgl::Id;
use shipyard::*;
use shipyard_scenegraph::prelude::*;

impl Renderer {
    pub fn add_sprite(&self, texture_id:Id, parent: Option<EntityId>) {
        let world = &self.world;


        let entity = {
            world.borrow::<SceneGraphStoragesMut>()
                .unwrap_throw()
                .spawn_child_identity(parent)
        };

        let (entities, mut primitives, mut materials) 
            = world.borrow::<(EntitiesViewMut, ViewMut<Primitive>, ViewMut<Material>)>().unwrap();
  
        let primitive = Primitive {
            geom: Geometry::Quad(self.static_geometry.unit_quad.clone()) 
        };

        /*Rough next steps:
         * Statically defined vertex shader
            like unit quad geom, and uses it
        */

        /*

        entities.add_component_unchecked(
            entity, 
            (&mut primitives, &mut materials), 
            (primitive, material)
        );
        */
    }
}
