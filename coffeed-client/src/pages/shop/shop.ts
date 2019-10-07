import { bindable } from "aurelia-framework"

import { client } from "../../classes/graphClient"
import { Coffee } from "classes/coffee"

export class Shop {
  @bindable
  coffees: Coffee[] = []

  constructor() {
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

      let res: { coffees: Coffee[] } = await client.request(query)
      this.coffees = res.coffees.map(coffee => new Coffee(coffee.id, coffee.name, coffee.price, coffee.imageUrl))
    } catch (error) {
      console.log(error)
    }
  }
}
