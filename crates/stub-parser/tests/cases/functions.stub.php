<?php

/**
 * @generate-class-entries
 */

function test1(): void {}

function test2(string $str = ""): string {}

/** @return mixed */
function test3() {}

/**
 * @param mixed $v
 * @return mixed
 */
function test4($v = 123) {}

/**
 * @param mixed $v
 * @return mixed
 */
function test5($v) {}

function test6(int $v = 123): int {}
