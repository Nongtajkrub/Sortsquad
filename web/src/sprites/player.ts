import Phaser from "phaser";
import type { Trash } from "./trash";
import type TrashesManager from "../core/trashes-manager";
import { Button } from "../core/button";
import { type TrashCategory } from "../core/trash-categories";
import { defaultFontConfig } from "../core/common";

type PlayerState = "Idle" | "Prerun" | "Running";

export interface PlayerConfig {
	x?: number,
	y?: number,
	scale?: number,
	binCategory: TrashCategory
};

export class Player extends Phaser.Physics.Arcade.Sprite {
	private currentState: PlayerState = "Idle";
	private oldVelocityX: number = 0;

	private binCategory!: TrashCategory;

	private score: number = 0;
	private scoreText!: Phaser.GameObjects.Text;

	private leftArrowButton!: Button;
	private rightArrowButton!: Button;

	private scoredAudio!: Phaser.Sound.BaseSound; 

	constructor(scene: Phaser.Scene, config: PlayerConfig) {
		super(scene, config.x ?? 0, config.y ?? 0, "");
		scene.add.existing(this);
		scene.physics.add.existing(this);
		this.setScale(config.scale ?? 1);
		this.body!.setSize(this.scale * 6.25 , this.scale * 2);
		this.setOrigin(0.5, 0.5);

		this.createGui(config);

		this.binCategory = config.binCategory;
		this.createAnimations();
		this.anims.play("idle");

		this.scoredAudio = scene.sound.add("scoredAudio");
	}

	private createGui(config: PlayerConfig): void {
		this.scoreText = this.scene.add.text(
			config.x ?? 0,
			(config.y ?? 0) - 100,
			`Score: ${this.score}`,
			defaultFontConfig("32px"),
		).setOrigin(0.5, 0.5);

		this.leftArrowButton = new Button(this.scene, {
			x: 100,
			y: window.innerHeight - 100,
			texture: "leftArrowButton",
			scale: 3
		});

		this.rightArrowButton = new Button(this.scene, {
			x: innerWidth - 100,
			y: window.innerHeight - 100,
			texture: "rightArrowButton",
			scale: 3
		});
	}

	private createAnimations(): void {
		this.anims.create({
			key: "idle",
			frames: this.anims.generateFrameNumbers("binIdle", { start: 0, end: 8 }),
			frameRate: 5,
			repeat: -1,
		});

		this.anims.create({
			key: "prerun",
			frames: this.anims.generateFrameNumbers("binPrerun", { start: 0, end: 4 }),
			frameRate: 13,
			repeat: 0,
		});

		this.anims.create({
			key: "running",
			frames: this.anims.generateFrameNumbers("binRunning", { start: 0, end: 8 }),
			frameRate: 5,
			repeat: -1,
		});
	}

	private updateMovement(cursor?: Phaser.Types.Input.Keyboard.CursorKeys): void {
		this.oldVelocityX = this.body!.velocity.x;

		if (this.leftArrowButton.isDown() || cursor?.left.isDown) {
			this.setVelocityX(-500);
			this.setFlipX(true);
		} else if (this.rightArrowButton.isDown() || cursor?.right.isDown) {
			this.setVelocityX(500);
			this.setFlipX(false);
		} else {
			this.setVelocityX(0);
		}
	}

	private updateMovementAnimation(): void {
		const recentlyMove: boolean = this.body!.velocity.x != this.oldVelocityX;

		if (recentlyMove) {
			this.anims.play("prerun");
			this.currentState = "Prerun";

			this.once("animationcomplete", () => {
				this.anims.play("running");
				this.currentState = "Running";
			});
		} else if (this.body!.velocity.x === 0 && this.currentState !== "Idle") {
			this.anims.play("idle");
			this.currentState = "Idle";
		}
	}

	private updateHitbox(): void {
		if (!this.flipX) {
			this.body!.setOffset(this.scale * 4.5, 20);
		} else {
			this.body!.setOffset(this.scale * 0.25, 20);
		}
	}

	private spawnScoredAnimation(isScored: boolean): void {
		const x = this.body!.x;
		const y = this.body!.y;

		const scoredText = this.scene.add.text(
			x,
			y,
			(isScored) ? "+1" : "X",
			defaultFontConfig("32px", (isScored) ? "#a3e635" : "#ef4444")
		).setOrigin(0.5, 0.5);

		this.scene.tweens.add({
			targets: scoredText,
			y: y - 50,
			scale: { from: 1, to: 1.5 },
			ease: "Cubic.easeOut",
			duration: 800,
		})

		this.scene.tweens.add({
			targets: scoredText,
			alpha: 0,
			ease: "Linear",
			duration: 1200,
			onComplete: () => {
				scoredText.destroy();
			}
		})
	}

	private updateCollision(trashes: Array<Trash>): void {
		trashes.forEach((trash: Trash) => {
			if (this.scene.physics.overlap(trash, this)) {
				const isScored = trash.getCategory() == this.binCategory;

				this.score += (isScored) ? 1 : -1;
				this.score = Math.max(this.score, 0);
				this.spawnScoredAnimation(isScored);

				if (isScored) {
					this.scoredAudio.play({ volume: 0.5 });
				}

				trash.setAlive(false);
			}
		});
	}

	private updateGraphic() {
		this.scoreText.setText(`Score: ${this.score}`);
		this.scoreText.setX(this.body?.center.x);
	}

	update(trashManager: TrashesManager, cursor?: Phaser.Types.Input.Keyboard.CursorKeys): void {
		this.updateMovement(cursor);
		this.updateHitbox();
		this.updateMovementAnimation();
		this.updateCollision(trashManager.getTrashes());
		this.updateGraphic();
	}

	getScore(): number {
		return this.score;
	}
}
