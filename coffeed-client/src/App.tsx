import Header from "./layout/header/Header"
import { Route } from "./classes/route"
import Router from "./router/router"
import { createSignal, onCleanup } from "solid-js"

const Home = () => (
  <>
    <h1>Welcome to this Simple Routing Example</h1>
    <p>Click the links in the Navigation above to load different routes.</p>
  </>
)

const Profile = () => (
  <>
    <h1>Your Profile</h1>
    <p>This section could be about you.</p>
  </>
)

const Settings = () => (
  <>
    <h1>Settings</h1>
    <p>All that configuration you never really ever want to look at.</p>
  </>
)

const routes = [{ path: "", component: Home }]

const App = () => {
  return (
    <div class="App">
      <Header />
      <ul>
        <li>
          <a href="#home">Home</a>
        </li>
        <li>
          <a href="#profile">Profile</a>
        </li>
        <li>
          <a href="#settings">Settings</a>
        </li>
      </ul>
      <Router routes={routes}></Router>
    </div>
  )
}

export default App
