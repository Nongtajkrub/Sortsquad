const TRASH_CATEGORIES = ["Organic", "General"] as const;
export type TrashCategory = typeof TRASH_CATEGORIES[number];

export function trashCategoryRandom(): TrashCategory {
	return TRASH_CATEGORIES[Math.floor(Math.random() * TRASH_CATEGORIES.length)];
}
