import { client } from "./graphClient"

export class Coffee {
  public description?: string

  /**
   * A representation of a Coffee
   * @param id The coffee's UUID
   * @param name The coffee name
   * @param price The coffee's price
   * @param imageUrl Where to retrieve the coffee image
   */
  constructor(public id: string, public name: string, public price: number, public imageUrl: string) {
    this.description = ""
  }

  fetchDescription = async () => {
    // Ask for description only once
    if (!this.description) {
      try {
        const query = `{
          coffee(id: "${this.id}") {
            description
          }
        }`

        const res: { coffee: { description: string } } = await client.request(query)
        const description = res.coffee.description
        this.description = description && description.length > 0 ? description : "No description available 😥"
      } catch (error) {
        console.log(error)
      }
    }
  }
}
