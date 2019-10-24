<template>
  <div id="mySidenav" class="sidenav">
    <ul class="nav">
      <li class="nav-link">
          
        <a href>Home</a>
      </li>
    </ul>
  </div>
</template>

<style lang="scss" scoped>
@import "../../style/variables.scss";
/* The side navigation menu */
.sidenav {
  position: fixed;
  width: 60px;
  top: 0;
  left: 0;
  height: 100vh;
  z-index: 100;
  background-color: $black;
  overflow: hidden;
  transition: width 0.3s ease;
  cursor: pointer;
  box-shadow: 4px 7px 10px rgba(0, 0, 0, 0.4);
  &:hover {
    width: 15rem;
  }
  @media screen and (min-width: 600px) {
    width: 80px;
  }

  .nav {
    list-style-type: none;
    color: white;
    &:first-child {
      padding-top: 1.5rem;
    }

    .nav-link {
      padding-bottom: 4rem;
      width: 15rem;
      a {
        position: relative;
        display: block;
        top: -35px;
        padding-left: 25px;
        padding-right: 15px;
        transition: all 0.3s ease;
        margin-left: 25px;
        margin-right: 10px;
        text-decoration: none;
        color: white;
        font-family: "roboto";
        font-weight: 100;
        font-size: 1.35em;
        &:after {
          content: "";
          width: 100%;
          height: 100%;
          position: absolute;
          top: 0;
          left: 0;
          border-radius: 2px;
          background: radial-gradient(circle at 94.02% 88.03%, #54a4ff, transparent 100%);
          opacity: 0;
          transition: all 0.5s ease;
          z-index: -10;
        }
      }
      &:hover a:after {
        opacity: 1;
      }
      svg {
        width: 26px;
        height: 26px;
        position: relative;
        left: -25px;
        cursor: pointer;
        @media screen and(min-width:600px) {
          width: 32px;
          height: 32px;
          left: -15px;
        }
      }
    }
  }

  /*
  .close-button {
    position: absolute;
    top: 1rem;
    right: 1rem;
    font-size: 1.5rem;
    cursor: pointer;
    align-self: flex-start;
    color: $white;
  }
  */
}
</style>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator"

@Component
export default class Sidenav extends Vue {
  @Prop() private isOpen!: boolean

  closeSidenav() {
    this.$emit("close-sidenav")
  }
}
</script>