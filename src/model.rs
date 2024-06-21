//a Imports
use model3d_base::Instance;
use model3d_gl::{Gl, ShaderInstantiable, UniformBuffer};

use crate::base_shader;
use crate::objects;

//a Light, WorldData
#[derive(Debug, Default)]
pub struct Light {
    position: model3d_gl::Vec4,
    color: model3d_gl::Vec4,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct WorldData {
    view_matrix: model3d_gl::Mat4,
    lights: [Light; 4],
}

//a Base
//tp Base
pub struct Base<G: Gl> {
    /// The instantiable objects
    objects: model3d_base::Instantiable<G>,
    /// The shader programs
    shader_program: G::Program,
    /// Uniform buffers
    material_gl: UniformBuffer<G>,
    world_gl: UniformBuffer<G>,
}

//tp Instantiable
/// Borrows from Base
///
/// ShaderInstantiable references the compiled Program and the Instantiable object (Base.objects)
pub struct Instantiable<'inst, G: Gl> {
    /// The set of instances of shader_instantiable (only one of them!)
    instantiables: model3d_gl::ShaderInstantiable<'inst, G>,
}

//tp Instances
/// Borrows from Base
///
/// Instances references the Instantiable object
pub struct Instances<'inst, G: Gl> {
    /// The set of instances of objects (only one of them!)
    ///
    /// These are independent of the GL context lifetime
    instance: Instance<'inst, G>,
}

//ip Base
impl<G: Gl> Base<G> {
    //fp new
    pub fn new(gl: &mut G, opt_glb: Option<&[u8]>) -> Result<Self, String> {
        let shader_program = base_shader::compile_shader_program(gl)?;

        let material_uid = 1;
        let world_uid = 2;

        let material_data = [0.0_f32; 8];
        let material_gl = gl
            .uniform_buffer_create(&material_data, false)
            .map_err(|_| "Could not create uniform buffer for material".to_string())?;
        gl.uniform_index_of_range(&material_gl, material_uid, 0, 0);
        gl.program_bind_uniform_index(&shader_program, 1, material_uid)
            .map_err(|_| "Could not bind uniform for material".to_string())?;

        let mut world_data = [WorldData::default(); 1];
        world_data[0].view_matrix[0] = 1.;
        world_data[0].view_matrix[5] = 1.;
        world_data[0].view_matrix[10] = 1.;
        world_data[0].view_matrix[15] = 1.;
        world_data[0].lights[0].position = [2., 0., 0., 0.1];
        world_data[0].lights[0].color = [1., 0., 0., 0.];
        world_data[0].lights[1].position = [-1., 0., 0., 0.1];
        world_data[0].lights[1].color = [0., 1., 0., 0.];
        world_data[0].lights[2].position = [-1., 0., 0., -1.];
        world_data[0].lights[2].color = [0., 0., 1., 0.];

        let world_gl = gl
            .uniform_buffer_create(&world_data, true)
            .map_err(|_| "Could not create uniform buffer for world data".to_string())?;
        gl.uniform_index_of_range(&world_gl, world_uid, 0, 0);
        gl.program_bind_uniform_index(&shader_program, 2, world_uid)
            .map_err(|_| "Could not bind uniform for world".to_string())?;

        let objects = {
            if let Some(glb) = opt_glb {
                objects::new_of_glb(gl, glb).unwrap()
            } else {
                objects::new(gl).unwrap()
            }
        };
        Ok(Self {
            objects,
            shader_program,
            material_gl,
            world_gl,
        })
    }

    //fp make_instantiable
    pub fn make_instantiable<'inst>(
        &'inst self,
        gl: &mut G,
    ) -> Result<Instantiable<'inst, G>, String> {
        let instantiables = ShaderInstantiable::new(gl, &self.shader_program, &self.objects)
            .map_err(|_| "Failed to create shader instantiable".to_string())?;
        Ok(Instantiable::<G> { instantiables })
    }

    //fp make_instances
    pub fn make_instances<'inst>(&'inst self) -> Instances<'inst, G> {
        let instance = self.objects.instantiate();
        Instances { instance }
    }

    //fp update
    pub fn update(
        &self,
        gl: &mut G,
        game_state: &mut GameState,
        instantiable: &Instantiable<G>,
        instances: &mut Instances<G>,
    ) {
        // Update world_gl.gl_buffer world_data[0] (there is only one)
        // view_transformation.rotate_by(&spin);
        // world_data[0].view_matrix = view_transformation.mat4();

        gl.uniform_buffer_update_data(&self.world_gl, &game_state.world_data, 0);
        gl.use_program(Some(&self.shader_program));
        instantiable.instantiables.gl_draw(gl, &instances.instance);

        let v = [1., 1., 0.];
        instances
            .instance
            .transformation
            .translate(&v, 0.01 * game_state.time.sin());
        instances
            .instance
            .transformation
            .rotate_by(&game_state.spin);
        game_state.time += 0.05;
    }

    //zz All done
}

//a GameState
//tp GameState
pub struct GameState {
    world_data: [WorldData; 1],
    time: f32,
    view_transformation: model3d_base::Transformation,
    spin: model3d_base::Quat,
}

//ip GameState
impl GameState {
    pub fn new() -> Self {
        let time: f32 = 0.0;
        let view_transformation = model3d_base::Transformation::new();
        let spin = geo_nd::quat::rotate_x(&geo_nd::quat::identity(), 0.01);
        let mut world_data = [WorldData::default(); 1];
        world_data[0].view_matrix[0] = 1.;
        world_data[0].view_matrix[5] = 1.;
        world_data[0].view_matrix[10] = 1.;
        world_data[0].view_matrix[15] = 1.;
        world_data[0].lights[0].position = [2., 0., 0., 0.1];
        world_data[0].lights[0].color = [1., 0., 0., 0.];
        world_data[0].lights[1].position = [-1., 0., 0., 0.1];
        world_data[0].lights[1].color = [0., 1., 0., 0.];
        world_data[0].lights[2].position = [-1., 0., 0., -1.];
        world_data[0].lights[2].color = [0., 0., 1., 0.];

        Self {
            world_data,
            time,
            view_transformation,
            spin,
        }
    }
}
