import React, { useLayoutEffect, memo } from 'react'
import { GraphicsClient } from '../build/graphics_lib_bg'

import GlobalStyle, { Container } from './styles'


const App = (props) => {

    useLayoutEffect(() => {
        const canvas = document.getElementById('glCanvas')
        const context = canvas.getContext('webgl')
    
        if (!context) {
            alert("A problem ocurred while trying to get context")
        }

        const FPS_THOTTLE = 1000 / 60
        let last_draw = -1
        let initialTime = Date.now()
        
        const graphicsClient = new GraphicsClient(props.image)
        
        function getContext() {
            window.requestAnimationFrame(getContext)
            
            let currTime = Date.now()
        
            if (currTime >= last_draw + FPS_THOTTLE) {  
                let elapsedTime = (currTime - last_draw) / 1000
                
                last_draw = currTime
                let totalElapsedTime = currTime - initialTime

                graphicsClient.render()
            }
        
        }

        getContext()

    }, [])

   
    return (
            <>
                <Container>
                    <canvas id='glCanvas' tabIndex={"0"}></canvas>
                </Container>
                <GlobalStyle/>
            </>
    )
}

export default memo(App)