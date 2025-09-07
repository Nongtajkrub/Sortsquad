import Phaser from "phaser";

export function randomPosX(y?: number): [number, number] {
	return [Math.round(Math.random() * window.innerWidth), y ?? 0];
}

export function defaultFontConfig(
	size?: string,
	color?: string,
	stroke?: number,
): Phaser.Types.GameObjects.Text.TextStyle {
	return {
		fontFamily: "pixelArt",
		fontSize: size ?? "24px",
		stroke: "#000000",
		color: color ?? "#FFFFFF",
		align: "center",
		strokeThickness: stroke ?? 6,
	};
}
