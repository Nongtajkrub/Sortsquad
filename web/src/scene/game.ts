import Phaser from "phaser";

import { Player } from "../sprites/player";
import TrashesManager from "../core/trashes-manager";
import config from "../../public/config.json"
import type {TrashCategory} from "../core/trash-categories";
import Background from "../core/background";
import Grass from "../sprites/grass";
import CountdownTimer from "../core/timer";

export default class GameScene extends Phaser.Scene {
	private player!: Player;
	private playerBinCategory!: TrashCategory;
	private trashManager!: TrashesManager;
	private countdownTimer!: CountdownTimer;
	private music!: Phaser.Sound.BaseSound;

	private cursor?: Phaser.Types.Input.Keyboard.CursorKeys;

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
		const loadingScreen = new Background(this, "loadingScreen");

		this.load.on('complete', () => {
			loadingScreen.destroy();
		});

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
		
		this.load.audio("music", config.path.audio.music);
		this.load.audio("scoredAudio", config.path.audio.scored);

		this.load.font("pixelArt", config.path.font.pixelArt);
	}

	private createEnvironment(): void {
		new Background(this, "sky", true);
		new Grass(this);
	}

	private createMusic(): void {
		this.music = this.sound.add("music");

		this.music.play({
			loop: true,
			seek: 68,
		});
	}

	private createTrashManager(): void {
		this.trashManager = new TrashesManager(this, this.playerBinCategory);

		setInterval(() => {
			this.trashManager.spawn();
		}, 300);
	}

	create(): void {
		this.createEnvironment();
		this.createMusic();

		this.cursor = this.input.keyboard?.createCursorKeys();

		this.player = new Player(this, {
			x: this.scale.width / 2,
			y: window.innerHeight - 100,
			scale: 4,
			binCategory: this.playerBinCategory
		}); 

		this.createTrashManager();
		
		this.countdownTimer = new CountdownTimer(this, {
			seconds: config.game.time,
			display: {
				x: this.scale.width / 2,
				y: 80,
				size: "96px"
			},
		});
	}

	private timerUpdate(): void {
		if (this.countdownTimer.isFinish()) {
			this.music.destroy();
			this.player.destroy();

			this.trashManager.setSpawnable(false);
			this.trashManager.clear();

			this.scene.start("end", { playerScore: this.player.getScore() });
		}
	}

	update(): void {
		this.player.update(this.trashManager, this.cursor);
		this.trashManager.update();
		this.timerUpdate();
	}
}
