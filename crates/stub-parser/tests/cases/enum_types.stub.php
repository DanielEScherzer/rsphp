<?php

/**
 * @generate-class-entries
 */

enum Unbacked {
	case Foo;
	case Bar;
}

enum IntBacked: int {
	case Foo = 1;
	case Bar = 2;
}

enum StringBacked: string {
	case Foo = "Foo";
	case Bar = "Bar";
}
