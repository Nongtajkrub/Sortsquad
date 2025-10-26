import { randomWeighted } from "./common";
import config from "../../public/config.json";

const TRASH_CATEGORIES = ["Organic", "General", "Recyclable", "Hazardous"] as const;
export type TrashCategory = typeof TRASH_CATEGORIES[number];

const ORGANIC_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "Organic");
const GENERAL_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "General");
const RECYCLABLE_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "Recyclable");
const HAZARDOUS_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "Hazardous");

export class TrashCategoryOrderManager {
	private order: Array<TrashCategory>;
	private currentCategoryIndex: number = -1;

	constructor(order: Array<TrashCategory>) {
		this.order = order;
	}

	getNewCategory(): TrashCategory {
		this.currentCategoryIndex =
			(this.currentCategoryIndex + 1) % this.order.length;

		return this.order[this.currentCategoryIndex];
	}

	getCurrentCategory(): TrashCategory {
		return this.order[this.currentCategoryIndex];
	}
}

function categoryToIndex(category: TrashCategory) {
	switch (category) {
		case "Organic":
			return ORGANIC_TRASH_CATEGORY_INDEX;
		case "General":
			return GENERAL_TRASH_CATEGORY_INDEX;
		case "Recyclable":
			return RECYCLABLE_TRASH_CATEGORY_INDEX;
		case "Hazardous":
			return HAZARDOUS_TRASH_CATEGORY_INDEX;
	}
}

export function trashCategoryRandom(): TrashCategory {
	return TRASH_CATEGORIES[Math.floor(Math.random() * TRASH_CATEGORIES.length)];
}

// Trash category that are the same as the playerBinCategory are more likely to get pick.
export function trashCategoryRandomBias(playerBinCategory: TrashCategory): TrashCategory {
	return TRASH_CATEGORIES[
		randomWeighted(
			TRASH_CATEGORIES.length,
			categoryToIndex(playerBinCategory),
			config.game.playerTrashCategoryBias
		)
	];
}
