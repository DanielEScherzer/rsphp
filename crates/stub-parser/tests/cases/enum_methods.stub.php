<?php

enum Color {
	case White;
	case Black;
	case Red;
	case Green;
	case Blue;

	public function isGreyScale(): bool {}

}
