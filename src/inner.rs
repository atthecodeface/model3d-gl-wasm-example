//a Imports
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::f64;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

use crate::shader;

//a Inner (and ClosureSet)
//ti ClosureSet
/// A dictionary of event name to closure, of event listeners added to
/// (e.g.) a Canvas
///
/// The closure set entries can be dropped, once they have been
/// removed from the element they were attached to as listeners
type ClosureSet = HashMap<&'static str, Closure<dyn FnMut()>>;

//tp Inner
/// The actual CanvasArt paint structure, with canvas and rendering
/// context, state, and closures
pub struct Inner {
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    vaos: RefCell<Vec<WebGlVertexArrayObject>>,
    closures: RefCell<ClosureSet>,
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

        let program = shader::compile_shader_program(&context)?;
        context.use_program(Some(&program));

        let closures = HashMap::new().into();
        let vaos = vec![].into();
        let inner = Self {
            canvas,
            context,
            program,
            closures,
            vaos,
        };
        let vao = inner.create_model()?;
        inner.vaos.borrow_mut().push(vao);
        Ok(inner.into())
    }

    //mp add_closures
    /// Add event listeners as required; they are also put into the
    /// ClosureSet so that they can be removed later, and the Inner
    /// (handled as a Rc<Inner>) will have its uses dropped as the
    /// Closures themselves are dropped; hence the Rc<Inner> should
    /// have no uses after this due to the event listeners that may
    /// have been added in the past.
    pub fn add_closures(self: &Rc<Self>) -> Result<(), JsValue> {
        {
            let inner = self.clone();
            self.add_closure("mousedown", move |event| inner.mouse_down(event))?;
        }
        {
            let inner = self.clone();
            self.add_closure("mouseup", move |event| inner.mouse_up(event))?;
        }
        {
            let inner = self.clone();
            self.add_closure("mousemove", move |event| inner.mouse_move(event))?;
        }
        Ok(())
    }

    //mp shutdown
    /// Remove all the event listeneres added (in the ClosureSet) and
    /// drop the closures
    ///
    /// This should be called prior to dropping the Inner so that it is not leaked.
    pub fn shutdown(&self) -> Result<(), JsValue> {
        let closures = self.closures.take();
        for (reason, closure) in closures.into_iter() {
            self.canvas
                .remove_event_listener_with_callback(reason, closure.as_ref().unchecked_ref())?
        }
        Ok(())
    }

    //mp fill
    /// Fill the canvas with transparent black
    pub fn fill(&self) {
        self.draw(3);
    }

    //mi add_closure
    /// Add a single event listener to the canvas given a callback
    /// function (that should match that required in terms of
    /// arguments)
    fn add_closure<Args, F>(
        self: &Rc<Self>,
        reason: &'static str,
        callback: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Args) + 'static,
        Args: wasm_bindgen::convert::FromWasmAbi + 'static,
    {
        let closure = Closure::<dyn FnMut(_)>::new(callback);
        self.canvas
            .add_event_listener_with_callback(reason, closure.as_ref().unchecked_ref())?;
        let closure = unsafe { std::mem::transmute::<_, Closure<dyn FnMut()>>(closure) };
        self.closures.borrow_mut().insert(reason, closure);
        Ok(())
    }

    //mi mouse_down
    /// The event handler for mouse being pressed
    fn mouse_down(&self, event: MouseEvent) {}

    //mi mouse_move
    /// The event handler for mouse moving, whether the button is pressed or not
    fn mouse_move(&self, event: MouseEvent) {}

    //mi mouse_up
    /// The event handler for mouse being released
    fn mouse_up(&self, event: MouseEvent) {}

    //mi create_model
    fn create_model(&self) -> Result<WebGlVertexArrayObject, String> {
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

        let position_attribute_location =
            self.context.get_attrib_location(&self.program, "position");
        let buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")?;
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let vao = self
            .context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        self.context.bind_vertex_array(Some(&vao));

        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context
            .enable_vertex_attrib_array(position_attribute_location as u32);
        Ok(vao)
    }

    //mp draw
    pub fn draw(&self, vert_count: i32) {
        self.context.bind_vertex_array(Some(&self.vaos.borrow()[0]));

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    }

    //zz All done
}
