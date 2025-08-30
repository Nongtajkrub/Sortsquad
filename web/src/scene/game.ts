import Phaser from "phaser";

import { Player } from "../sprites/player";
import TrashesManager from "../core/trashes-manager";
import config from "../../public/config.json"
import {defaultFontConfig} from "../core/common";

export default class GameScene extends Phaser.Scene {
	private player!: Player;
	private trashManager!: TrashesManager;
	private cursor!: Phaser.Types.Input.Keyboard.CursorKeys;

	constructor() {
		super({ key: "game" });
	}

	preload(): void {
		this.load.image("sky", "assets/environment/sky.jpg");

		this.load.spritesheet("organicBinIdle", config.path.bins.organic.idle, {
			frameWidth: 45,
			frameHeight: 45
		});
		this.load.spritesheet("organicBinPrerun", config.path.bins.organic.prerun, {
			frameWidth: 45,
			frameHeight: 45
		});
		this.load.spritesheet("organicBinRunning", config.path.bins.organic.running, {
			frameWidth: 45,
			frameHeight: 45
		});

		this.load.image("apple", config.path.trash.organic.apple);
		this.load.image("vegatable", config.path.trash.organic.vegatable);
		this.load.image("fishbone", config.path.trash.organic.fishbone);

		this.load.image("shoe", config.path.trash.general.shoe);
		this.load.image("tissue", config.path.trash.general.tissue);
		this.load.image("ciggarette", config.path.trash.general.ciggarette);

		this.load.image("waterbottle", config.path.trash.recyclable.waterBottle);
		this.load.image("coke", config.path.trash.recyclable.coke);
		this.load.image("newspaper", config.path.trash.recyclable.newspaper);
		
		this.load.image("electronic", config.path.trash.hazardous.electronic);
		this.load.image("battery", config.path.trash.hazardous.battery);
		this.load.image("bleach", config.path.trash.hazardous.bleach);

		this.load.font("PixelArt", config.path.font.pixelArt);
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
		}, 300);

		const timerDisplay = this.add.text(
			window.innerWidth / 2,
			80,
			"0",
			defaultFontConfig("96px")
		).setOrigin(0.5, 0.5);

		timerDisplay.setDepth(999);

		setInterval(() => {
			timerDisplay.setText(Math.floor(this.time.now / 1000).toString());
		}, 1000);
	}

	update(): void {
		this.player.update(this.cursor, this.trashManager);
		this.trashManager.update();
	}
}

