//a Imports
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod inner;
use inner::Inner;
pub mod shader;

mod base_shader;
mod model;
mod objects;

//a CanvasWebgl - the external interface
//tp CanvasWebgl
/// A paint module that is attached to a Canvas element in an HTML
/// document, which uses mouse events etc to provide a simple paint
/// program
#[wasm_bindgen]
pub struct CanvasWebgl {
    inner: Rc<Inner>,
}

//ip CanvasWebgl
#[wasm_bindgen]
impl CanvasWebgl {
    //fp new
    /// Create a new CanvasWebgl attached to a Canvas HTML element,
    /// adding events to the canvas that provide the paint program
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<CanvasWebgl, JsValue> {
        console_error_panic_hook::set_once();
        let inner = Inner::new(canvas)?;
        Ok(Self { inner })
    }

    //mp shutdown
    /// Shut down the CanvasWebgl, removing any event callbacks for the canvas
    pub fn shutdown(&self) -> Result<(), JsValue> {
        self.inner.shutdown()
    }

    //mp create_f
    pub fn create_f(&mut self) -> Result<(), JsValue> {
        Ok(std::rc::Rc::get_mut(&mut self.inner).unwrap().create_f()?)
    }
    //mp fill
    /// Fill
    pub fn fill(&mut self) -> Result<(), JsValue> {
        Ok(std::rc::Rc::get_mut(&mut self.inner).unwrap().fill())
    }

    //zz All done
}
