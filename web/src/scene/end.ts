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
			(this.scale.height / 2) - 60,
			`Total Score`,
			defaultFontConfig({ size: "104px", color: "#ffe700", stroke: 10 })
		).setOrigin(0.5, 0.5);

		this.add.text(
			this.scale.width / 2,
			(this.scale.height / 2) + 60,
			this.playerScore.toString(),
			defaultFontConfig({ size: "104px", stroke: 10 })
		).setOrigin(0.5, 0.5);

		this.input.on(Phaser.Input.Events.POINTER_DOWN, () => {
			this.scene.start("menu");
		})
	}

	update(): void {
		
	}
} 
