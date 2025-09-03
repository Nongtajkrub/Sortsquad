import Phaser from "phaser";

export default class Grass extends Phaser.GameObjects.Image {
	constructor(scene: Phaser.Scene) {
		super(scene, scene.scale.width / 2, 0, "grass");

		this.setOrigin(0.5, 1);
		this.setDisplaySize(scene.scale.width, scene.scale.height / 3);
		this.setY(scene.scale.height);
		scene.add.existing(this);
	}
}
