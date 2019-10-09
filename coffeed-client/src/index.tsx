import { render } from "solid-js/dom"
import "./style/index.scss"
import App from "./App"
import * as serviceWorker from "./serviceWorker"

render(App, document.getElementById("root") as any)

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister()
