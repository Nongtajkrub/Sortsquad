export function randomPosX(y?: number): [number, number] {
	return [Math.round(Math.random() * window.innerWidth), y ?? 0];
}
