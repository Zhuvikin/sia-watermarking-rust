import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import init, { create_image } from "react-image";

function App() {
   const [ans, setAns] = useState(0);
   useEffect(() => {
     init().then(() => {
       setAns(create_image(256, 256));
     })
   }, [])
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <p>1 + 1 = {ans}</p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
