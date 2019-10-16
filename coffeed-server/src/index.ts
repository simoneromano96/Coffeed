import { prisma } from './generated/prisma-client'

// A `main` function so that we can use async/await
async function main() {
  const newCoffee = await prisma.createCoffee({ name: "Espresso", price: 1.0, imageUrl: "public/images/espresso.jpg" })
  console.log(newCoffee)
  console.log("___________________")
  const allCoffees = await prisma.coffees()
  console.log(allCoffees)
}

main().catch(e => console.error(e))
