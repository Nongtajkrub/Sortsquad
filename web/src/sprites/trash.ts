import Phaser from "phaser";

import { trashTypeRandom, trashTypeToId } from "../core/trash-types";
import { trashCategoryRandom, trashCategoryRandomBias, type TrashCategory } from "../core/trash-categories";
import { randomPosX } from "../core/common";

export class Trash extends Phaser.Physics.Arcade.Sprite {
	private category!: TrashCategory;
	private alive: boolean = true;
	private greenCircle?: Phaser.GameObjects.Image;	

	constructor(scene: Phaser.Scene, playerBinCategory: TrashCategory) {
		const [x, y] = randomPosX(-50);
		const category = trashCategoryRandomBias(playerBinCategory);
		const scale = 3;
		super(scene, x, y, trashTypeToId(trashTypeRandom(category)));

		scene.add.existing(this);
		scene.physics.add.existing(this);

		if (category == playerBinCategory) {
			this.greenCircle = scene.add.image(x, y, "correctCategoryCircle")
				.setOrigin(0.5, 0.5);

			this.greenCircle.setScale(scale + 0.5);
		} else {
			this.body!.setSize(scale);
		}

		this.category = category;
		this.setVelocityY(200);
		this.setScale(scale);
	}

	getCategory(): TrashCategory {
		return this.category;
	}

	setAlive(value: boolean) {
		this.alive = value;
	}

	isAlive(): boolean {
		return this.alive;
	}

	pos(): [number, number] {
		return [this.body!.position.x, this.body!.position.y];
	}

	update(): void {
		this.greenCircle?.setY(this.body!.center.y);

		if (this.body!.position.y >= window.innerHeight) {
			this.alive = false;
		} 
	}

	remove(): void {
		this.destroy();
		this.greenCircle?.destroy();
	}
}
