{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "mekuteriya";
  version = "0.1.4";

  src = fetchFromGitHub {
    owner = "frectonz";
    repo = "mek-ut-er-ya";
    rev = version;
    hash = "sha256-ar69qGjvq+jur+WfomVau3UM7Fna7UGzzBJMx9h8+pI=";
  };

  cargoHash = "sha256-LyCMZpcuynXQaXpuC7iI3b65dXepoBDNAtcfxmCqEVk=";

  meta = {
    description = "A simple program for handling Ethiopian calendar dates.";
    homepage = "https://github.com/frectonz/mek-ut-er-ya";
    license = lib.licenses.unlicense;
    maintainers = [ ];
  };
}
