import React, {useState, useEffect} from 'react';
import logo from './logo.svg';
import './App.css';
import * as lesss from 'lesss';

function App() {
useEffect(() => {
console.log(lesss);
}, [])
  const [content, setContent] = useState("a {}")
  return (
    <div className="App">
        <textarea name="" id="" value={content} onChange={e => setContent(e.target.value)}></textarea>
        {lesss.parse(content)}
    </div>
  );
}

export default App;
