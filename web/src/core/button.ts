import Phaser from "phaser";

interface ButtonConfig {
	x?: number,
	y?: number,
	texture: string,
	scale?: number,
	ontop?: boolean,
};

export class Button extends Phaser.GameObjects.Image {
	private down: boolean = false;

	constructor(scene: Phaser.Scene, config: ButtonConfig) {
		super(scene, config.x ?? 0, config.y ?? 0, config.texture);

		scene.add.existing(this);

		this.setOrigin(0.5, 0.5);
		this.setInteractive({ useHandCursor: true });
		this.setScale(config.scale ?? 1);

		if (config.ontop ?? true) {
			this.setDepth(999);
		}

		this.on("pointerdown", () => this.down = true);
		this.on("pointerup", () => this.down = false);
		this.on("pointerout", () => this.down = false);
	}

	isDown(): boolean {
		return this.down;
	}
}
