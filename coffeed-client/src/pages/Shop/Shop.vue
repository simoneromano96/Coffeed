<template>
  <div>
    <h1>Try and taste one of our coffees</h1>
    <div class="coffee-list">
      <template v-for="coffee in coffees">
        <coffee-card :key="coffee.id" :coffee="coffee" />
      </template>
    </div>
  </div>
</template>

<style lang="scss" scoped>
h1 {
  text-align: center;
}

.coffee-list {
  display: flex;
  flex-wrap: wrap;
}
</style>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator"
import { client } from "../../classes/graphClient"
import { Coffee } from "../../classes/coffee"
import CoffeeCard from "../../components/CoffeeCard/CoffeeCard.vue"

const components = { CoffeeCard }

@Component({ components })
export default class Shop extends Vue {
  public coffees: Coffee[] = []

  mounted() {
    this.fetchCoffees()
  }

  async fetchCoffees() {
    try {
      const query = `
      {
        coffees {
          id
          name
          price
          imageUrl
        }
      }`

      let res: { coffees: Coffee[] } = await client.request(query)
      this.coffees = res.coffees.map(coffee => new Coffee(coffee.id, coffee.name, coffee.price, coffee.imageUrl))
    } catch (error) {
      console.log(error)
    }
  }
}
</script>