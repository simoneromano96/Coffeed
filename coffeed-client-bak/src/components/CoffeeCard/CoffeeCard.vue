<template>
  <div class="card">
    <span class="name">{{coffee.name}}</span>
    <div class="image-wrap" @click="fetchDescription()">
      <img :src="coffee.imageUrl" alt="Coffee image" class="image" />
      <div class="details">
        <div
          class="opaque"
          :class="coffee.description === '' ? 'show' : 'hide'"
        >Click to get my details!</div>
        <div class="loading" :class="loadingPercentage < 100 ? 'show' : 'hide'">
          <coffee-loader :id="coffee.id" :loading-percentage="loadingPercentage" :clicked="clicked"></coffee-loader>
        </div>
        <div
          class="animate"
          :class="coffee.description === '' || loadingPercentage < 100 ? 'hide' : 'show'"
        >{{coffee.description}}</div>
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

      .loading {
        display: flex;
        align-content: center;
        justify-content: center;
        transition: opacity 0.5s;
        &.show {
          opacity: 1;
        }
        &.hide {
          opacity: 0;
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
  clicked: boolean = false

  async fetchDescription() {
    this.clicked = true
    setInterval(() => {
      if (this.loadingPercentage <= 100) {
        this.loadingPercentage += 5
      }
    }, 200)
    await this.coffee.fetchDescription()
  }
}
</script>