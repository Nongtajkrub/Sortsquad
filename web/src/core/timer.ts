import Phaser from "phaser";
import {defaultFontConfig} from "./common";

interface CountdownTimerConfig {
	seconds: number,
	display?: { x: number, y: number, size?: string },
}

export default class CountdownTimer {
	private startTime!: number;
	private config!: CountdownTimerConfig;
	private scene!: Phaser.Scene;
	
	private calculateTimeLeft(scene: Phaser.Scene, config: CountdownTimerConfig) {
		return Math.max(
			config.seconds - (Math.floor(scene.time.now / 1000) - this.startTime), 0
		);
	}

	private createTimerDisplay(scene: Phaser.Scene, config: CountdownTimerConfig): void {
		if (config.display) {
			const timerDisplay = scene.add.text(
				config.display.x,
				config.display.y,
				(config.seconds).toString(),
				defaultFontConfig({ size: config.display.size ?? "32px" })
			).setOrigin(0.5, 0.5); 

			timerDisplay.setDepth(999);

			setInterval(() => {
				const timeLeft = this.calculateTimeLeft(scene, config);
				const redValue = 255 - (timeLeft * (255 / config.seconds));

				timerDisplay.setText(timeLeft.toString());
				timerDisplay.setColor(`rgb(${255}, ${255 - redValue}, ${255 - redValue})`);
			}, 500);
		}
	}

	constructor(scene: Phaser.Scene, config: CountdownTimerConfig) {
		this.startTime = Math.floor(scene.time.now / 1000); 
		this.scene = scene;
		this.config = config;
		this.createTimerDisplay(scene, config);
	}

	isFinish(): boolean {
		return (this.calculateTimeLeft(this.scene, this.config) == 0);
	}
}
