import type { TrashCategory } from "./trash-categories";

const ORGANIC_TRASH_TYPES = ["Apple", "Vegatable", "Fishbone"] as const;
type OrganicTrashType = typeof ORGANIC_TRASH_TYPES[number];

const GENERAL_TRASH_TYPES = ["PlasticBag", "Tissue", "Snack"] as const;
type GeneralTrashType = typeof GENERAL_TRASH_TYPES[number];

const RECYCLABLE_TRASH_TYPE = ["WaterBottle", "Coke", "Newspaper"] as const;
type RecyclableTrashType = typeof RECYCLABLE_TRASH_TYPE[number];

const HAZARDOUS_TRASH_TYPE = ["Electronic", "Battery", "Bleach"] as const;
type HazardousTrashType = typeof HAZARDOUS_TRASH_TYPE[number];

export type AnyTrashTypes =
	OrganicTrashType | GeneralTrashType | RecyclableTrashType | HazardousTrashType;

export function trashTypeRandom(trashCategory: TrashCategory): AnyTrashTypes {
	switch (trashCategory) {
		case "Organic":
			return ORGANIC_TRASH_TYPES[Math.floor(Math.random() * ORGANIC_TRASH_TYPES.length)];
		case "General":
			return GENERAL_TRASH_TYPES[Math.floor(Math.random() * GENERAL_TRASH_TYPES.length)]; 
		case "Recyclable":
			return RECYCLABLE_TRASH_TYPE[Math.floor(Math.random() * RECYCLABLE_TRASH_TYPE.length)];
		case "Hazardous":
			return HAZARDOUS_TRASH_TYPE[Math.floor(Math.random() * HAZARDOUS_TRASH_TYPE.length)];
	}
}

export function trashTypeToId(trashType: AnyTrashTypes): string {
	return trashType.toLowerCase();
}
