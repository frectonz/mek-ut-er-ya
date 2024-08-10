{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "mekuteriya";
  version = "0.1.5";

  src = fetchFromGitHub {
    owner = "frectonz";
    repo = "mek-ut-er-ya";
    rev = version;
    hash = "sha256-bWp2UNrhCHY2DQWusGS9L9/jI2r23F34yLpuE6nuOD0=";
  };

  cargoHash = "sha256-WkrlQbNTP3lNEtlnAMrTd9lBo2Q4dECqtV29kmV8F7A=";

  meta = {
    description = "A simple program for handling Ethiopian calendar dates.";
    homepage = "https://github.com/frectonz/mek-ut-er-ya";
    license = lib.licenses.unlicense;
    maintainers = [ ];
  };
}
