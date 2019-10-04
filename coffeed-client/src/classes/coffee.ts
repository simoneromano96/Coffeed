import {  } from 'module';
// GraphQL Client
// https://github.com/prisma-labs/graphql-request
// https://github.com/khaosdoctor/gotql

class Details {
  constructor() {
    
  }
}

export class Coffee {
  details?: object;

  constructor(private name: string, private price: number) { }

  // API actions
  fetchDetails = (): Details => {
    return "a";
  }

  // Getters
  getName = () => this.name;
  getPrice = () => this.price;
  getDetails = () => this.details ? this.details : "Please buy me";
  getImage = () => require(`../assets/images/shop/${this.getName().toLowerCase()}.jpg`)

  // Setters
  setName = (name: string) => this.name = name;
  setPrice = (price: number) => this.price = price;
  setDetails = (details: object) => this.details = details;
}
