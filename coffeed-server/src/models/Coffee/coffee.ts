const uuidv4 = require("uuid/v4")

export class Coffee {
  public id: String

  /**
   * A representation of a Coffee
   * @param name The coffee name
   * @param price The coffee's price
   * @param imageUrl Where to retrieve the coffee image
   * @param details Optional details about the coffee
   */
  constructor(public name: string, public price: number, public imageUrl: string, public details: string = null) {
    this.id = uuidv4()
  }
}
