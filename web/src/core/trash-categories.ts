import { randomWeighted } from "./common";

const TRASH_CATEGORIES = ["Organic", "General", "Recyclable", "Hazardous"] as const;
export type TrashCategory = typeof TRASH_CATEGORIES[number];

const ORGANIC_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "Organic");
const GENERAL_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "General");
const RECYCLABLE_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "Recyclable");
const HAZARDOUS_TRASH_CATEGORY_INDEX = TRASH_CATEGORIES.findIndex((category) => category == "Hazardous");

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

// Trash category that are the same as the playerBinCategory are more likely to get pick.
export function trashCategoryRandom(playerBinCategory: TrashCategory): TrashCategory {
	return TRASH_CATEGORIES[
		randomWeighted(TRASH_CATEGORIES.length, categoryToIndex(playerBinCategory))
	];
}
