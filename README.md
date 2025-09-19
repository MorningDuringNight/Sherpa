# Sherpa
CS1666 game design game: Katsuo

First manager is Ket-Hwa Cheng (MorningDuringNight KEC344)

Advanced topic group AI: Ket-Hwa (github: MorningDuringNight Pitt: KEC344), 
                         Tingxu (github: RaineyMisty Pitt: tic128), 
                         Xiaoting (github: XtWang299 Pitt: xiw323), 
                         Zhuoyan (github: qhynSamuel Pitt: zhc158)

Advanced topic group multiplayer: Jagger (github: Jagger-Hershey Pitt: JCH168),
                                  Alli (github: AlliBatell Pitt: ALB594),
                                  Sean (github: Sean-Shmulevich Pitt: szs23)

## game premise:

It is a 2D pixel-art platformer where cooperation is necessary. Players control several characters bound by a rope, to conquer the majestic and treacherous slopes of a mysterious mountain, you must master the art of collaborative climbing to reach the top.

The game is a 2d platformer where 2 players are chained together and must make it to the top of the mountain, and must collaborate to make it to the top. The player can at most move left, right, jump, jump left, and jump right and try their best to jump on platforms to ascend the mountain. But be aware there are obstacle that might or might not hinder your progress towards your goal.

Such things that you may encounter are moving platforms, spikes, large distances which require more effort, and even trampolines that may aid your ascent. 

<img width="1816" height="1656" alt="image" src="https://github.com/user-attachments/assets/37e76089-c64b-4bda-ba48-b0f3b335bc07" />

<img width="124" height="127" alt="image" src="https://github.com/user-attachments/assets/3d582d10-5948-4b25-a33e-1673e10fc0c4" />

<img width="162" height="120" alt="image" src="https://github.com/user-attachments/assets/06c7c7c6-2263-4f06-9ee6-7f125cee44ec" />


## Advanced topic AI: 
main implementation: AI created should be a Q learning model focused as a cooperative player for platforming, using a behavior tree to solve jumping. We are using a Q learning model since it is a state based learning algorithm which we can trim branches that do not meet specifications. How we will reward the AI is by adding coins to the map as both a reward to the players and a reward for the bot. These coins are completely optional otherwise. 

Q type learning while slow can provide a rather comprehnsive learning algorithm option with it's use as 2 players, thereby allowing the AI to learn independently and treat the other AI as just background variables.

## Advanced topic Multiplayer:
main implementation: multiplayer should be a client server model

server should have the same codebase which would be running/ simmulating the game. 

information that will be sent should be player positions, and getting that information flowing for communication between the clients and the server. The server should communicate with client to give other player position and be able to display on the screens where the other player has been moved.

Lag compensation: 
packet loss can disrupt synchronization between client and server. Our strategy depends on the duration of loss:

Short (<200ms) – Prediction: Client predicts movement/actions locally, server later corrects small errors. Keeps gameplay responsive.

Long (>1s) – Resync or Timeout: Prediction fails, client is disconnected/ghosted until stable.
## midterm goal:

AI - AI can move character it controls left, right, up, jump right, left jump. Meaning that at least the implementation for AI to interact with an enviorment is plausable and that while the decision tree itself may not be functional, it means that it can interact with the game world without learning yet.

Multiplayer - connect to a host client and both client and server can send and receive packages on both ends. 

maps - 1st map platforms, moving platform, sidewall collision. Players can land on or hit the side of each of these objects and it should have hit detection such that players will not go through these objects these will be 2 vertical screens, defined as being unable to see anythin from the previous screen.

rope implementation - rope physics slack, The rope implementation displays similar real world rope physics in terms of visual aesthetics, meaning the rope will be tight when farther away and grow loose when near.  

rope implementation - rope prevents players from moving a set distance apart from each other, this is based off the scharacter position relative to each other.

camera movement - A 2d camera that shows the general location of where characters on the screen and tracks as they move up and down on the screen, in a general location. There is no Left and right tracking

player movement - jump left, right, up, moving left and right from farther distances will have added momentum for greater velocity if direction has been constant.

## Finals goal: 

AI - two AI can play together and are able to be maneuver to at least 1 platforms from the ground on their own proving partial training - 5%

AI - two AI can play together and are able to be maneuver to at least 2 platforms from the ground on their own proving partial training - 7%

AI - two AI can play together and are able to be maneuver to at least 3 platforms from the ground on their own proving partial training - 8%

Multiplayer - at least 2 players can connect to the game and be able to play together, with the ability to coordinate with the other outloud and complete the map. - 20%

completed map - with moving platforms, hazards (spikes or trampoline) and standard platforms until reaching the top of the map. Spike implementation will cause instant game over, trampoline will give a jump boost to the player upon contact with the trampoline, and standard platforms are standing grounds. - 10%

completed map - The amount of these platforms with or without any hazards will be 30 platforms.  - 5%

movement - able to jump and walk on platforms - 5%

movement - jump off the other player character - 5%

movement - wall jump once off walls - 5%

One time ability- The players each get one time ability to put down a platform next to a wall which they can stand on. This platform exists for 10 real life seconds and will disapear once time has ellapsed. - 5%

finish - players finishing the first map achieve a completetion screen - 5%

### Stretch goal

1. We will have a leaderboard that saves the top 10 ten highest scores a player has scored, as well as if they achieved the score with ai or a different player. Allowing for a quick refrence for highest point climbed to.

2. we will have an enemy that starts from the top of the screen and falls straight down, targeting to hit player, these enemies will track where player characters are located and fall directly down like a thamp from mario, but not come back up.
