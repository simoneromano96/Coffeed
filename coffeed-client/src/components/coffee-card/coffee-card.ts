import { bindable } from 'aurelia-framework';
import { Coffee } from "classes/coffee";

export class CoffeeCard {
  @bindable coffee: Coffee;
  loading: boolean;  

  fetchDetails = () => {
    this.loading = true;
    this.coffee.fetchDetails();
    this.loading = false;
  }
}
