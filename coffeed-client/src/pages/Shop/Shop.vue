<template>
  <div>
    <h1>Try and taste one of our coffees</h1>

    <div v-for="(coffee, index) in coffees" :key="coffee.id">
      <p>{{index}}</p>
    </div>
  </div>

  <!--
  <div class="coffee-list">
    <template repeat.for="coffee of coffees">
      <coffee-card coffee.bind="coffee" />
    </template>
  </div>
  -->
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

@Component
export default class Shop extends Vue {
  public coffees: Coffee[]

  mounted () {
    this.fetchCoffees()
  }

  fetchCoffees = async () => {
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

      // let res: { coffees: Coffee[] } = await client.request(query)
      // console.log(res);
      // this.coffees = res.coffees.map(coffee => new Coffee(coffee.id, coffee.name, coffee.price, coffee.imageUrl))
      // console.log(this.coffees)
      client.request(query).then((res: {coffees: Coffee[]}) => {
        // TODO!
        res.coffees.map(coffee => new Coffee(coffee.id, coffee.name, coffee.price, coffee.imageUrl))
      })
    } catch (error) {
      console.log(error)
    }
  }
}
</script>