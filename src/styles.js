import styled, { createGlobalStyle } from 'styled-components'


export const Container = styled.div`
    width: 100vw;
    height: 100vh !important;
   
    /* display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: repeat(2, 1fr);
     */

    canvas {
        width: 100%;
        height: 450px;
        //height: 100%;
        /* width: 900px;
        height: 700px; */
    }



`

const GlobalStyle = createGlobalStyle`
    body {
        margin: 0;
        padding: 0;
    }

`

export default GlobalStyle
