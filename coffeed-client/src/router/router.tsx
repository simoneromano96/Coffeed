import { Switch, Match, For } from "solid-js/dom"
import { createSignal, onCleanup } from "solid-js"
import { Route } from "../classes/route"

function createRouteHandler() {
  const [location, setLocation] = createSignal(""),
    locationHandler = () => setLocation(window.location.hash.slice(1))
  window.addEventListener("hashchange", locationHandler)
  onCleanup(() => window.removeEventListener("hashchange", locationHandler))
  return (match: string) => {console.log(match, location(), match === location()); return match === location()}
}

const Router = ({routes}) => {
  console.log(routes);
  console.log(routes[0].path);
  console.log(routes[0].component);
  console.log(routes[0].component())
  const matches = createRouteHandler();

  console.log (
  )

  return <p>porco dio</p>
}

export default Router
