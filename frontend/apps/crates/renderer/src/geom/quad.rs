use awsm_web::webgl::{
    WebGl2Renderer,
    Id,
    BufferData,
    BufferTarget,
    BufferUsage,
    AttributeOptions,
    DataType
};

#[derive(Debug, Clone)]
pub struct Quad {
    vertex_buffer_id: Id
}


impl Quad {
    pub fn new_unit(gl:&mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {

        const QUAD_GEOM_UNIT: [f32; 8] = [
            0.0, 1.0, // top-left
            0.0, 0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0, // bottom-right
        ];

        let id = gl.create_buffer()?;

        //TODO - change to just upload_buffer
        //Not to bind to shader program
        gl.upload_buffer(
            id,
            BufferData::new(
                &QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;

        Ok(Quad {
            vertex_buffer_id: id
        })
    }
}
