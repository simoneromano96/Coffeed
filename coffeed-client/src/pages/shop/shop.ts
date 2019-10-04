import { Coffee } from "classes/coffee";
import { bindable } from "aurelia-framework";

export class Shop {
  @bindable
  coffees: Coffee[] = [
    new Coffee("Moka", 2),
    new Coffee("Cappuccino", 1.75),
    new Coffee("Espresso", 1.5),
    new Coffee("Macchiato", 2),
    new Coffee("American", 9.99),
    new Coffee("CafèLatte", 2.25),
  ];
}
