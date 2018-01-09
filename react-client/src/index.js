import React from 'react';
import ReactDOM from 'react-dom';
import ChatElement from './App';

function getOrCreateChatElement(){
  let chatElement = document.getElementById("#commenteRs");

  if(!chatElement){
    // if the element doesn't exist, a div is created at the end of the <body>
    chatElement = document.createElement("div");
    chatElement.id = "commenteRs";
    document.getElementsByTagName("body")[0].appendChild(chatElement);
  }
  return chatElement;
}

ReactDOM.render(<ChatElement backendUrl="http://localhost:8000"/>, getOrCreateChatElement());
