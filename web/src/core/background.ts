import Phaser from "phaser";

export default class Background extends Phaser.GameObjects.Image {
	constructor(scene: Phaser.Scene, texture: string, stretch?: boolean) {
		super(scene, 0, 0, texture);
		this.setOrigin(0, 0);
		
		if (stretch ?? false) {
			this.setDisplaySize(scene.scale.width, scene.scale.height);
		} else {
			this.setScale(this.computeMaxScaleNoStretch(scene));
			this.setOrigin(0.5, 0.5);
			this.setPosition(scene.scale.width / 2, scene.scale.height / 2);
		}

		scene.add.existing(this);
	}

	private computeMaxScaleNoStretch(scene: Phaser.Scene): number {
		return Math.min(scene.scale.width / this.width, scene.scale.height / this.height);
	}
}
