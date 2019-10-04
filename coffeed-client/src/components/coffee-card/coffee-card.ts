import { bindable } from 'aurelia-framework';
import { Coffee } from "classes/coffee";

export class CoffeeCard {
  @bindable coffee: Coffee;

  getDetails = () => {
    return this.coffee.getDetails();
  }
}
