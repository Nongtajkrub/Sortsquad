import Phaser from "phaser";

import { Player } from "./sprites/player";

class MainScene extends Phaser.Scene {
	private player!: Player;
	private cursor!: Phaser.Types.Input.Keyboard.CursorKeys;

	constructor() {
		super({ key: "main" });
	}

	preload(): void {
		this.load.image("sky", "assets/environment/sky.jpg");

		this.load.spritesheet("organicBinIdle", "assets/bins/organic/idle.png", {
			frameWidth: 45,
			frameHeight: 45
		})
		this.load.spritesheet("organicBinPrerun", "assets/bins/organic/prerunning.png", {
			frameWidth: 45,
			frameHeight: 45
		})
		this.load.spritesheet("organicBinRunning", "assets/bins/organic/running.png", {
			frameWidth: 45,
			frameHeight: 45
		})
	}

	create(): void {
		this.cursor = this.input.keyboard!.createCursorKeys();

		this.player = new Player(this, {
			x: 200,
			y: 200,
			scale: 2,
			idleKey: "organicBinIdle",
			prerunKey: "organicBinPrerun",
			runningKey: "organicBinRunning"
		}); 
	}

	update(): void {
		this.player.update(this.cursor);
	}
}

const config: Phaser.Types.Core.GameConfig = {
	type: Phaser.AUTO,
	pixelArt: true,
	scale: {
		mode: Phaser.Scale.FIT,
		autoCenter: Phaser.Scale.CENTER_BOTH,
		width: window.innerWidth,
		height: window.innerHeight,
	},
	backgroundColor: "#242424",
	scene: MainScene,
	physics: {
		default: "arcade",
		arcade: {
			debug: false
		}
	}
};

new Phaser.Game(config);
