import { BACKEND_URL } from "./configuration"
import { client } from "./graphClient";

export class Coffee {
  public details?: string

  /**
   * A representation of a Coffee
   * @param id The coffee's UUID
   * @param name The coffee name
   * @param price The coffee's price
   * @param imageUrl Where to retrieve the coffee image
   */
  constructor(public id: string, public name: string, public price: number, public imageUrl: string) {
    this.imageUrl = `${BACKEND_URL}${imageUrl}`
  }

  fetchDetails = async () => {
    if (!this.details) {
      try {
        const query = `{
          coffee(id: "${this.id}") {
            details
          }
        }`

        let res: { coffee: { details: string } } = await client.request(query)
        this.details = res.coffee.details
      } catch (error) {
        console.log(error)
      }
    }
  }
}
