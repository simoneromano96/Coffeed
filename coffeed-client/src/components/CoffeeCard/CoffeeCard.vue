<template>
  <div class="card">
    <span class="name">{{coffee.name}}</span>
    <div class="image-wrap" @click="fetchDetails()">
      <img :src="coffee.imageUrl" alt="Coffee image" class="image" />
      <div class="details">
        <div
          class="opaque"
          :class="coffee.details === '' ? 'show' : 'hide'"
        >Click to get my details!</div>
        <div class="loading">
          <coffee-loader :id="coffee.id" :loading-percentage="loadingPercentage"></coffee-loader>
        </div>
        <div class="animate" :class="coffee.details === '' ? 'hide' : 'show'">{{coffee.details}}</div>
      </div>
    </div>
    <span class="price">{{coffee.price}} €</span>
  </div>
</template>

<style lang="scss" scoped>
@import "../../style/variables.scss";

.card {
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
  transition: 0.3s;
  padding: 1rem;
  margin: 1rem;
  width: calc(33% - 2rem);
  border-radius: 1rem;

  display: flex;
  flex-flow: column;

  .name {
    font-size: 1.15em;
  }

  .image-wrap {
    flex: 1 auto;
    position: relative;

    .image {
      width: 100%;
      height: 100%;
      border-radius: 0.5rem;
      object-fit: cover;
      transition: opacity 0.25s;
    }

    .details {
      position: absolute;
      top: 0;
      bottom: 0;
      left: 0;
      right: 0;
      background-color: rgba($color: $secondary, $alpha: 0.75);
      border-radius: 0.5rem;
      color: $white;
      opacity: 0;
      transition: opacity 0.25s;
      overflow: hidden;

      .opaque {
        transition: opacity 0.5s;
        padding: 1em;

        &.hide {
          opacity: 0;
        }
        &.show {
          opacity: 1;
        }
      }

      .animate {
        width: 100%;
        height: 100%;
        position: absolute;
        padding: 1em;
        bottom: 0;
        transition: transform 0.5s;

        &.hide {
          transform: translateY(100%);
        }
        &.show {
          transform: translateY(0%);
        }
      }
    }
  }

  .price {
    align-self: flex-end;
  }

  &:hover {
    box-shadow: 0 8px 16px 0 rgba(0, 0, 0, 0.2);
    cursor: pointer;

    .image-wrap {
      .details {
        opacity: 1;
      }
    }
  }
}
</style>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator"
import { Coffee } from "../../classes/coffee"
import CoffeeLoader from "../CoffeeLoader/CoffeeLoader.vue"

const components = { CoffeeLoader }

@Component({ components })
export default class CoffeeCard extends Vue {
  @Prop() private coffee!: Coffee
  loadingPercentage: number = 0

  async fetchDetails() {
    setInterval(() => {
      if (this.loadingPercentage <= 100) {
        this.loadingPercentage += 1
      }
    }, 100)
    await this.coffee.fetchDetails()
  }
}

/**
 * import { bindable } from "aurelia-framework"
import { Coffee } from "classes/coffee"

export class CoffeeCard {
  @bindable coffee: Coffee
  // loading: boolean
  loadingPercentage: number = 0

  fetchDetails = () => {
    this.loadingPercentage = 0
    this.coffee.fetchDetails()
    this.loadingPercentage = 100
  }

  getName = () => this.coffee.name
}
 */
</script>