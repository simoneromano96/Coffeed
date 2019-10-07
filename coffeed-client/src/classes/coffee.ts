// GraphQL Client
// https://github.com/prisma-labs/graphql-request

class Details {
  constructor() {}
}

export class Coffee {
  details?: object

  constructor(private id: String, private name: string, private price: number) {}

  // API actions
  fetchDetails = (): Details => ({ a: "a" })

  // Getters
  getId = () => this.id
  getName = () => this.name
  getPrice = () => this.price
  getDetails = () => (this.details ? this.details : "Please buy me")
  getImage = () => require(`../assets/images/shop/${this.getName().toLowerCase()}.jpg`)

  // Setters
  setName = (name: string) => (this.name = name)
  setPrice = (price: number) => (this.price = price)
  setDetails = (details: object) => (this.details = details)
}
