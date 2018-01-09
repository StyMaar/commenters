function commenteRs (url){
  if(!url){
    throw new Error("Please provide the url of a commenteRs back-end");
  }

  let doLoad = loadData(url);
  doLoad.then(()=>{});
}

async function loadData(url){
  let myRequest = new Request(`${url}/comments`);
  let response = await(await fetch(myRequest)).json();

  let chatElement = getOrCreateChatElement();
  console.debug(response);
  createPostingArea(chatElement);
  for ( {message, author} of response){
    addMessageElement(chatElement,message, author);
  }
}

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

function addMessageElement(chatElement, message, author){
  let messageElement = document.createElement("div");
    let authorElement = document.createElement("h4");
      var authorText = document.createTextNode(author);
      authorElement.appendChild(authorText);
    messageElement.appendChild(authorElement);
    let textElement = document.createElement("p");
      var messageText = document.createTextNode(message);
      textElement.appendChild(messageText);
    messageElement.appendChild(textElement);
  chatElement.appendChild(messageElement);
}

function createPostingArea(chatElement){
  let postingAreaTitle = "Laissez un commentaire";

  let postAreaElement = document.createElement("div");
    let postingAreaTitleElement = document.createElement("h4");
      var postingAreaTitleText = document.createTextNode(postingAreaTitle);
      postingAreaTitleElement.appendChild(postingAreaTitleText);
    postAreaElement.appendChild(postingAreaTitleElement);
    let postingArea = document.createElement("textarea");
    postAreaElement.appendChild(postingArea);
    let postingButton = document.createElement("button");
      var buttonCaption = document.createTextNode("Envoyer");
      postingButton.appendChild(buttonCaption);
      postingButton.onclick = function (){
        let textarea = document.querySelector("#commenteRs textarea");
        console.log(textarea.value);
      }
    postAreaElement.appendChild(postingButton);

  chatElement.appendChild(postAreaElement);
}


commenteRs("poney");
