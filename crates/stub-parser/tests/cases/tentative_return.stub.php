<?php

/**
 * @generate-class-entries
 */

class Demo {
	/** @tentative-return-type */
	public function test1(): array {}

	/** @tentative-return-type */
	public function test2(): ?SomeObject {}

	/** @tentative-return-type */
	public function test3(): int|string {}
	
}
