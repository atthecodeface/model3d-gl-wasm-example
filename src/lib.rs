//a Imports
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod inner;
use inner::Inner;
pub mod shader;

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
    pub fn new(canvas: HtmlCanvasElement) -> CanvasWebgl {
        let inner = Inner::new(canvas).unwrap();
        Self { inner }
    }

    //mp shutdown
    /// Shut down the CanvasWebgl, removing any event callbacks for the canvas
    pub fn shutdown(&self) -> Result<(), JsValue> {
        self.inner.shutdown()
    }

    //mp fill
    /// Fill
    pub fn fill(&self) -> Result<(), JsValue> {
        Ok(self.inner.fill())
    }

    //zz All done
}
