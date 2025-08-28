import Phaser from "phaser";

import { trashTypeRandom, trashTypeToId } from "../core/trash-types";
import { trashCategoryRandom, type TrashCategory } from "../core/trash-categories";
import { randomPosX } from "../core/common";

export class Trash extends Phaser.Physics.Arcade.Sprite {
	private category!: TrashCategory;
	private alive: boolean = true;
		
	constructor(scene: Phaser.Scene) {
		const [x, y] = randomPosX(-50);
		const category = trashCategoryRandom();
		super(scene, x, y, trashTypeToId(trashTypeRandom(category)));

		scene.add.existing(this);
		scene.physics.add.existing(this);

		this.category = category;

		this.setVelocityY(200);
		this.setScale(3);
	}

	getCategory(): TrashCategory {
		return this.category;
	}

	setAlive(value: boolean) {
		this.alive = value;
	}

	isOutOfBound(): boolean {
		return this.body!.position.y >= window.innerHeight;
	}

	isAlive(): boolean {
		return this.alive;
	}

	pos(): [number, number] {
		return [this.body!.position.x, this.body!.position.y];
	}
}
