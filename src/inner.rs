//a Imports
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use model3d_gl::Gl;
use model3d_gl::{GlProgram, Model3DWebGL};
type Program = <Model3DWebGL as model3d_gl::Gl>::Program;

use crate::shader;

//a Inner (and ClosureSet)
//ti F
struct F {
    base: Box<crate::model::Base<Model3DWebGL>>,
    instantiable: crate::model::Instantiable<'static, Model3DWebGL>,
    instances: RefCell<crate::model::Instances<'static, Model3DWebGL>>,
    game_state: RefCell<crate::model::GameState>,
}

//tp Inner
/// The actual CanvasArt paint structure, with canvas and rendering
/// context, state, and closures
pub struct Inner {
    /// We hold on to the canvas - this might be needed to keep
    /// control of the context, or it might just be a hangover from
    /// when this module used event listeners
    #[allow(dead_code)]
    canvas: HtmlCanvasElement,
    model3d: model3d_gl::Model3DWebGL,
    program: Program,
    vaos: RefCell<Vec<WebGlVertexArrayObject>>,
    f: Option<F>,
}

//ip Inner
impl Inner {
    //fp new
    /// Create a new Inner canvas paint structure given a Canvas element
    ///
    /// Does not add the event listeners (for no really good reason)
    pub fn new(canvas: HtmlCanvasElement) -> Result<Rc<Self>, JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let model3d = Model3DWebGL::new(context);
        let program = shader::compile_shader_program(&model3d)?;
        model3d.use_program(Some(&program));

        let f = None;

        let vaos = vec![].into();
        let inner = Self {
            canvas,
            model3d,
            program,
            vaos,
            f,
        };
        let vao = inner.create_model()?;
        inner.vaos.borrow_mut().push(vao);

        Ok(inner.into())
    }

    //mp shutdown
    /// Remove all the event listeneres added (in the ClosureSet) and
    /// drop the closures
    ///
    /// This should be called prior to dropping the Inner so that it is not leaked.
    pub fn shutdown(&self) -> Result<(), JsValue> {
        Ok(())
    }

    //mp fill
    /// Fill the canvas with transparent black
    pub fn fill(&mut self) {
        self.draw();
    }

    //mi create_model
    fn create_model(&self) -> Result<WebGlVertexArrayObject, String> {
        let vertices: &[f32] = &[
            -0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0, -1.0, 1.0, 0.0,
        ];

        let position_attribute_location = self
            .program
            .attributes()
            .iter()
            .find_map(|(n, v)| (*v == model3d_base::VertexAttr::Position).then_some(*n))
            .unwrap();
        let buffer = self
            .model3d
            .create_buffer()
            .ok_or("Failed to create buffer")?;
        self.model3d
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        // unsafe {
        // let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
        //
        // self.model3d.buffer_data_with_array_buffer_view(
        // WebGl2RenderingContext::ARRAY_BUFFER,
        // &positions_array_buf_view,
        // WebGl2RenderingContext::STATIC_DRAW,
        // );
        // }

        let len = std::mem::size_of_val(vertices);
        let data = &vertices[0] as *const f32 as *const u8;
        let data = unsafe { std::slice::from_raw_parts(data, len) };
        self.model3d.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            data,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let vao = self
            .model3d
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        self.model3d.bind_vertex_array(Some(&vao));

        self.model3d.vertex_attrib_pointer_with_i32(
            position_attribute_location,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.model3d
            .enable_vertex_attrib_array(position_attribute_location);
        self.model3d
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        Ok(vao)
    }

    //mp create_f
    pub fn create_f(&mut self) -> Result<(), String> {
        if self.f.is_some() {
            return Err("Already created".to_string());
        }
        let m = Box::new(crate::model::Base::new(&mut self.model3d, None)?);
        let f = {
            let m_ref =
                unsafe { std::mem::transmute::<_, &'static crate::model::Base<Model3DWebGL>>(&*m) };
            let instantiable = m_ref.make_instantiable(&mut self.model3d)?;
            let instances = m_ref.make_instances().into();
            let game_state = crate::model::GameState::new().into();
            F {
                base: m,
                instantiable,
                instances,
                game_state,
            }
        };
        self.f = Some(f);
        Ok(())
    }

    //mp create_f2
    pub fn create_f2(&mut self, glb: &[u8], node_names: &[&str]) -> Result<(), String> {
        if self.f.is_some() {
            return Err("Already created".to_string());
        }
        crate::console_log!("create_f2 create model {}", glb.len());
        let m = Box::new(crate::model::Base::new(
            &mut self.model3d,
            Some((glb, node_names)),
        )?);
        crate::console_log!("create_f2 created model");
        let f = {
            let m_ref =
                unsafe { std::mem::transmute::<_, &'static crate::model::Base<Model3DWebGL>>(&*m) };
            let instantiable = m_ref.make_instantiable(&mut self.model3d)?;
            let instances = m_ref.make_instances().into();
            let game_state = crate::model::GameState::new().into();
            F {
                base: m,
                instantiable,
                instances,
                game_state,
            }
        };
        self.f = Some(f);
        Ok(())
    }

    //mp draw
    pub fn draw(&mut self) {
        // self.model3d.enable(WebGl2RenderingContext::CULL_FACE);
        self.model3d.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.model3d.clear_color(0.0, 0.0, 0.0, 1.0);
        self.model3d.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let model3d = &mut self.model3d;
        if let Some(ref mut f) = self.f {
            let base = &f.base;
            let mut game_state = f.game_state.borrow_mut();
            let instantiable = &f.instantiable;
            let mut instances = f.instances.borrow_mut();
            base.update(model3d, &mut game_state, instantiable, &mut instances);
        }
    }

    //zz All done
}
