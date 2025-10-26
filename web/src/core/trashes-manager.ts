import { Trash } from "../sprites/trash";
import type { TrashCategory } from "./trash-categories";

export default class TrashesManager {
	private trahse: Array<Trash> = [];
	private scene!: Phaser.Scene;
	private playerBinCategory!: TrashCategory;
	private spawmIntervalId!: ReturnType<typeof setInterval>;

	private spawn(): void {
		this.trahse.push(new Trash(this.scene, this.playerBinCategory));
	}

	constructor(
		scene: Phaser.Scene,
		playerBinCategory: TrashCategory,
		interval: number
	) {
		this.scene = scene;
		this.playerBinCategory = playerBinCategory;

		this.spawmIntervalId = setInterval(() => {
			this.spawn();
		}, interval);
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

	getTrashes(): Array<Trash> {
		return this.trahse;
	}

	clear(): void {
		this.trahse.forEach((trash: Trash) => {
			trash.remove();
		});
	}

	destroy(): void {
		this.clear();
		clearInterval(this.spawmIntervalId);
	}
}
