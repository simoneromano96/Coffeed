<template>
  <main>
    <Sidenav />
    <Header />
    <div class="main-view">
      <router-view />
    </div>
  </main>
</template>

<style lang="scss">
@import "./style/index.scss";
@import "./style/variables.scss";

.main-view {
  padding: 0 2rem 0 2rem + $sidenav-base-width;
}
</style>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator"
import Header from "./layout/Header/Header.vue"
import Sidenav from "./layout/Sidenav/Sidenav.vue"

const components = { Header, Sidenav }

@Component({
  components,
})
export default class App extends Vue {}
</script>