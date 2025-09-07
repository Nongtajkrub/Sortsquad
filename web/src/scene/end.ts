import Phaser from "phaser";
import {defaultFontConfig} from "../core/common";

export default class EndScene extends Phaser.Scene {
	private playerScore!: number;

	constructor() {
		super({ key: "end" });
	}

	init(data: { playerScore: number }) {
		this.playerScore = data.playerScore;
	}

	preload(): void {

	}

	create(): void {
		this.add.text(
			this.scale.width / 2,
			this.scale.height / 2,
			`Total Score ${this.playerScore}`,
			defaultFontConfig("108px")
		).setOrigin(0.5, 0.5);
	}

	update(): void {
		
	}
} 
