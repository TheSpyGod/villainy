## Game launcher

### Requirements:
- Game select page
- Launch game button


#### Steps:
- Get the logic for legendary to work
- Hook up legendary to the Game Page UI
- Make the "Launch Game" button work 
- - Configure Proton Compatibility Layer


#### Structure
backend ---> legendary connection ---> legendary configuration
```

/legendary
//config.rs
//initialization.rs
//controller.rs

```

#### Current features:
-[x] System checks
-[x] Game launch
-[x] Backend <-> Frontend communication

#### TO-DO 
-[] Generated grid of clickable icons for games, taking the "name" from the image itself to launch it.
-[] Instead of launching it VIA Legendary, search for the .exe and use Proton
