import Phaser from "phaser";

export function randomPosX(y?: number): [number, number] {
	return [Math.round(Math.random() * window.innerWidth), y ?? 0];
}

export function defaultFontConfig(modifyConfig: {
	size?: string,
	color?: string,
	stroke?: number
}): Phaser.Types.GameObjects.Text.TextStyle {
	return {
		fontFamily: "pixelArt",
		fontSize: modifyConfig.size ?? "24px",
		stroke: "#000000",
		color: modifyConfig.color ?? "#FFFFFF",
		align: "center",
		strokeThickness: modifyConfig.stroke ?? 6,
	};
}

export function randomWeighted(max: number, prefer: number, bias?: number): number {
	bias = bias ?? 3;
	const totalWeight = (max - 1) * 1 + bias;
	const random = Math.random() * totalWeight;

	let cumulative = 0;
	for (let i = 0; i < max; i++) {
		cumulative += ((i === prefer) ? bias : 1);
		if (random < cumulative) {
			return i;
		}
	}

	return max - 1;
}
