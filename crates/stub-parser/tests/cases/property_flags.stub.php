<?php

/**
 * @generate-class-entries
 */

class Demo {
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

readonly class ReadonlyClass {
	public mixed $is_public;
}
