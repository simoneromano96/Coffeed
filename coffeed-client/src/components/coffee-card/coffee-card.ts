import { bindable } from 'aurelia-framework';
import { Coffee } from "classes/coffee";

export class CoffeeCard {
  @bindable coffee: Coffee;

  fetchDetails = () => {
    this.coffee.fetchDetails();
  }
}
