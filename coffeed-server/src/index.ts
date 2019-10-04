/*
import myAddFunction, { subtract } from './math'

console.log(myAddFunction(1, -2))
console.log(subtract(10, 2))
*/
import { GraphQLServer } from "graphql-yoga"

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
  type User {
    id: ID!,
    name: String!,
    email: String!,
    age: Int
  }

  type Query {
    me: User
    greeting(name: String): String
  },
`

const resolvers = {
  Query: {
    me: () => ({
      id: "abc123",
      name: "Simone Romano",
      email: "simone@simoneromano.eu",
      age: null,
    }),
    greeting: (parent, { name }, context, info) => `Hello, ${name || "anonymous"}`
  },
}

const options = {
  port: 4001,
  endpoint: "/graphql",
  // subscriptions: '/subscriptions',
  playground: "/playground",
}

const server = new GraphQLServer({ typeDefs, resolvers })

server.start(options, ({ port }) => console.log(`Server started, listening on port ${port} for incoming requests.`))
