<?php

/**
 * @generate-class-entries
 */

class Demo {
	public function test1(): void {}

	public function test2(string $str = ""): string {}

	/** @return mixed */
	public function test3() {}
	
	/**
	 * @param mixed $v
	 * @return mixed
	 */
	public function test4($v = 123) {}

	/**
	 * @param mixed $v
	 * @return mixed
	 */
	public function test5($v) {}

	public function test6(int $v = 123): int {}
}
