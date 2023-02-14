# Zooliens

Zooliens is a GameFi project similar to Pokemon Go, built on Vara Network, a
wasm smart contract platform, and written in Rust. The project aims to bring the
fun of collecting, training, and battling creatures to the blockchain space,
allowing players to interact with digital creatures called Zooliens.

## Game Mechanism

Zooliens is a game where players can explore the virtual world to find and
capture Zooliens, train them, and compete with other players. Here is an
overview of the game mechanism:

### Zooliens

There are various Zooliens in the game, each with unique attributes such as
type, attack, defense, and speed. Players can capture Zooliens by throwing a
virtual ball, and add them to their collection. Each Zoolien will have the
following attributes:

- Picture: An image representing the Zoolien
- Name: A unique name for the Zoolien chosen by the player
- Level: The current level of the Zoolien
- Strength: The attack power of the Zoolien
- Agility: The speed of the Zoolien
- Intelligence: The ability to use special moves
- Stamina: The amount of health the Zoolien has

### Exploration

Players can explore the game world to find new Zooliens and collect resources
such as food and items. The game world will consist of different regions with
different types of Zooliens, and the rarity of Zooliens will vary based on their
type and region.

### Training

Players can train their Zooliens to increase their attributes, such as strength,
agility, intelligence, and stamina. The more a Zoolien is trained, the stronger
it becomes.

### Battles

Players can battle other players' Zooliens to gain experience and level up.
Battles will take place in real-time, and the winner will receive rewards such
as experience points and rare items.

### Trading

Players can trade Zooliens and game items with other players using Vara
Network's smart contract platform. The rarity of a Zoolien and its attributes
will determine its trading value.

## Frontend

Zooliens will have a web-based frontend with three parts:

### Profile

The profile page will display the attributes of each Zoolien, including picture,
name, level, strength, agility, intelligence, and stamina. Players can view and
manage their Zooliens on this page.

### Shop

The shop page will allow users to buy and sell Zooliens and game items using
Vara Network's smart contract platform. Players can browse the marketplace and
make transactions with other players.

### Adventure

The adventure page is where the gameplay takes place. Players can explore the
game world, collect resources, and battle other players' Zooliens. The more
resources a player collects, the higher the chance of finding rare Zooliens.

## Smart Contract

Zooliens is built on Vara Network, a wasm smart contract platform. The smart
contract exposes several functions that allow players to interact with the game
world. Here are the main functions exposed by the contract:

- `capture_zoolien`: Allows a player to capture a Zoolien by spending resources.
  The function generates a new Zoolien with random attributes and adds it to the
  player's collection.

- `train_zoolien`: Allows a player to train their Zoolien by spending resources.
  The function increases the Zoolien's attributes such as strength, agility,
  intelligence, and stamina.

- `battle_zoolien`: Allows a player to battle another player's Zoolien by
  spending resources. The function compares the attributes of both Zooliens and
  determines the winner based on a random algorithm
