import { Trash } from "../sprites/trash";
import type { TrashCategory } from "./trash-categories";

export default class TrashesManager {
	private trahse: Array<Trash> = [];
	private scene!: Phaser.Scene;
	private playerBinCategory!: TrashCategory;
	private spawnable: boolean = true;

	constructor(scene: Phaser.Scene, playerBinCategory: TrashCategory) {
		this.scene = scene;
		this.playerBinCategory = playerBinCategory;
	}	

	spawn(): void {
		if (this.spawnable) {
			this.trahse.push(new Trash(this.scene, this.playerBinCategory));
		}
	}

	getTrashes(): Array<Trash> {
		return this.trahse;
	}

	update(): void {
		this.trahse = this.trahse.filter((trash: Trash) => {
			trash.update();

			if (!trash.isAlive()) {
				trash.remove();
				return false;
			}
			return true;
		});
	}

	setSpawnable(value: boolean): void {
		this.spawnable = value;
	}

	clear(): void {
		this.trahse.forEach((trash: Trash) => {
			trash.remove();
		});
	}
}
