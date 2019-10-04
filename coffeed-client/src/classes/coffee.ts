export class Coffee {
  details?: object;

  constructor(private name: string, private price: number){}

  // Getters
  getName = () => this.name;
  getPrice = () => this.price;
  getDetails = () => this.details;

  // Setters
  setName = (name: string) => this.name = name;
  setPrice = (price: number) => this.price = price;
  setDetails = (details: object) => this.details = details;
}
