import Phaser from "phaser";

type PlayerState = "Idle" | "Prerun" | "Running";

export interface PlayerConfig {
	x?: number,
	y?: number,
	scale?: number,
	idleKey: string,
	prerunKey: string,
	runningKey: string,
};

export class Player extends Phaser.Physics.Arcade.Sprite {
	private currentState: PlayerState = "Idle";
	private oldVelocityX: number = 0;

	constructor(scene: Phaser.Scene, config: PlayerConfig) {
		super(scene, config.x ?? 0, config.y ?? 0, config.idleKey);
		scene.add.existing(this);
		scene.physics.add.existing(this);

		this.setScale(config.scale ?? 1);

		this.createAnimations(config);
		this.anims.play("idle");
	}

	private createAnimations(config: PlayerConfig): void {
		this.anims.create({
			key: "idle",
			frames: this.anims.generateFrameNumbers(config.idleKey, { start: 0, end: 8 }),
			frameRate: 5,
			repeat: -1,
		});

		this.anims.create({
			key: "prerun",
			frames: this.anims.generateFrameNumbers(config.prerunKey, { start: 0, end: 4 }),
			frameRate: 10,
			repeat: 0,
		});

		this.anims.create({
			key: "running",
			frames: this.anims.generateFrameNumbers(config.runningKey, { start: 0, end: 8 }),
			frameRate: 5,
			repeat: -1,
		});
	}

	private updateMovement(cursor: Phaser.Types.Input.Keyboard.CursorKeys): void {
		this.oldVelocityX = this.body!.velocity.x;

		if (cursor.left.isDown) {
			this.setVelocityX(-500);
			this.setFlipX(true);
		} else if (cursor.right.isDown) {
			this.setVelocityX(500);
			this.setFlipX(false);
		} else {
			this.setVelocityX(0);
		}
	}

	private updateAnimation(): void {
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

	update(cursor: Phaser.Types.Input.Keyboard.CursorKeys): void {
		this.updateMovement(cursor);
		this.updateAnimation();
	}
}
