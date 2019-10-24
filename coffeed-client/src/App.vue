<template>
  <main>
    <Sidenav v-on:close-sidenav="switchSidenav()" :isOpen="isSidenavOpen" />
    <Header v-on:open-sidenav="switchSidenav()" />
    <div class="main-view">
      <router-view />
    </div>
  </main>
</template>

<style lang="scss">
@import "./style/index.scss";

.main-view {
  padding: 0 2rem;
}
</style>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator"
import Header from "@/layout/Header/Header.vue"
import Sidenav from "@/layout/Sidenav/Sidenav.vue"

const components = { Header, Sidenav }

@Component({
  components,
})
export default class App extends Vue {
  public isSidenavOpen: boolean = false

  openSidenav() {
    this.isSidenavOpen = true
  }

  closeSidenav() {
    this.isSidenavOpen = false
  }

  switchSidenav() {
    this.isSidenavOpen = !this.isSidenavOpen
  }
}
</script>