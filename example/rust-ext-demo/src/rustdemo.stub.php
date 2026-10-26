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

class PropertiesDemo {
	public mixed $is_public;
	protected mixed $is_protected;
	private mixed $is_private;

	public readonly mixed $pub_readonly;
	protected readonly mixed $prot_readonly;
	private readonly mixed $priv_readonly;

	public public(set) readonly mixed $pub_pub_readonly;
	public protected(set) readonly mixed $pub_prot_readonly;
	public private(set) readonly mixed $pub_priv_readonly;

	protected protected(set) readonly mixed $prot_prot_readonly;
	protected private(set) readonly mixed $prot_priv_readonly;

	private private(set) readonly mixed $priv_priv_readonly;

	public static mixed $is_static;
	public final mixed $is_final;
}
