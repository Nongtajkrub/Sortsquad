import type { TrashCategory } from "./trash-categories";

const ORGANIC_TRASH_TYPES = ["Apple", "Vegatable"] as const;
type OrganicTrashType = typeof ORGANIC_TRASH_TYPES[number];

const GENERAL_TRASH_TYPES = ["Shoe"] as const;
type GeneralTrashType = typeof GENERAL_TRASH_TYPES[number];

export type AnyTrashTypes = OrganicTrashType | GeneralTrashType;

export function trashTypeRandom(trashCategory: TrashCategory): AnyTrashTypes {
	switch (trashCategory) {
		case "Organic":
			return ORGANIC_TRASH_TYPES[Math.floor(Math.random() * ORGANIC_TRASH_TYPES.length)];
		case "General":
			return GENERAL_TRASH_TYPES[Math.floor(Math.random() * GENERAL_TRASH_TYPES.length)]; 
	}
}

export function trashTypeToId(trashType: AnyTrashTypes): string {
	return trashType.toLowerCase();
}
