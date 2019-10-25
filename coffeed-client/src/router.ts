import Vue from "vue"
import Router from "vue-router"
import Homepage from "./pages/Homepage/Homepage.vue"
import About from "./pages/About/About.vue"
import Shop from "./pages/Shop/Shop.vue"
import Login from "./pages/Login/Login.vue"

Vue.use(Router)

export default new Router({
  mode: "history",
  base: process.env.BASE_URL,
  routes: [
    {
      path: "/",
      name: "home",
      component: Homepage,
    },
    {
      path: "/shop",
      name: "shop",
      component: Shop,
    },
    {
      path: "/about",
      name: "about",
      component: About,
    },
    {
      path: "/login",
      name: "login",
      component: Login,
    },
  ],
})
