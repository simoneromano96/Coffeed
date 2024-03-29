import { GraphQLServer } from "graphql-yoga"
import * as express from "express"

import { Coffee } from "./models/Coffee/coffee"

/**
 * GraphQL Scalar Types:
 * String
 * Boolean
 * Int
 * Float
 * ID
 *
 */

// Type definitions / Schema

const typeDefs = `
  type Coffee {
    id: ID!
    name: String!
    price: Float!
    imageUrl: String!
    details: String
  }

  type Query {
    coffees: [Coffee!]!
    coffee(id: String!): Coffee
  }

  type Mutation {
    createCoffee(data: CreateCoffeeInput): Coffee!
    deleteCoffee(id: ID!): Coffee!
  }

  input CreateCoffeeInput {
    name: String! 
    price: Float! 
    imageUrl: String! 
    details: String
  }
`

const coffees = [
  new Coffee(
    "Mocaccino",
    2,
    "images/mocaccino.jpg",
    "A caffè mocha, also called mocaccino, is a chocolate-flavored variant of a caffè latte.",
  ),
  new Coffee(
    "Cappuccino",
    1.75,
    "images/cappuccino.jpg",
    "A cappuccino is an espresso-based coffee drink that originated in Italy, and is traditionally prepared with steamed milk foam (microfoam).Variations of the drink involve the use of cream instead of milk, and flavoring with cinnamon or chocolate powder.",
  ),
  new Coffee("Espresso", 1.5, "images/espresso.jpg"),
]

const resolvers = {
  Query: {
    coffees: (): Coffee[] => coffees,
    coffee: (parent, { id }): Coffee => coffees.find(coffee => coffee.id === id),
  },
  Mutation: {
    createCoffee: (parent, args, ctx, info): Coffee => {
      const { name, price, imageUrl, details } = args.data
      const nameTaken: boolean = coffees.some(coffee => coffee.name === name)
      if (nameTaken) {
        throw new Error(`Coffee "${name}" already exists!`)
      }
      const newCoffee = new Coffee(name, price, imageUrl, details)
      coffees.push(newCoffee)
      return newCoffee
    },
    deleteCoffee: (parent, args, ctx, info): Coffee => {
      const { id } = args
      const toDeleteIndex = coffees.findIndex(coffee => id === coffee.id)
      if (toDeleteIndex !== -1) {
        return coffees.splice(toDeleteIndex, 1)[0]
      } else {
        throw Error(`Could not find coffee with id: ${id}`)
      }
    },
  },
}

const options = {
  port: 4001,
  endpoint: "/graphql",
  // subscriptions: '/subscriptions',
  playground: "/playground",
}

const server = new GraphQLServer({ typeDefs, resolvers })
server.express.use("/", express.static(__dirname + "/public"))

server.start(options, ({ port }) => console.log(`Server started, listening on port ${port} for incoming requests.`))
