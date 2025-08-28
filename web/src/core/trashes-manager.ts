import { Trash } from "../sprites/trash";

export default class TrashesManager {
	private trahse: Array<Trash> = [];
	private scene!: Phaser.Scene;

	constructor(scene: Phaser.Scene) {
		this.scene = scene;
	}	

	spawn(): void {
		this.trahse.push(new Trash(this.scene));
	}

	getTrashes(): Array<Trash> {
		return this.trahse;
	}

	update(): void {
		this.trahse = this.trahse.filter((trash: Trash) => {
			if (trash.isOutOfBound() || !trash.isAlive()) {
				trash.destroy();
				return false;
			}
			return true;
		});
	}
}
