//a Imports
import init, {CanvasWebgl} from "./pkg/canvas_webgl.js";
// import init, {WasmProject, WasmCip, WasmCameraDatabase, WasmCameraInstance, WasmNamedPoint, WasmNamedPointSet, WasmPointMappingSet, WasmRay} from "../pkg/image_calibrate_wasm.js";
// import {Directory, FileSet} from "./files.js";
// import {Log} from "./log.js";
// import * as html from "./html.js";
// import * as utils from "./utils.js";


class Thing {
    constructor() {
        this.run_step_pending = false;
        this.animating = false;
        this.filename = "ToyCar.glb";
        this.node_names = ["0"];
    }
    set_animating(a) {
        console.log("Set animating",a, this.run_step_pending);
        if (a) {
            if (this.run_step_pending) {return;}
            this.animating = true;
            this.init_time = Date.now();
            this.run_step();
        } else {
            this.animating = false;
        }
    }
    run_step() {
        this.run_step_pending = false;
        if (this.animating) {
            this.time_last = this.time;
            this.time = (Date.now() - this.init_time) * 0.001;
            //            this.handle_tick(this.time, this.time_last);
            window.canvas_webgl.fill();
            requestAnimationFrame(()=>this.run_step());
            this.run_step_pending = true;
        }
    }

    //mp fetch_glb
    async fetch_glb() {
        return fetch(this.filename)
            .then((response) => {
                if (!response.ok) {
                    throw new Error(`Failed to fetch interesting points: ${response.status}`);
                }
                return response.blob();
            })
    }

    load_glb() {
        const me = this;
        let promises = [];
        promises.push(
                this.fetch_glb()
                .then((b) => {
                    console.log(b);
                    return b.arrayBuffer();
                    })
                .then((m) => {
                    console.log("Give it buffer", m);
                    window.canvas_webgl.create_f2(m);
                    console.log("Created");
                    })
                    .catch((err) => console.error(`Fetch problem: ${err.message}`))
            );
        Promise.all(promises).then(() => {});
        // ;
    }
}

//a Top level on load
window.addEventListener("load", (e) => {
    init().then(() => {
        var canvas = document.getElementById('canvas');
        console.log(canvas);
        window.canvas_webgl = new CanvasWebgl(canvas);
        window.thing = new Thing();
    });
               });
