<?php

/**
 * @generate-class-entries
 * @undocumentable
 */

function rd_test1(): void {}

function rd_test2(string $str = ""): string {}

function rd_test3(string $str): string {}

function zpp_testing(
    bool $b,
    ?bool $bn,
    int $l,
    ?int $ln,
    float $f,
    ?float $fn,
    string $s1,
    ?string $s1n
): string {}

function zpp_testing_macro(
    bool $b,
    ?bool $bn,
    int $l,
    ?int $ln,
    float $f,
    ?float $fn,
    string $s1,
    ?string $s1n
): string {}

function dump_named_variadic_args(int $initial, mixed ...$args): void {}

function display_value($v): void {}

class RSDemo {
    private int $inner;

    public function __construct(
        int $initial
    ) {}

    public function increment(): void {}

    public function asString(): string {}
}
