import Phaser from "phaser";

import { Player } from "./sprites/player";
import TrashesManager from "./core/trashes-manager";

class MainScene extends Phaser.Scene {
	private player!: Player;
	private trashManager!: TrashesManager;
	private cursor!: Phaser.Types.Input.Keyboard.CursorKeys;

	constructor() {
		super({ key: "main" });
	}

	preload(): void {
		this.load.image("sky", "assets/environment/sky.jpg");

		this.load.spritesheet("organicBinIdle", "assets/bins/organic/idle.png", {
			frameWidth: 45,
			frameHeight: 45
		});
		this.load.spritesheet("organicBinPrerun", "assets/bins/organic/prerunning.png", {
			frameWidth: 45,
			frameHeight: 45
		});
		this.load.spritesheet("organicBinRunning", "assets/bins/organic/running.png", {
			frameWidth: 45,
			frameHeight: 45
		});

		this.load.image("apple", "assets/trahse/organic/apple.png");
		this.load.image("vegatable", "assets/trahse/organic/vegatable.png");
		this.load.image("shoe", "assets/trahse/general/shoe.png");

		this.load.font("PixelArt", "assets/fonts/font.otf");
	}

	create(): void {
		this.cursor = this.input.keyboard!.createCursorKeys();

		this.player = new Player(this, {
			x: 200,
			y: window.innerHeight - 100,
			scale: 4,
			idleKey: "organicBinIdle",
			prerunKey: "organicBinPrerun",
			runningKey: "organicBinRunning"
		}); 

		this.trashManager = new TrashesManager(this);

		setInterval(() => {
			this.trashManager.spawn();
		}, 500);
	}

	update(): void {
		this.player.update(this.cursor, this.trashManager);
		this.trashManager.update();
	}
}

const config: Phaser.Types.Core.GameConfig = {
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
	scene: MainScene,
	physics: {
		default: "arcade",
		arcade: {
			debug: false
		}
	}
};

new Phaser.Game(config);
