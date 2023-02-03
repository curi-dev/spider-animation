import React from 'react'
import { createRoot } from 'react-dom/client'

const container = document.getElementById("root")
const root = createRoot(container)

import App from './App'

const wasm = import("../build/graphics_lib_bg")

wasm.then(() => {
    console.log("downloading .wasm")
    root.render(<App />)
});