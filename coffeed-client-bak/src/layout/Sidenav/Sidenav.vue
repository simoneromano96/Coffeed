<template>
  <div id="mySidenav" class="sidenav">
    <ul class="nav">
      <li class="nav-link">
        <div class="wrapper">
          <router-link to="/login">
            <span class="icon-login icon"></span>
            <span class="nav-text">Login</span>
          </router-link>
        </div>
      </li>
    </ul>
  </div>
</template>

<style lang="scss" scoped>
@import "../../style/variables.scss";
/* The side navigation menu */
.sidenav {
  position: fixed;
  width: $sidenav-base-width;
  top: 0;
  left: 0;
  height: 100vh;
  z-index: 2;
  background-color: $black;
  overflow: hidden;
  transition: width 0.3s ease;
  box-shadow: 1px 2px 10px rgba(0, 0, 0, 0.3);
  &:hover {
    width: $sidenav-expanded-width;
  }

  .nav {
    list-style-type: none;
    color: $white;
    padding-left: 1rem;
    margin: 0;
    &:first-child {
      padding-top: 1rem;
    }

    .nav-link {
      padding-bottom: 1rem;
      width: $sidenav-expanded-width - 1rem;

      .wrapper {
        a {
          display: flex;
          align-items: center;
          justify-content: flex-start;
          cursor: pointer;
          text-decoration: none;
          color: $white;
          .icon,
          .nav-text {
            transition: color 0.5s;
          }
          .icon {
            font-size: 2em;
            height: 100%;
          }
          .nav-text {
            margin-left: 2rem;
            height: 100%;
          }
          &:hover {
            .icon,
            .nav-text {
              color: $tertiary;
            }
          }
        }
      }
    }
  }
}
</style>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator"

@Component
export default class Sidenav extends Vue {}
</script>