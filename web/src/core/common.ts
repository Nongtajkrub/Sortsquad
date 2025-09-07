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
