import React, { useState, useEffect } from 'react';
import './App.css';
import * as lesss from 'lesss';

function App() {
  useEffect(() => {
    console.log(lesss);
  }, [])
  const [content, setContent] = useState("a {}")
  const [parsed, setParsed] = useState("")

  useEffect(() =>{
    try{
      setParsed(lesss.parse(content));
    }catch(e) {
      setParsed("internal error")
    }
  }, [content])
  return (
    <div className="container">
      <div className="left">
        <h1>Lesss</h1>
        <textarea className='input' value={content} onChange={e => setContent(e.target.value)}></textarea>
      </div>

      <div className="right">
        <code>
          {parsed}
        </code>
      </div>

    </div>
  );
}

export default App;
