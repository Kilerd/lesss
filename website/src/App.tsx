import React, { useState, useEffect } from 'react';
import './App.css';
import * as lesss from 'lesss';

function App() {
  useEffect(() => {
    console.log(lesss);
  }, [])
  const [content, setContent] = useState("a {}")
  return (
    <div className="container">
      <div className="left">
        <h1>Lesss</h1>
        <textarea className='input' value={content} onChange={e => setContent(e.target.value)}></textarea>
      </div>
      <div className="right">
        {lesss.parse(content)}
      </div>

    </div>
  );
}

export default App;
