import { GraphQLServer } from "graphql-yoga"
import * as express from "express";

import { Coffee } from "./models/Coffee/coffee";

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
  },
`

const coffees = [
  new Coffee("Mokaccino", 2, "/images/mokaccino.jpg"),
  new Coffee("Cappuccino", 1.75, "/images/cappuccino.jpg"),
  new Coffee("Espresso", 1.5, "/images/espresso.jpg"),
];

const resolvers = {
  Query: {
    coffees: (): Coffee[] => coffees,
    coffee: (parent, { id }): Coffee => coffees.find(coffee => coffee.id === id)
  },
}

const options = {
  port: 4001,
  endpoint: "/graphql",
  // subscriptions: '/subscriptions',
  playground: "/playground",
}

const server = new GraphQLServer({ typeDefs, resolvers })
server.express.use('/', express.static(__dirname + '/public'));

server.start(options, ({ port }) => console.log(`Server started, listening on port ${port} for incoming requests.`))
