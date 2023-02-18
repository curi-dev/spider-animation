import React from 'react'
import { createRoot } from 'react-dom/client'

const container = document.getElementById("root")
const root = createRoot(container)

import uri from "./assets/images/world_wide.jpg"

import App from './App'

const wasm = import("../build/graphics_lib_bg")

wasm.then(() => {
    console.log("downloading .wasm")
    var image = new Image();
    image.src = uri
    console.log("image: ", image)
    
    console.log("image.onload")
    root.render(<App image={image} />)
    
});