import Phaser from "phaser";

import GameScene from "./scene/game"; 
import MenuScene from "./scene/menu";
import EndScene from "./scene/end";

const phaserConfig: Phaser.Types.Core.GameConfig = {
	type: Phaser.AUTO,
	pixelArt: true,
	fps: {
		target: 60,
	},
	scale: {
		mode: Phaser.Scale.FIT,
		autoCenter: Phaser.Scale.CENTER_BOTH,
		width: window.innerWidth,
		height: window.innerHeight,
	},
	backgroundColor: "#242424",
	scene: [MenuScene, GameScene, EndScene],
	physics: {
		default: "arcade",
		arcade: {
			debug: false
		}
	}
};

new Phaser.Game(phaserConfig);
