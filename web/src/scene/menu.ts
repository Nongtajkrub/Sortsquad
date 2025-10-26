import Phaser from "phaser";
import Background from "../core/background";
import {type TrashCategory, TrashCategoryOrderManager} from "../core/trash-categories";
import config from "../../public/config.json"

const trashCategoryOrderManager = new TrashCategoryOrderManager([
	"Organic",
	"General",
	"Recyclable",
	"Hazardous",
]);

export default class MenuScene extends Phaser.Scene {
	private binCategory!: TrashCategory;

	constructor() {
		super({ key: "menu" });
	}

	init() {
		this.binCategory = trashCategoryOrderManager.getNewCategory();
		// reload background for new category
		if (this.textures.exists("background")) this.textures.remove("background");
	}

	private categoryToBackgroundPath(): string {
		switch (this.binCategory) {
			case "Organic":
				return config.path.menu.startOrganic;
			case "General":
				return config.path.menu.startGeneral;
			case "Recyclable":
				return config.path.menu.startRecycle;
			case "Hazardous":
				return config.path.menu.startHazardous;
		}
	}

	preload(): void {
		this.load.image("background", this.categoryToBackgroundPath());
		this.load.image("loadingScreen", config.path.menu.loadingScreen);
	}

	create(): void {
		new Background(this, "background");

		this.input.on(Phaser.Input.Events.POINTER_DOWN, () => {
			this.scene.start("game", { binCategory: this.binCategory });
		});
	}
} 
