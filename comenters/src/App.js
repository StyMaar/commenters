import React, { Component } from 'react';
import axios from 'axios';

class ChatElement extends Component {

  // pour mettre à jour seulement une propriété du state sans toucher aux autres
  updateState(object){
    this.setState(Object.assign({}, this.state, object));
  }

  constructor(props) {
    super(props);
    this.state = {
      commentList:[],
      articleId: window.location.pathname.split( "/" ).pop(), // récupère la fin de l'url
    };
  }

  async componentDidMount() {
    try {
      let {data} = await axios.get(`${this.props.backendUrl}/${this.state.articleId||"root_page"}/comments`);
      let commentList = data.map(({author,message,uuid, date})=> ({author, message, key:uuid, date}));
      this.updateState({commentList});
    } catch (e){
      console.error(e);
    }
 }

  renderCommentList() {
    return this.state.commentList.map(({author, date, message, key}) => <Comment id={key} key={key} author={author} date={date} message={message} />);
  }

  async postMessage(author, text) {
    let response = await axios.post(`${this.props.backendUrl}/${this.state.articleId||"root_page"}/comments`, {
      text,
      author,
    })
    let {message: formattedMessage, uuid, author: formattedAuthor, date} = response.data;
    let comment = {message: formattedMessage, key: uuid, author: formattedAuthor, date};
    this.updateState({ commentList: [comment, ...this.state.commentList] });
  }

  render() {
    return (
      <div>
        <header>
          <h1>Commentaires</h1>
        </header>
        <NewCommentArea handleSubmit={this.postMessage.bind(this)}/>
        <div className="commentList">
          { this.renderCommentList() }
        </div>
      </div>
    );
  }
}

class Comment extends Component {
  render() {
    let date = new Date(this.props.date);
    let formattedDateString = date.toLocaleString();

    let permalink = `${window.location}#${this.props.id}`;

    return (
      <div className="comment" id={this.props.id}>
        <header>
          <h3>{this.props.author}</h3>
          <h3>Le {formattedDateString}</h3>
        </header>
        <p>{this.props.message}</p>
        <div className="permalink">
          <a href={permalink}>Lien permanent vers le commentaire</a>
        </div>
      </div>
    );
  }
}

class NewCommentArea extends Component {

  constructor(props) {
    super(props);
    this.state = {
      author: "",
      comment: "",
    };
  }

  handleSubmit(event){
    let {author, comment} = this.state;
    this.props.handleSubmit(author, comment);
    event.preventDefault();
  }

  handleNameChange(event) {
    this.setState(Object.assign({}, this.state,{author: event.target.value}));
  }

  handleCommentChange(event) {
    this.setState(Object.assign({}, this.state,{comment: event.target.value}));
  }

  render() {
    return (
      <form onSubmit={this.handleSubmit.bind(this)}>
        <label>
          Votre nom :
          <input type="text" value={this.state.author} onChange={this.handleNameChange.bind(this)} />
        </label>
        <label>
          Votre commentaire :
          <textarea value={this.state.comment} onChange={this.handleCommentChange.bind(this)} />
        </label>
        <input type="submit" value="Submit" />
      </form>
    );
  }
}


export default ChatElement;
