{ pkgs }: {
	deps = [
		pkgs.htop
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}