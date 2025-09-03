import { Trash } from "../sprites/trash";
import type { TrashCategory } from "./trash-categories";

export default class TrashesManager {
	private trahse: Array<Trash> = [];
	private scene!: Phaser.Scene;
	private playerBinCategory!: TrashCategory;

	constructor(scene: Phaser.Scene, playerBinCategory: TrashCategory) {
		this.scene = scene;
		this.playerBinCategory = playerBinCategory;
	}	

	spawn(): void {
		this.trahse.push(new Trash(this.scene, this.playerBinCategory));
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
}
