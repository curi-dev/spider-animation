import React, { useLayoutEffect, memo } from 'react'
import { GraphicsClient } from '../build/graphics_lib_bg'

import GlobalStyle, { Container } from './styles'


const App = () => {

    useLayoutEffect(() => {
        const canvas = document.getElementById('glCanvas')
        const context = canvas.getContext('webgl')
        
        if (!context) {
            alert("A problem ocurred while trying to get context")
        }

        const FPS_THOTTLE = 1000 / 60
        let last_draw = -1
        let last_second = 0

        const graphicsClient = new GraphicsClient()
    
        // const pixelRatio = window.devicePixelRatio || 1
        // canvas.width = pixelRatio * canvas.clientWidth
        // canvas.height = pixelRatio * canvas.clientHeight
        // context.viewport(0, 0, canvas.width, canvas.height)
        
        function getContext() {
            window.requestAnimationFrame(getContext)
            
            let currTime = Date.now()

            if (currTime >= last_second + 1000) {
                //console.log("a second has passed: ", last_second)
                //graphicsClient.update_triangles()

                last_second = currTime
            }
        
            if (currTime >= last_draw + FPS_THOTTLE) {                
                last_draw = currTime

                graphicsClient.render()
            }
        
        }

        getContext()

    }, [])

    // async function handleClick() {
    //     await graphicsClient.update_triangles()
    //     setAddDisabled(true)
    //     setTimeout(() => {
    //         setAddDisabled(false)
    //     }, 1000)
    // }

    return (
            <>
                <Container>
                    {/* <button onClick={handleClick} disabled={addDisabled} >
                        Add triangle
                    </button> */}
                    <h1>
                        Hello Line
                    </h1>
                    <canvas id='glCanvas' ></canvas>
                </Container>
                <GlobalStyle/>
            </>
    )
}

export default memo(App)