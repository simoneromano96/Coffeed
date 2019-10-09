import { bindable } from "aurelia-framework"
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
