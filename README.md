# hyprtool

A Hyprland utility cli tool intended primarily for personal use. Currently only toggles windows between regular workspaces and special (scratchpad) workspaces.

## Installation

### As a Nix Flake

```nix
{
  inputs.hyprtool.url = "github:josh-romer/hyprtool";
  inputs.hyprtool.inputs.follows = "nixpkgs";
}
```

Add to your packages:
```nix
environment.systemPackages = [ hyprtool.packages.${system}.default ];
```

### Manual Build

```bash
cargo build --release
```

## Usage

```bash
# Run directly with nix
nix run github:josh-romer/hyprtool

# Or if installed
hyprtool

# Specify a different special workspace
hyprtool scratchpad
```

## License

MIT
