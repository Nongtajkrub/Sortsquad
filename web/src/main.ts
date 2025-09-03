import Phaser from "phaser";
import GameScene from "./scene/game"; 
import MenuScene from "./scene/menu";

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
	scene: [MenuScene, GameScene],
	physics: {
		default: "arcade",
		arcade: {
			debug: true
		}
	}
};

new Phaser.Game(phaserConfig);
