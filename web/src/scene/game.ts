import Phaser from "phaser";

import { Player } from "../sprites/player";
import TrashesManager from "../core/trashes-manager";
import config from "../../public/config.json"
import {defaultFontConfig} from "../core/common";
import type {TrashCategory} from "../core/trash-categories";
import Background from "../core/background";
import Grass from "../sprites/grass";

export default class GameScene extends Phaser.Scene {
	private player!: Player;
	private trashManager!: TrashesManager;
	private cursor?: Phaser.Types.Input.Keyboard.CursorKeys;
	private playerBinCategory!: TrashCategory;

	constructor() {
		super({ key: "game" });
	}

	init(data: { binCategory: TrashCategory }): void {
		this.playerBinCategory = data.binCategory;
	}

	private trashCategoryToAnimationPath(): {
		idle: string,
		prerun: string,
		running: string
	} {
		switch (this.playerBinCategory) {
			case "Organic":
				return {
					idle: config.path.bins.organic.idle,
					prerun: config.path.bins.organic.prerun,
					running: config.path.bins.organic.running,
				};
			case "General":
				return {
					idle: config.path.bins.general.idle,
					prerun: config.path.bins.general.prerun,
					running: config.path.bins.general.running,
				};
			case "Recyclable":
				return {
					idle: config.path.bins.recyclable.idle,
					prerun: config.path.bins.recyclable.prerun,
					running: config.path.bins.recyclable.running,
				};
			case "Hazardous":
				return {
					idle: config.path.bins.hazardous.idle,
					prerun: config.path.bins.hazardous.prerun,
					running: config.path.bins.hazardous.running,
				};
		}
	}

	preload(): void {
		this.load.image("sky", config.path.environment.sky);
		this.load.image("grass", config.path.environment.grass);

		const animationPath = this.trashCategoryToAnimationPath();

		this.load.spritesheet("binIdle", animationPath.idle, {
			frameWidth: 45,
			frameHeight: 45
		});
		this.load.spritesheet("binPrerun", animationPath.prerun, {
			frameWidth: 45,
			frameHeight: 45
		});
		this.load.spritesheet("binRunning", animationPath.running, {
			frameWidth: 45,
			frameHeight: 45
		});
		
		this.load.spritesheet("scoredAnimation1", config.path.gui.scoredAnimation1, {
			frameWidth: 32,
			frameHeight: 32
		});
		this.load.spritesheet("scoredAnimation2", config.path.gui.scoredAnimation2, {
			frameWidth: 32,
			frameHeight: 32
		});
		this.load.spritesheet("wrongedAnimation", config.path.gui.wrongedAnimation, {
			frameWidth: 32,
			frameHeight: 32
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

		this.load.image("leftArrowButton", config.path.gui.leftArrowButton);
		this.load.image("rightArrowButton", config.path.gui.rightArrowButton);
		this.load.image("correctCategoryCircle", config.path.gui.correctCategoryCircle);
		
		this.load.font("PixelArt", config.path.font.pixelArt);
	}

	create(): void {
		new Background(this, "sky", true);
		new Grass(this);

		this.cursor = this.input.keyboard?.createCursorKeys();

		this.player = new Player(this, {
			x: 200,
			y: window.innerHeight - 100,
			scale: 4,
			binCategory: this.playerBinCategory
		}); 

		this.trashManager = new TrashesManager(this, this.playerBinCategory);

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
		this.player.update(this.trashManager, this.cursor);
		this.trashManager.update();
	}
}

