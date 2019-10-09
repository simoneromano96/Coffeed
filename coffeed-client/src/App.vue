<template>
  <main>
    <Header />
    <div class="main-view">
      <router-view />
    </div>
  </main>
</template>

<style lang="scss">
@import "./style/index.scss";
</style>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator"
import Header from "@/layout/Header/Header.vue";

@Component({
  components: {
    Header
  }
})

export default class App extends Vue {}
</script>