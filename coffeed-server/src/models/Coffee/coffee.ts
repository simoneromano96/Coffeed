const uuidv4 = require('uuid/v4');

class Details {
  constructor() {}
}

export class Coffee {
  details?: Details //TODO
  id: String

  /**
   * A representation of a Coffee
   * @param name The coffee name
   * @param price The coffee's price
   * @param imageUrl Where to retrieve the coffee image
   */
  constructor(public name: string, public price: number, public imageUrl: string) {
    this.id = uuidv4();
  }
}
