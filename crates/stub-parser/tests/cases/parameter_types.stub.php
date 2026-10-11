<?php

/**
 * @generate-class-entries
 */

/** @param mixed $param */
function noType($param): void {}

/** @param mixed $param */
function noTypeDefault($param = "test"): void {}

function unionMask(int|string|bool $param): void {}

function classUnion(stdClass|Stringable $param): void {}

function classMaskUnion(stdClass|int|bool $param): void {}
