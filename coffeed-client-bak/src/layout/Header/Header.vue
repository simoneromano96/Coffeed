<template>
  <header>
    <img class="logo" src="../../assets/logo/logov3.svg" alt="Coffeed logo" />
    <nav>
      <ul>
        <li>
          <router-link to="/">Homepage</router-link>
        </li>
        <li>
          <router-link to="/about">About</router-link>
        </li>
        <li>
          <router-link to="/shop">Shop</router-link>
        </li>
        <li>
          <router-link to="/blog">Blog</router-link>
        </li>
        <li>
          <router-link to="#">Contact us</router-link>
        </li>
      </ul>
    </nav>
  </header>
</template>

<style lang="scss" scoped>
@import "../../style/variables.scss";

header {
  background-color: $black;
  display: flex;
  flex-flow: column;
  width: 100%;
  height: 100%;
  padding-top: 1rem;

  .menu-button {
    position: absolute;
    margin-left: 1rem;
    font-size: 1.5rem;
    cursor: pointer;
    align-self: flex-start;
    color: $white;
  }

  img.logo {
    height: 10rem;
    width: auto;
  }

  nav {
    display: flex;
    justify-content: center;
    margin-top: 0.25rem;
    margin-bottom: 1rem;

    ul {
      list-style-type: none;
      margin: 0;
      padding: 0;
      overflow: hidden;

      li {
        float: left;

        a {
          transition: all 0.5s;
          display: block;
          color: $white;
          text-align: center;
          padding: 14px 16px;
          text-decoration: none;
          border-bottom: 2px double $white;

          &:hover,
          &.router-link-exact-active {
            background-color: $primary;
            border-bottom: 2px double $tertiary;
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
export default class Header extends Vue {}
</script>