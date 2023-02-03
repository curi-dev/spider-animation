import styled, { createGlobalStyle } from 'styled-components'


export const Container = styled.div`
    width: 100vw;
    height: 100vh;
    padding: 2px;
    border: 2px solid blue;

    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;

    & canvas {
        width: 60%;
        height: 60%;
    }

`

const GlobalStyle = createGlobalStyle`
    body {
        margin: 0;
        padding: 0;
    }

`

export default GlobalStyle
