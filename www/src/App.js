import React, {Component} from 'react'
import './App.css';
import * as wasm from "react-wasm";

export default class App extends Component {

    constructor(props) {
        super(props);
    }

    sayHello() {
        wasm.say_hello();
    }

    render() {
        return (
            <button onClick={this.sayHello}>
                Say hello
            </button>
        );
    }

}
