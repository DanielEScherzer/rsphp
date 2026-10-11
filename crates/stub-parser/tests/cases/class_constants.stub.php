<?php

/**
 * @generate-class-entries
 */

final class CustomCastable {
	public const int TARGET_BOOL = 1;
	public const int TARGET_FLOAT = 2;
	public const int TARGET_LONG = 4;
	public const int TARGET_ALL = 7;

	private int $target;
	public function __construct(int $target = CustomCastable::TARGET_ALL) {}
}

class ConstantValues {
	/** @cvalue ZEND_DEBUG */
	public const float IS_5 = UNKNOWN;
	/** @cvalue PHP_OS */
	public const string IS_STR = UNKNOWN;
}

class ImplicitPublicConstant {
	const int FOO = 5;
}
